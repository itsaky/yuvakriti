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

use crate::ast::BlockStmt;
use crate::ast::ClassDecl;
use crate::ast::FuncDecl;
use crate::ast::IdentifierExpr;
use crate::ast::LiteralExpr;
use crate::ast::PrintStmt;
use crate::ast::Program;
use crate::ast::VarStmt;
use crate::ast::Visitable;
use crate::ast::{ASTVisitor, IdentifierType};
use crate::ast::{AssignExpr, BinaryExpr};
use crate::ast::{BinaryOp, IfStmt};
use crate::bytecode::attrs::{Attr, Code, CodeSize};
use crate::bytecode::bytes::AssertingByteConversions;
use crate::bytecode::cp::ConstantEntry;
use crate::bytecode::cp_info::NumberInfo;
use crate::bytecode::cp_info::Utf8Info;
use crate::bytecode::file::YKBFile;
use crate::bytecode::opcode::{get_opcode, opcode_cmp, opcode_cmpz, OpCode, OpCodeExt};
use crate::bytecode::{attrs, decls};
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
        let mut fpv = CodeGen::new(&mut self.file, &self.features);
        let mut context = CodeGenContext::new();
        program.accept(&mut fpv, &mut context);
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
    pending_jumps: Option<Chain>,
    cp: CodeSize,
    instructions: Vec<u8>,
}

struct CodeGenContext<'a> {
    pub scope: Scope<'a>,
    pub chain: Option<Chain>,
    pub resolve_chain: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Chain {
    pub pc: CodeSize,
    pub next: Option<Box<Chain>>,
}

impl Chain {
    pub fn new(pc: CodeSize) -> Self {
        return Chain { pc, next: None };
    }
}

impl CodeGenContext<'_> {
    fn new<'a>() -> CodeGenContext<'a> {
        return CodeGenContext {
            scope: Scope::new(),
            chain: None,
            resolve_chain: true,
        };
    }

    fn with_scope(scope: Scope) -> CodeGenContext {
        return CodeGenContext {
            scope,
            chain: None,
            resolve_chain: true,
        };
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
            pending_jumps: None,
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

    fn len(&self) -> usize {
        return self.instructions().len();
    }

    fn get1(&self, index: CodeSize) -> u8 {
        return self.instructions[index as usize];
    }
    fn get2(&self, index: CodeSize) -> u16 {
        return (self.instructions[index as usize].as_u16() << 8)
            | self.instructions[index as usize + 1].as_u16();
    }

    fn put1(&mut self, index: CodeSize, value: u8) {
        self.instructions[index as usize] = value;
    }
    fn put2(&mut self, index: CodeSize, f: u8, s: u8) {
        self.instructions[index as usize] = f;
        self.instructions[index as usize + 1] = s;
    }
    fn put2_16(&mut self, index: CodeSize, value: u16) {
        self.instructions[index as usize] = (value >> 8).as_u8();
        self.instructions[index as usize + 1] = value.as_u8();
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

    fn emit1(&mut self, opcode: OpCode, operand: u8) {
        self.ensure_size_incr(2);

        self.instructions[self.cp as usize] = opcode.as_op_size();
        self.instructions[self.cp as usize + 1] = operand;
        self.cp += 2;

        self._update_max_stack(opcode)
    }

    fn emit1_16(&mut self, opcode: OpCode, operand: u16) {
        self.ensure_size_incr(3);
        self.instructions[self.cp as usize] = opcode.as_op_size();
        self.instructions[self.cp as usize + 1] = (operand >> 8).as_u8();
        self.instructions[self.cp as usize + 2] = operand.as_u8();
        self.cp += 3;

        self._update_max_stack(opcode);
    }

    fn emit2(&mut self, opcode: OpCode, operand1: u8, operand2: u8) {
        self.ensure_size_incr(3);
        self.instructions[self.cp as usize] = opcode.as_op_size();
        self.instructions[self.cp as usize + 1] = operand1;
        self.instructions[self.cp as usize + 2] = operand2;
        self.cp += 3;

        self._update_max_stack(opcode);
    }

    fn push_insns_3(&mut self, opcode: OpCode, operand1: u8, operand2: u8, operand3: u8) {
        self.ensure_size_incr(4);
        self.instructions[self.cp as usize] = opcode.as_op_size();
        self.instructions[self.cp as usize + 1] = operand1;
        self.instructions[self.cp as usize + 2] = operand2;
        self.instructions[self.cp as usize + 3] = operand3;
        self.cp += 4;

        self._update_max_stack(opcode);
    }

    fn patch(&mut self, index: CodeSize, data: u8) {
        self.instructions[index as usize] = data;
    }

    fn patch_16(&mut self, index: CodeSize, data: u16) {
        self.instructions[index as usize] = (data >> 8) as u8;
        self.instructions[index as usize + 1] = data as u8;
    }

    fn jmptocp(&mut self, jmp_from: CodeSize) {
        self.patch_16(jmp_from + 1, (self.cp() - jmp_from - 3).as_u16());
    }

    fn patch_jmp(&mut self, jmp_from: CodeSize, jmp_to: CodeSize) {
        self.patch_16(jmp_from + 1, (jmp_to - jmp_from - 3).as_u16());
    }

    fn emitjmp(&mut self, opcode: OpCode) -> CodeSize {
        self.emit1_16(opcode, 0);
        return self.cp() - 3;
    }

    fn reset(&mut self) {
        self.stack_count = 0;
        self.max_stack = 0;
        self.local_count = 0;
        self.max_locals = 0;
        self.pending_jumps = None;
        self.cp = 0;
        self.instructions = Vec::with_capacity(0);
    }

    fn branch(&mut self, op: OpCode, ctx: &mut CodeGenContext) -> CodeSize {
        let pc = self.emitjmp(op);
        let mut new = Chain::new(pc);
        new.next = ctx.chain.take().map(|c| Box::from(c));
        ctx.chain = Some(new);
        return pc;
    }

    fn resolve_chain(&mut self, ctx: &mut CodeGenContext) {
        if let Some(chain) = ctx.chain.as_mut() {
            self._resolve(chain);
        }

        ctx.chain = None;
    }

    fn _resolve(&mut self, chain: &mut Chain) {
        if let Some(next) = chain.next.as_deref_mut() {
            let pc = chain.pc;
            let op = get_opcode(self.get1(pc));
            let target = self.get2(pc + 1).as_code_size();

            let nxtpc = next.pc;
            let nxtop = get_opcode(self.get1(nxtpc));

            if op == nxtop || (op == OpCode::Jmp && nxtop.is_jmp()) {
                self.patch_jmp(nxtpc, pc + target + 3);
            }

            self._resolve(next);
        }
    }

    fn handle_short_circuit(
        &mut self,
        binary: &mut BinaryExpr,
        op: OpCode,
        ctx: &mut CodeGenContext,
    ) {
        let resolve = ctx.resolve_chain;
        ctx.resolve_chain = false;

        self.visit_expr(&mut binary.left, ctx);

        let jmpiffalsy = self.branch(op, ctx);
        self.emitop0(OpCode::Pop);

        self.visit_expr(&mut binary.right, ctx);
        self.jmptocp(jmpiffalsy);

        if resolve {
            self.resolve_chain(ctx);
        }

        ctx.resolve_chain = resolve;
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
        let mut scope = Scope::new();
        scope.parent = Some(&ctx.scope);

        let mut new_ctx = CodeGenContext::with_scope(scope);
        new_ctx.chain = ctx.chain.clone();
        new_ctx.resolve_chain = ctx.resolve_chain;

        self.default_visit_block_stmt(block_stmt, &mut new_ctx)
    }

    fn visit_if_stmt(&mut self, if_stmt: &mut IfStmt, ctx: &mut CodeGenContext) -> Option<()> {
        self.visit_expr(&mut if_stmt.condition, ctx);

        let jmp = self.branch(OpCode::IfFalsy, ctx);
        self.emitop0(OpCode::Pop);
        self.visit_block_stmt(&mut if_stmt.then_branch, ctx);

        let mut jmpto = self.cp();
        if let Some(else_branch) = if_stmt.else_branch.as_mut() {
            let elsejmp = self.branch(OpCode::Jmp, ctx);
            jmpto = self.cp();

            self.emitop0(OpCode::Pop);
            self.visit_block_stmt(else_branch, ctx);
            self.jmptocp(elsejmp);
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
        let (is_zero, on_left) = binary
            .left
            .Literal()
            .and_then(|l| l.Number())
            .or_else(|| binary.right.Literal().and_then(|l| l.Number()))
            .map(|l| (&l.0 == &0f64, true))
            .unwrap_or((false, false));

        // Determine the opcode for when the condition is **FALSE**
        // For example, the binary operator is EqEq in `if true == false`,
        // therefore, the else condition must be executed if true != false
        // as a result, the actual opcode should be `IfNe`
        let _opcode = match (is_zero, on_left) {
            (false, false) => opcode_cmp(&binary.op),
            (false, true) => opcode_cmp(&binary.op.inv_cmp().unwrap()),
            (true, false) => opcode_cmpz(&binary.op),
            (true, true) => opcode_cmpz(&binary.op.inv_cmp().unwrap()),
        };

        match &binary.op {
            BinaryOp::EqEq => {}
            BinaryOp::NotEq => {}
            BinaryOp::Gt => {}
            BinaryOp::GtEq => {}
            BinaryOp::Lt => {}
            BinaryOp::LtEq => {}
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
