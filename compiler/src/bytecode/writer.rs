/*
 * Copyright (c) 2024 The YuvaKriti Lang Authors.
 *
 * This program is free software: you can redistribute it and/or modify it under the
 *  terms of the GNU General Public License as published by the Free Software
 *  Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this
 * program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::ops::Deref;

use crate::ast::{AssignExpr, BreakStmt};
use crate::ast::ASTVisitor;
use crate::ast::BinaryExpr;
use crate::ast::BinaryOp;
use crate::ast::BlockStmt;
use crate::ast::ClassDecl;
use crate::ast::ContinueStmt;
use crate::ast::ForStmt;
use crate::ast::FuncDecl;
use crate::ast::IdentifierExpr;
use crate::ast::IdentifierType;
use crate::ast::IfStmt;
use crate::ast::LiteralExpr;
use crate::ast::PrintStmt;
use crate::ast::Program;
use crate::ast::VarStmt;
use crate::ast::Visitable;
use crate::ast::WhileStmt;
use crate::bytecode::{attrs, decls};
use crate::bytecode::attrs::Attr;
use crate::bytecode::attrs::Code;
use crate::bytecode::attrs::CodeSize;
use crate::bytecode::bytes::AssertingByteConversions;
use crate::bytecode::cp::ConstantEntry;
use crate::bytecode::cp_info::NumberInfo;
use crate::bytecode::cp_info::Utf8Info;
use crate::bytecode::file::YKBFile;
use crate::bytecode::opcode::{get_opcode, opcode_cmp};
use crate::bytecode::opcode::OpCode;
use crate::bytecode::opcode::opcode_cmpz;
use crate::bytecode::opcode::OpCodeExt;
use crate::features::CompilerFeatures;
use crate::messages;
use crate::scope::Scope;
use crate::symtab::VarSym;

/// Converts a program into a YKB file.
pub struct YKBFileWriter<'inst> {
    file: &'inst mut YKBFile,
    features: &'inst CompilerFeatures,
}

impl YKBFileWriter<'_> {
    pub fn new<'a>(file: &'a mut YKBFile, features: &'a CompilerFeatures) -> YKBFileWriter<'a> {
        return YKBFileWriter { file, features };
    }

    pub fn file(&self) -> &YKBFile {
        return &self.file;
    }

    pub fn file_mut(&mut self) -> &mut YKBFile {
        return &mut self.file;
    }

    pub fn write(&mut self, program: &mut Program) {
        let mut codegen = CodeGen::new(&mut self.file, &self.features);
        let mut loops = vec![];
        let mut context = CodeGenContext::new(&mut loops);
        program.accept(&mut codegen, &mut context);
    }
}

struct CodeGen<'a> {
    file: &'a mut YKBFile,
    #[allow(unused)]
    features: &'a CompilerFeatures,
    stack_count: i16,
    max_stack: u16,
    local_count: i16,
    max_locals: u16,
    cp: CodeSize,
    instructions: Vec<u8>,
}

struct CodeGenContext<'a> {
    
    /// The scope in which [CodeGen] is visiting the AST.
    pub scope: Scope<'a>,

    /// A stack of [LoopContext]s, which keep track of information about the loops [CodeGen]
    /// is currently visiting.
    pub loops: &'a mut Vec<LoopContext>,
}

#[derive(Debug, PartialEq)]
struct PendingJump {

    /// The type of jump.
    pub typ: JumpType,
    
    /// The index of the instruction where jump occurs.
    pub from: CodeSize,
}

#[derive(Debug, PartialEq, Clone)]
enum JumpType {
    Continue,
    Break
}

impl PendingJump {
    
    fn new(typ: JumpType, from: CodeSize) -> PendingJump {
        return PendingJump {
            typ,
            from
        }
    }
    
    pub fn _continue(from: CodeSize) -> PendingJump {
        return PendingJump::new(JumpType::Continue, from)
    }

    pub fn _break(from: CodeSize) -> PendingJump {
        return PendingJump::new(JumpType::Break, from)
    }
}

#[derive(Debug, PartialEq)]
struct LoopContext {
    
    /// The index of the instruction which begins the loop.
    pub pc: CodeSize,

    /// The type of loop.
    pub typ: LoopType,

    /// An optional label for the loop.
    pub label: Option<String>,
    
    /// A list of [PendingJump]s, which contain information about jump instructions which must be
    /// patched after the bytecode for a loop has been written. 
    pub pending_jumps: Vec<PendingJump>,
}

#[derive(Debug, PartialEq, Clone)]
enum LoopType {
    For,
    While
}

impl LoopContext<> {

    fn new(pc: CodeSize, typ: LoopType, label: Option<String>) -> LoopContext {
        return LoopContext {
            pc,
            typ,
            label,
            pending_jumps: Vec::with_capacity(0),
        };
    }

    fn matches_label(&mut self, label: Option<&IdentifierExpr>) -> bool {
        if label.is_none() {
            return true;
        }

        let _label = &label.unwrap().name;
        if self
            .label.as_ref()
            .map(|l| l == _label)
            .unwrap_or(false)
        {
            return true;
        }

        false
    }
}

impl CodeGenContext<'_> {
    fn new(loops: &mut Vec<LoopContext>) -> CodeGenContext {
        return CodeGenContext {
            scope: Scope::new(),
            loops,
        };
    }

    fn with_scope<'a>(scope: Scope<'a>, loops: &'a mut Vec<LoopContext>) -> CodeGenContext<'a> {
        let mut ctx = CodeGenContext::new(loops);
        ctx.scope = scope;
        return ctx;
    }

    fn push_loop(&mut self, pc: CodeSize, typ: LoopType, label: Option<String>) {
        self.loops.push(LoopContext::new(pc, typ, label))
    }

    fn pop_loop(&mut self) -> Option<LoopContext> {
        self.loops.pop()
    }
    
    fn find_loop(&mut self, label: Option<&IdentifierExpr>) -> Option<&mut LoopContext> {
        for _loop in self.loops.iter_mut().rev() {
            if _loop.matches_label(label) {
                return Some(_loop)
            }
        }
        
        None
    }
}

impl<'a> CodeGen<'a> {
    /// The number of bytes of instructions that can be written to the [Attr::Code] attribute.
    pub const MAX_INSN_SIZE: CodeSize = 0xFFFFFFFF;

    /// The maximum (overall) depth of the operand stack for [Code] attributes.
    pub const MAX_STACK_SIZE: u16 = 0xFFFF;

    fn new(file: &'a mut YKBFile, features: &'a CompilerFeatures) -> Self {
        return CodeGen {
            file,
            features,
            stack_count: 0,
            max_stack: 0,
            local_count: 0,
            max_locals: 0,
            cp: 0,
            instructions: Vec::with_capacity(65),
        };
    }

    fn cp(&self) -> CodeSize {
        return self.cp.clone();
    }

    pub fn update_max_locals(&mut self, locals_effect: i8) {
        self.local_count += locals_effect as i16;

        if self.local_count > self.max_locals as i16 {
            self.max_locals = self.local_count as u16;
        }
    }

    fn _update_max_stack(&mut self, op_code: OpCode) {
        self.update_max_stack(op_code.stack_effect());
    }

    fn update_max_stack(&mut self, stack_effect: i8) {
        self.stack_count += stack_effect as i16;
        if self.stack_count > self.max_stack as i16 {
            self.max_stack = self.stack_count as u16;
        }

        if self.max_stack >= Self::MAX_STACK_SIZE {
            panic!("Stack size too large!");
        }
    }

    fn check_size(&self, additional: CodeSize) {
        if self.instructions.len().as_code_size() + additional > Self::MAX_INSN_SIZE {
            panic!("Instruction size too large!");
        }
    }

    fn ensure_size_incr(&mut self, additional: CodeSize) {
        self.check_size(additional);
        self.instructions
            .resize(self.cp as usize + additional as usize, 0);
    }

    fn instructions(&self) -> &Vec<u8> {
        return &self.instructions;
    }

    fn get_u1(&self, index: CodeSize) -> u8 {
        return self.instructions[index as usize];
    }
    
    #[allow(unused)]
    fn get_u2(&self, index: CodeSize) -> u16 {
        return (self.instructions[index as usize].as_u16() << 8)
            | self.instructions[index as usize + 1].as_u16();
    }

    fn get_i2(&self, index: CodeSize) -> i16 {
        return ((self.instructions[index as usize] as i16) << 8)
            | self.instructions[index as usize + 1] as i16;
    }

    fn emitop(&mut self, opcode: OpCode) {
        self.emitop0(opcode);
    }

    fn emitop0(&mut self, opcode: OpCode) {
        self.ensure_size_incr(1);

        self.instructions[self.cp as usize] = opcode.as_op_size();
        self.cp += 1;

        self.update_max_stack(opcode.stack_effect());
    }

    fn emit1_16(&mut self, opcode: OpCode, operand: u16) {
        self.ensure_size_incr(3);
        self.instructions[self.cp as usize] = opcode.as_op_size();
        self.instructions[self.cp as usize + 1] = (operand >> 8).as_u8();
        self.instructions[self.cp as usize + 2] = operand.as_u8();
        self.cp += 3;

        self._update_max_stack(opcode);
    }

    fn patch2(&mut self, index: CodeSize, d1: u8, d2: u8) {
        self.instructions[index as usize] = d1;
        self.instructions[index as usize + 1] = d2;
    }

    fn patch_u16(&mut self, index: CodeSize, data: u16) {
        self.patch2(index, (data >> 8).as_u8(), data.as_u8());
    }

    fn jmptocp(&mut self, jmp_from: CodeSize) {
        self.patch_u16(jmp_from + 1, (self.cp() - jmp_from - 3).as_u16());
    }

    fn patch_jmp(&mut self, jmp_from: CodeSize, jmp_to: CodeSize) {
        let target = (jmp_to as i64 - jmp_from as i64 - 3) as i16;
        self.patch2(jmp_from + 1, (target >> 8) as u8, target as u8);
    }

    fn emitjmp(&mut self, opcode: OpCode) -> CodeSize {
        self.emit1_16(opcode, 0);
        return self.cp() - 3;
    }

    fn emitjmp1(&mut self, opcode: OpCode, target: CodeSize) -> CodeSize {
        let idx = self.emitjmp(opcode);
        self.patch_jmp(idx, target);
        idx
    }

    fn reset(&mut self) {
        self.stack_count = 0;
        self.max_stack = 0;
        self.local_count = 0;
        self.max_locals = 0;
        self.cp = 0;
        self.instructions = Vec::with_capacity(0);
    }

    fn handle_short_circuit(
        &mut self,
        binary: &mut BinaryExpr,
        op: OpCode,
        ctx: &mut CodeGenContext,
    ) {
        self.visit_expr(&mut binary.left, ctx);

        let jmpiffalsy = self.emitjmp(op);
        self.emitop0(OpCode::Pop);

        self.visit_expr(&mut binary.right, ctx);
        self.jmptocp(jmpiffalsy);
    }

    fn optimize(&mut self) {
        let mut idx = 0;
        while idx < self.cp() {
            let opcode = get_opcode(self.get_u1(idx));
            if opcode.is_jmp() {
                self.resolve_jmp(opcode, idx);
            }
        
            idx += 1 + opcode.operand_size() as CodeSize;
        }
    }

    fn resolve_jmp(&mut self, op: OpCode, idx: CodeSize) {
        let jmp_delta = self.get_i2(idx + 1);
        let target = idx.checked_add_signed(jmp_delta as i32 + 3).unwrap();

        // target address jumps out of execution
        if target >= self.cp() {
            return;
        }

        let t_op = get_opcode(self.get_u1(target));
        if !t_op.is_jmp() {
            return;
        }

        // if the instruction to which `op` jumps is also a jump instruction
        // then resolve that target jump first
        self.resolve_jmp(t_op, target);

        let t_delta = self.get_i2(target + 1);
        let t_addr = target.checked_add_signed(t_delta as i32 + 3).unwrap();

        // target's target address jumps out of execution
        if t_addr >= self.cp() {
            return;
        }

        // if both opcodes are same, or if the current jmp jumps to an unconditional jmp
        // then update this jmp to jump directly to where the target jmp jumps
        if op == t_op || t_op == OpCode::Jmp {
            self.patch_jmp(idx, t_addr);
        }
    }
    
    /// Patch pending jumps in loops. `continue_at` is the address of the instruction at which the
    /// program continues when a [JumpType::Continue] jump is encountered. `break_to` is the address of
    /// the instruction at which the program continues when a [JumpType::Break] jump is encountered.
    fn patch_pending(
        &mut self,
        pending: &Vec<PendingJump>,
        continue_at: CodeSize,
        break_to: CodeSize
    ) {
        for pending in pending {
            match pending.typ {
                JumpType::Continue => {
                    self.patch_jmp(pending.from, continue_at)
                }
                JumpType::Break => {
                    self.patch_jmp(pending.from, break_to)
                }
            }
        }
    }
}

impl ASTVisitor<CodeGenContext<'_>, ()> for CodeGen<'_> {
    fn visit_program(&mut self, program: &mut Program, ctx: &mut CodeGenContext) -> Option<()> {
        if self
            .file
            .attributes()
            .iter()
            .find(|attr| attr.name() == attrs::CODE)
            .is_some()
        {
            panic!("A YKBFile cannot have multiple Code attributes")
        }

        self.default_visit_program(program, ctx, true, false);
        for i in 0..program.stmts.len() {
            let stmt = program.stmts.get_mut(i).unwrap();
            self.visit_stmt(stmt, ctx);
        }

        if self.instructions().len() > 0 {
            self.file
                .constant_pool_mut()
                .push(ConstantEntry::Utf8(Utf8Info::from(attrs::CODE)));

            let code = Attr::Code(Code::with_insns(
                self.max_stack,
                self.max_locals,
                self.instructions.clone(),
            ));
            self.file.attributes_mut().push(code);
        }

        self.optimize();

        self.reset();
        None
    }

    fn visit_class_decl(
        &mut self,
        class_decl: &mut ClassDecl,
        _ctx: &mut CodeGenContext,
    ) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        let name_index =
            constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&class_decl.name.name)));
        self.file
            .declarations_mut()
            .push(Box::new(decls::ClassDecl::new(name_index)));
        None
    }

    fn visit_func_decl(
        &mut self,
        func_decl: &mut FuncDecl,
        ctx: &mut CodeGenContext,
    ) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        let name_index =
            constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&func_decl.name.name)));
        self.file
            .declarations_mut()
            .push(Box::new(decls::FuncDecl::new(name_index)));

        self.default_visit_func_decl(func_decl, ctx)
    }

    fn visit_var_stmt(
        &mut self,
        var_decl: &mut VarStmt,
        ctx: &mut CodeGenContext<'_>,
    ) -> Option<()> {
        // Visit the initializer first, if there is one
        // this ensures that the result of the initializer is pushed onto the stack before
        // the variable is declared
        if let Some(expr) = var_decl.initializer.as_mut() {
            self.visit_expr(expr, ctx);
        }

        let var_name = &var_decl.name.name.clone();
        let var_idx = match ctx.scope.push_var(VarSym::new(var_name.clone())) {
            // This duplicate variable error must have been handled during the attribution phase
            // if it wansn't somehow reported at that point, then we panic
            Err(_) => panic!("{}", &messages::err_dup_var(&var_name)),
            Ok(index) => index,
        };

        let opcode = match var_idx {
            0 => OpCode::Store0,
            1 => OpCode::Store1,
            2 => OpCode::Store2,
            3 => OpCode::Store3,
            _ => OpCode::Store,
        };

        if opcode != OpCode::Store {
            self.emitop(opcode);
        } else {
            self.emit1_16(opcode, var_idx);
        }

        // Update the max locals to account for the new variable
        self.update_max_locals(1);

        None
    }

    fn visit_block_stmt(
        &mut self,
        block_stmt: &mut BlockStmt,
        ctx: &mut CodeGenContext,
    ) -> Option<()> {
        
        // push a new scope of variables
        let mut scope = Scope::with_var_count(ctx.scope.var_count);
        scope.parent = Some(&ctx.scope);

        self.default_visit_block_stmt(block_stmt, &mut CodeGenContext::with_scope(scope, ctx.loops.as_mut()))
    }

    fn visit_for_stmt(
        &mut self,
        for_stmt: &mut ForStmt,
        ctx: &mut CodeGenContext<'_>,
    ) -> Option<()> {
        // step1: exec init stmt
        if let Some(init) = for_stmt.init.as_mut() {
            self.visit_stmt(init, ctx);
        }

        let cp = self.cp();
        let mut _continue = cp;
        
        ctx.push_loop(cp, LoopType::For, for_stmt.label.as_ref().map(|l| l.name.clone()));

        // step2: exec condition, if any
        if let Some(condition) = for_stmt.condition.as_mut() {
            self.visit_expr(condition, ctx);
        }

        // if condition is false, jump to the end
        let branch = self.emitjmp(OpCode::IfFalsy);
        self.emitop0(OpCode::Pop);

        // if condition is true, exec body
        self.visit_block_stmt(&mut for_stmt.body, ctx);

        // exec step expr, if any
        if let Some(step) = for_stmt.step.as_mut() {
            _continue = self.cp();
            self.visit_expr(step, ctx);
        }

        // step3: jmp to start of loop (condition check)
        self.emitjmp1(OpCode::Jmp, cp);

        self.jmptocp(branch);
        self.emitop0(OpCode::Pop);
        
        let _break = self.cp();

        let _loop = ctx.pop_loop().unwrap();
        self.patch_pending(_loop.pending_jumps.as_ref(), _continue, _break);

        None
    }

    fn visit_break_stmt(&mut self, break_stmt: &mut BreakStmt, ctx: &mut CodeGenContext<'_>) -> Option<()> {
        let _loop = ctx
            .find_loop(break_stmt.label.as_ref())
            .expect("Expected a loop context");

        let pc = self.emitjmp(OpCode::Jmp);
        _loop.pending_jumps.push(PendingJump::_break(pc));

        None
    }

    fn visit_continue_stmt(
        &mut self,
        continue_stmt: &mut ContinueStmt,
        ctx: &mut CodeGenContext<'_>,
    ) -> Option<()> {
        let _loop = ctx
            .find_loop(continue_stmt.label.as_ref())
            .expect("Expected a loop context");
        
        let pc = self.emitjmp(OpCode::Jmp);
        _loop.pending_jumps.push(PendingJump::_continue(pc));
            
        None
    }

    fn visit_if_stmt(&mut self, if_stmt: &mut IfStmt, ctx: &mut CodeGenContext) -> Option<()> {
        self.visit_expr(&mut if_stmt.condition, ctx);

        let jmp = self.emitjmp(OpCode::IfFalsy);
        self.emitop0(OpCode::Pop);
        self.visit_block_stmt(&mut if_stmt.then_branch, ctx);

        let mut jmpto = self.cp();
        if let Some(else_branch) = if_stmt.else_branch.as_mut() {
            let elsejmp = self.emitjmp(OpCode::Jmp);
            jmpto = self.cp();

            self.emitop0(OpCode::Pop);
            self.visit_block_stmt(else_branch, ctx);
            self.jmptocp(elsejmp);
        } else {
            // iffalsy (or iftruthy) does not pop the operands.
            // so we pop the condition operand manually
            // this must be done in any case, whether we have the else branch or not
            self.emitop0(OpCode::Pop);
        }

        self.patch_jmp(jmp, jmpto);

        None
    }

    fn visit_print_stmt(
        &mut self,
        print_stmt: &mut PrintStmt,
        ctx: &mut CodeGenContext,
    ) -> Option<()> {
        self.visit_expr(&mut print_stmt.expr, ctx);
        self.emitop(OpCode::Print);
        None
    }

    fn visit_while_stmt(
        &mut self,
        while_stmt: &mut WhileStmt,
        ctx: &mut CodeGenContext<'_>,
    ) -> Option<()> {
        let cp = self.cp();
        ctx.push_loop(
            cp,
            LoopType::While,
            while_stmt.label.as_ref().map(|l| l.name.clone()),
        );

        let _continue = cp;
        
        self.visit_expr(&mut while_stmt.condition, ctx);
        let jmp = self.emitjmp(OpCode::IfFalsy);
        self.emitop0(OpCode::Pop);

        self.visit_block_stmt(&mut while_stmt.body, ctx);
        self.emitjmp1(OpCode::Jmp, cp);

        self.jmptocp(jmp);
        self.emitop0(OpCode::Pop);
        
        let _break = self.cp();

        if let Some(_loop) = ctx.pop_loop() {
            self.patch_pending(_loop.pending_jumps.as_ref(), _continue, _break);
        }

        None
    }

    fn visit_assign_expr(
        &mut self,
        assign_expr: &mut AssignExpr,
        ctx: &mut CodeGenContext<'_>,
    ) -> Option<()> {
        if let Some(identifier) = assign_expr.target.Identifier() {
            self.visit_expr(&mut assign_expr.value, ctx);

            if let Some(idx) = ctx.scope.get_var_idx(&identifier.name) {
                let opcode = match &idx {
                    0 => OpCode::Store0,
                    1 => OpCode::Store1,
                    2 => OpCode::Store2,
                    3 => OpCode::Store3,
                    _ => OpCode::Store,
                };

                if opcode != OpCode::Store {
                    self.emitop(opcode);
                } else {
                    self.emit1_16(opcode, idx.clone());
                }

                return None;
            }
        }

        // TODO: Support more assign expressions
        panic!("Unsupported assign expr: {:?}", assign_expr);
    }

    fn visit_binary_expr(
        &mut self,
        binary: &mut BinaryExpr,
        ctx: &mut CodeGenContext,
    ) -> Option<()> {
        match &binary.op {
            BinaryOp::Plus | BinaryOp::Minus | BinaryOp::Mult | BinaryOp::Div => {
                self.visit_expr(&mut binary.left, ctx);
                self.visit_expr(&mut binary.right, ctx);

                let opcode = match binary.op {
                    BinaryOp::Plus => OpCode::Add,
                    BinaryOp::Minus => OpCode::Sub,
                    BinaryOp::Mult => OpCode::Mult,
                    BinaryOp::Div => OpCode::Div,
                    _ => unreachable!(),
                };

                self.emitop(opcode);
                return None;
            }

            BinaryOp::And => {
                self.handle_short_circuit(binary, OpCode::IfFalsy, ctx);
                return None;
            }

            BinaryOp::Or => {
                self.handle_short_circuit(binary, OpCode::IfTruthy, ctx);
                return None;
            }

            _ => {}
        }

        // 0(is_zero): whether any of the operands are 0
        // 1(on_left): whether the left operand is 0
        let (is_z, z_on_left) = binary
            .left
            .Literal()
            .and_then(|l| l.Number().map(|l| (&l.0 == &0f64, true)))
            .or_else(|| {
                binary
                    .right
                    .Literal()
                    .and_then(|l| l.Number().map(|l| (&l.0 == &0f64, false)))
            })
            .unwrap_or((false, false));

        // Determine the opcode which checks if the condition is **FALSE**
        // For example, the binary operator is EqEq in `if true == false`,
        // therefore, the else condition must be executed if true != false
        // as a result, the actual opcode should be `IfNe`
        let opcode = match (is_z, z_on_left) {
            (false, false) => opcode_cmp(&binary.op),
            (false, true) => opcode_cmp(&binary.op.inv_cmp().unwrap()),
            (true, false) => opcode_cmpz(&binary.op),

            (true, true) => {
                if binary.op != BinaryOp::EqEq && binary.op != BinaryOp::NotEq {
                    opcode_cmpz(&binary.op.inv_cmp().unwrap())
                } else {
                    // if 0 is the left operand, and the operator is EqEq or NotEq,
                    // we don't need to invert the binary operator
                    // this is becuase a == 0 and 0 == a have the same result
                    // same goes for a != 0 and 0 != a
                    opcode_cmpz(&binary.op)
                }
            }
        };

        match &binary.op {
            BinaryOp::EqEq
            | BinaryOp::NotEq
            | BinaryOp::Gt
            | BinaryOp::GtEq
            | BinaryOp::Lt
            | BinaryOp::LtEq => {
                let mut l = &mut binary.left;
                let mut r = &mut binary.right;

                // if zero is the left operand, swap the operands
                if z_on_left {
                    let t = l;
                    l = r;
                    r = t;
                }

                self.visit_expr(l, ctx);

                // if none of the operands are zero, we need to compare with the right operand
                if !is_z {
                    self.visit_expr(r, ctx);
                }

                // compare operands
                let ifcmps = self.emitjmp(opcode);

                // 1... if comparison succeeds, push true, and jmp to next insn
                self.emitop0(OpCode::BPush1);
                let cmpsj = self.emitjmp(OpCode::Jmp);

                // 2... if comparison fails, push false
                let push0 = self.cp();
                self.emitop0(OpCode::BPush0);

                self.jmptocp(cmpsj); // jump to next insn if comparison succeeds
                self.patch_jmp(ifcmps, push0); // push false if comparison succeeds
            }
            _ => unreachable!(),
        }

        None
    }

    fn visit_identifier_expr(
        &mut self,
        identifier: &mut IdentifierExpr,
        ctx: &mut CodeGenContext,
    ) -> Option<()> {
        let typ = identifier.ident_typ();

        if typ == &IdentifierType::ClassName {
            let constant_pool = self.file.constant_pool_mut();
            constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&identifier.name)));
        }

        if !typ.is_decl_name() && typ != &IdentifierType::Keyword {
            if let Some(idx) = ctx.scope.get_var_idx(&identifier.name) {
                let opcode = match idx {
                    0 => OpCode::Load0,
                    1 => OpCode::Load1,
                    2 => OpCode::Load2,
                    3 => OpCode::Load3,
                    _ => OpCode::Load,
                };
                if opcode != OpCode::Load {
                    self.emitop(opcode);
                } else {
                    self.emit1_16(opcode, idx.clone());
                }
            } else {
                panic!("Variable not found: {}", &identifier.name);
            }
        }

        None
    }

    fn visit_literal_expr(
        &mut self,
        literal: &mut LiteralExpr,
        _ctx: &mut CodeGenContext,
    ) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        match literal {
            LiteralExpr::Null(_) => {}
            LiteralExpr::Bool((boo, _)) => {
                self.emitop(if *boo { OpCode::BPush1 } else { OpCode::BPush0 });
            }
            LiteralExpr::Number((num, _)) => {
                let idx = constant_pool.push(ConstantEntry::Number(NumberInfo::from(num.deref())));
                self.emit1_16(OpCode::Ldc, idx);
            }
            LiteralExpr::String((str, _)) => {
                let str = &str[1..str.len() - 1]; // remove double quotes
                let idx = constant_pool.push_str(str);
                self.emit1_16(OpCode::Ldc, idx);
            }
        }

        None
    }
}
