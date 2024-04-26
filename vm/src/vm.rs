/*
 * Copyright (c) 2024 Akash Yadav
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

use std::any::Any;
use std::cmp::max;
use std::fmt::Display;

use log::error;
use log::Level::Trace;
use log::log_enabled;
use log::trace;
use log::warn;

use compiler::bytecode::attrs;
use compiler::bytecode::attrs::Attr;
use compiler::bytecode::attrs::Code;
use compiler::bytecode::bytes::AssertingByteConversions;
use compiler::bytecode::ConstantEntry;
use compiler::bytecode::ConstantPool;
use compiler::bytecode::CpSize;
use compiler::bytecode::opcode;
use compiler::bytecode::opcode::get_mnemonic;
use compiler::bytecode::opcode::OpSize;
use compiler::bytecode::YKBFile;

/// The YuvaKriti Virtual Machine
#[allow(unused)]
pub struct YKVM<'inst> {
    _s: &'inst dyn Any,
}

impl<'inst> YKVM<'inst> {
    pub fn new<'a>() -> YKVM<'a> {
        return YKVM { _s: &"" };
    }
}

impl<'inst> YKVM<'inst> {
    pub fn run(&mut self, file: &YKBFile) -> Result<Option<Value>, String> {
        let attrs = file.attributes();
        let code = attrs.iter().find(|attr| attr.name() == attrs::CODE);
        if code.is_none() {
            return Err(String::from("Missing code attribute"));
        }

        let attr = code.unwrap();
        let code = if let Attr::Code(code) = attr {
            code
        } else {
            return Err(format!(
                "Invalid attribute with name {}. Expected Code.",
                attrs::CODE
            ));
        };

        self.run_code(code, file.constant_pool())
    }

    /// Execute the instructions in the [Code] and returns the value at the top of the stack
    /// after execution.
    pub fn run_code(
        &mut self,
        code: &Code,
        constant_pool: &ConstantPool,
    ) -> Result<Option<Value>, String> {
        let mut executor = CodeExecutor::new(Some(constant_pool));
        executor.execute(code)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[allow(non_snake_case)]
impl Value {
    pub fn String(&self) -> Option<&String> {
        match self {
            Value::String(str) => Some(str),
            _ => None,
        }
    }

    pub fn Number(&self) -> Option<&f64> {
        match self {
            Value::Number(num) => Some(num),
            _ => None,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => false,
        }
    }

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(str) => write!(f, "{}", str),
            Value::Number(num) => write!(f, "{}", num),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    value: Value,
}

impl Variable {
    pub const NONE: Variable = Variable::new(Value::Null);
    pub const fn new(value: Value) -> Self {
        return Variable { value };
    }
}

/// The code executor responsible for executing the code.
pub struct CodeExecutor<'inst> {
    constant_pool: Option<&'inst ConstantPool>,
    variables: Vec<Variable>,
    operands: Vec<Value>,
    max_stack: u16,
    max_locals: u16,
}

macro_rules! read1 {
    ($insns:expr, $pc:expr) => {{
        let r = $insns[$pc];
        $pc += 1;
        r
    }};
}

macro_rules! read2 {
    ($insns:expr, $pc:expr) => {{
        let r = ($insns[$pc]).as_u16() << 8 | $insns[$pc + 1].as_u16();
        $pc += 2;
        r
    }};
}

impl<'inst> CodeExecutor<'inst> {
    pub fn new(constant_pool: Option<&ConstantPool>) -> CodeExecutor {
        CodeExecutor {
            constant_pool,
            variables: Vec::with_capacity(0),
            operands: Vec::with_capacity(0),
            max_stack: 0,
            max_locals: 0,
        }
    }

    pub fn constant_pool(&self) -> &ConstantPool {
        return self.constant_pool.unwrap();
    }

    pub fn set_constant_pool(&mut self, pool: Option<&'inst ConstantPool>) {
        if log_enabled!(Trace) {
            trace!("VM::set_constant_pool: {:?}", pool);
        }

        self.constant_pool = pool;
    }

    pub fn reset(&mut self) {
        if log_enabled!(Trace) {
            trace!("VM::reset()");
        }
        self.variables = vec![];
        self.operands = vec![];
    }

    fn try_peek_operand(&mut self) -> Option<&Value> {
        self.operands.last()
    }

    fn try_peek_operand_mut(&mut self) -> Option<&mut Value> {
        self.operands.last_mut()
    }

    fn peek_operand(&mut self) -> &Value {
        self.try_peek_operand()
            .expect("Expected an operand to peek")
    }

    fn peek_operand_mut(&mut self) -> &mut Value {
        self.try_peek_operand_mut()
            .expect("Expected an operand to peek")
    }

    fn try_pop_operand(&mut self) -> Option<Value> {
        self.operands.pop()
    }

    fn pop_operand(&mut self) -> Value {
        if log_enabled!(Trace) {
            trace!("VM::pop_operand()");
        }
        let op = self.try_pop_operand().expect("Expected an operand to pop");

        if log_enabled!(Trace) {
            trace!("VM::pop_operand(): {:?}", op);
            trace!("VM::pop_operand(): operands.len(): {}", self.operands.len());
        }

        op
    }

    fn push_operand(&mut self, value: Value) {
        if log_enabled!(Trace) {
            trace!("VM::push_operand({:?})", value);
        }
        if self.max_stack != 0 && self.operands.len() >= self.max_stack as usize {
            panic!("Critical: Operand stack overflow! max_stack={}. Did the compiler compute invalid stack depth?", self.max_stack);
        }

        self.operands.push(value);

        if log_enabled!(Trace) {
            trace!(
                "VM::push_operand(): operands.len(): {}",
                self.operands.len()
            );
        }
    }

    fn load_constant(&mut self, index: CpSize) {
        if log_enabled!(Trace) {
            trace!("VM::load_constant({})", index);
        }

        if let Some(str) = self.constant_pool().get_string(index) {
            self.push_operand(Value::String(str));
            return;
        }

        let constant = self
            .constant_pool()
            .get(index)
            .expect(&format!("Expected constant at index {}", index));

        match constant {
            ConstantEntry::Number(num) => self.push_operand(Value::Number(num.to_f64())),
            _ => {
                warn!(
                    "Unsupported constant type: {:?}, index: {}, ignoring.",
                    constant, index
                );
            }
        }
    }

    pub fn store_var(&mut self, index: u16) {
        if log_enabled!(Trace) {
            trace!("VM::store_var({})", index);
        }
        let value = self.try_pop_operand().unwrap_or(Value::Null);
        self.variables[index as usize] = Variable::new(value);
    }

    pub fn load_var(&mut self, index: u16) {
        if log_enabled!(Trace) {
            trace!("VM::load_var({})", index);
        }
        // TODO(itsaky): Avoid cloning
        let value = self.variables[index as usize].value.clone();
        self.push_operand(value);
    }

    pub fn execute(&mut self, code: &Code) -> Result<Option<Value>, String> {
        self.max_stack = code.max_stack();
        self.max_locals = code.max_locals();

        if log_enabled!(Trace) {
            trace!(
                "VM::execute(max_stack={}, max_locals={})",
                self.max_stack,
                self.max_locals
            );
        }

        self.operands = Vec::with_capacity(max(0, self.max_stack) as usize);

        self.variables.clear();
        self.variables
            .resize(max(0, self.max_locals) as usize, Variable::NONE);

        let insns = code.instructions();
        if log_enabled!(Trace) {
            trace!("VM::execute(insns.len()={})", insns.len());
        }

        let mut pc = 0;
        let mut is_halted = false;

        'insn: while pc < insns.len() {
            let insn = read1!(insns, pc).as_op_size();
            if log_enabled!(Trace) {
                trace!(
                    "VM::execute(pc={}, instruction={}, opcode={:?})",
                    pc,
                    insn,
                    get_mnemonic(&insn)
                );
            }

            match insn {
                opcode::Nop => {}
                opcode::Halt => {
                    is_halted = true;
                    break 'insn;
                }
                opcode::Add | opcode::Sub | opcode::Mult | opcode::Div => {
                    self.exec_arithmetic(&insn);
                }
                opcode::Print => {
                    let value = self.pop_operand();
                    println!("{}", value);
                }
                opcode::Ldc => {
                    let const_idx = read2!(insns, pc);
                    self.load_constant(const_idx);
                }
                opcode::BPush0 => self.push_operand(Value::Bool(false)),
                opcode::BPush1 => self.push_operand(Value::Bool(true)),
                opcode::Store0 => self.store_var(0),
                opcode::Store1 => self.store_var(1),
                opcode::Store2 => self.store_var(2),
                opcode::Store3 => self.store_var(3),
                opcode::Store => {
                    let var_idx = read2!(insns, pc);
                    self.store_var(var_idx);
                }
                opcode::Load0 => self.load_var(0),
                opcode::Load1 => self.load_var(1),
                opcode::Load2 => self.load_var(2),
                opcode::Load3 => self.load_var(3),
                opcode::Load => {
                    let var_idx = read2!(insns, pc);
                    self.load_var(var_idx);
                }

                opcode::IfTruthy | opcode::IfFalsy => {
                    let addr = read2!(insns, pc) as i16;
                    let value = self.peek_operand();

                    if (insn == opcode::IfTruthy && value.is_truthy())
                        || (insn == opcode::IfFalsy && value.is_falsy())
                    {
                        // jump to the specified address
                        jmp(&mut pc, addr);
                        if log_enabled!(Trace) {
                            trace!("VM::execute::jmp(pc={})", pc);
                        }
                    }
                }

                opcode::IfEq
                | opcode::IfNe
                | opcode::IfLt
                | opcode::IfGt
                | opcode::IfLe
                | opcode::IfGe => {
                    let addr = read2!(insns, pc) as i16;
                    if self.cmp(&insn) {
                        jmp(&mut pc, addr);
                    }
                }

                opcode::IfEqZ
                | opcode::IfNeZ
                | opcode::IfLtZ
                | opcode::IfGtZ
                | opcode::IfLeZ
                | opcode::IfGeZ => {
                    let addr = read2!(insns, pc) as i16;
                    if self.cmpz(&insn) {
                        jmp(&mut pc, addr);
                    }
                }

                opcode::Jmp => {
                    let addr = read2!(insns, pc) as i16;
                    jmp(&mut pc, addr);
                    if log_enabled!(Trace) {
                        trace!("VM::execute::jmp(pc={})", pc);
                    }
                }

                opcode::Pop => {
                    if log_enabled!(Trace) {
                        trace!("VM::execute::pop()");
                    }
                    self.pop_operand();
                }

                opcode::Neg => {
                    let value = self.pop_operand();
                    self.push_operand(match value {
                        Value::Number(num) => Value::Number(-num),
                        _ => {
                            // TODO: Should we warn the user?
                            Value::Number(0.0)
                        }
                    });
                }

                opcode::Not => {
                    let value = self.pop_operand();
                    self.push_operand(match value {
                        Value::Bool(bool) => Value::Bool(!bool),
                        _ => {
                            // TODO: Should we warn the user?
                            Value::Bool(false)
                        }
                    });
                }

                _ => panic!("Unexpected instruction: {:?}", insn),
            }
        }

        if pc != insns.len() && !is_halted {
            error!(
                "Expected all instructions to be executed, but {} bytes are remaining",
                insns.len() - pc
            );
        }

        trace!("VM::execute(): pc: {}, is_halted: {}", pc, is_halted);

        let result = self.try_pop_operand();
        if result.is_some() {
            trace!("VM::execute(): result: {:?}", result);
        }
        
        self.variables.clear();
        self.operands.clear();

        // Return the result at the top of the stack
        Ok(result)
    }

    fn cmp(&mut self, op: &OpSize) -> bool {
        let op2 = self.pop_operand();
        let op1 = self.pop_operand();

        if let (Some(n2), Some(n1)) = (op2.Number(), op1.Number()) {
            return match op {
                &opcode::IfEq => n1 == n2,
                &opcode::IfNe => n1 != n2,
                &opcode::IfLt => n1 < n2,
                &opcode::IfLe => n1 <= n2,
                &opcode::IfGt => n1 > n2,
                &opcode::IfGe => n1 >= n2,
                _ => unreachable!("cmp is not implemented for {:?}", op),
            };
        }

        // TODO: implement comparison between other types
        false
    }

    fn cmpz(&mut self, op_code: &OpSize) -> bool {
        let op = self.pop_operand();
        if let Some(n) = op.Number() {
            return match op_code {
                &opcode::IfEqZ => n == &0f64,
                &opcode::IfNeZ => n != &0f64,
                &opcode::IfLtZ => n < &0f64,
                &opcode::IfLeZ => n <= &0f64,
                &opcode::IfGtZ => n > &0f64,
                &opcode::IfGeZ => n >= &0f64,
                _ => unreachable!("cmp is not implemented for {:?}", op),
            };
        }

        // TODO: implement comparison between other types
        false
    }

    fn exec_arithmetic(&mut self, op: &OpSize) {
        let op2 = self.pop_operand();
        let op1 = self.peek_operand_mut();

        match (op1, op2) {
            (Value::Number(n1), Value::Number(n2)) => {
                *n1 = match op {
                    &opcode::Add => *n1 + n2,
                    &opcode::Sub => *n1 - n2,
                    &opcode::Mult => *n1 * n2,
                    &opcode::Div => *n1 / n2,
                    _ => panic!("Expected a binary numeric operator"),
                };
            }
            _ => panic!("Expected numeric operands"),
        };
    }
}

#[inline(always)]
fn jmp(pc: &mut usize, offset: i16) {
    *pc = pc
        .checked_add_signed(offset as isize)
        .expect(&format!("Invalid jump address: too big: {}", offset));
}
