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

use std::any::Any;
use std::fmt::Display;

use log::debug;
use log::error;
use log::warn;

use compiler::bytecode::attrs;
use compiler::bytecode::attrs::Attr;
use compiler::bytecode::attrs::Code;
use compiler::bytecode::bytes::AssertingByteConversions;
use compiler::bytecode::opcode::get_opcode;
use compiler::bytecode::opcode::OpCode;
use compiler::bytecode::ConstantEntry;
use compiler::bytecode::ConstantPool;
use compiler::bytecode::CpSize;
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
    pub fn run(&mut self, file: &YKBFile) -> Result<(), String> {
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

        self.run_code(code, file.constant_pool()).map(|_res| ())
    }

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
    Nil,
}

impl Value {
    pub fn as_str(&self) -> Option<&String> {
        match self {
            Value::String(str) => Some(str),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<&f64> {
        match self {
            Value::Number(num) => Some(num),
            _ => None,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(str) => {
                write!(f, "{}", str)
            }
            Value::Number(num) => {
                write!(f, "{}", num)
            }
            Value::Bool(b) => {
                write!(f, "{}", b)
            }
            Value::Nil => {
                write!(f, "nil")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    value: Value,
}

impl Variable {
    pub const NONE: Variable = Variable::new(Value::Nil);
    pub const fn new(value: Value) -> Self {
        return Variable { value };
    }
}

/// The code executor responsible for executing the code.
pub struct CodeExecutor<'inst> {
    constant_pool: Option<&'inst ConstantPool>,
    variables: Vec<Variable>,
    operands: Vec<Value>,
}

impl<'inst> CodeExecutor<'inst> {
    pub fn new(constant_pool: Option<&ConstantPool>) -> CodeExecutor {
        CodeExecutor {
            constant_pool,
            variables: vec![],
            operands: vec![],
        }
    }

    pub fn constant_pool(&self) -> &ConstantPool {
        return self.constant_pool.unwrap();
    }

    pub fn set_constant_pool(&mut self, pool: Option<&'inst ConstantPool>) {
        self.constant_pool = pool;
    }

    pub fn reset(&mut self) {
        self.variables = vec![];
        self.operands = vec![];
    }

    fn try_pop_operand(&mut self) -> Option<Value> {
        self.operands.pop()
    }

    fn pop_operand(&mut self) -> Value {
        self.try_pop_operand().expect("Expected an operand to pop")
    }

    fn push_operand(&mut self, value: Value) {
        self.operands.push(value);
    }

    fn load_constant(&mut self, index: CpSize) {
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

    pub fn execute(&mut self, code: &Code) -> Result<Option<Value>, String> {
        let instructions = code.instructions();
        let mut index = 0;
        let mut is_halted = false;

        'insn: while index < instructions.len() {
            let instruction = instructions[index].as_op_size();
            index += 1;
            let opcode = get_opcode(instruction);
            match opcode {
                OpCode::Nop => {
                    debug!("Encountered a nop opcode. Skipping.");
                }
                OpCode::Halt => {
                    debug!("Encountered halt opcode. Halting.");
                    is_halted = true;
                    break 'insn;
                }
                OpCode::Add | OpCode::Sub | OpCode::Mult | OpCode::Div => {
                    self.exec_binary_num_op(opcode);
                }
                OpCode::Print => {
                    let value = self.pop_operand();
                    println!("{}", value);
                }
                OpCode::Ldc => {
                    let const_idx = instructions[index].as_cp_size() << 8
                        | instructions[index + 1].as_cp_size();

                    // we consumed 2 bytes here, so increment the index
                    index += 2;

                    self.load_constant(const_idx);
                }
                OpCode::BPush0 => self.push_operand(Value::Bool(false)),
                OpCode::BPush1 => self.push_operand(Value::Bool(true)),

                _ => return Err(format!("Unsupported opcode: {}", opcode)),
            }
        }

        if index != instructions.len() && !is_halted {
            error!(
                "Expected all instructions to be executed, but {} bytes are remaining",
                instructions.len() - index
            );
        }

        // Return the result at the top of the stack
        Ok(self.try_pop_operand())
    }

    pub fn exec_binary_num_op(&mut self, op: OpCode) {
        let op2 = self.pop_operand();
        let op1 = self.pop_operand();
        let op2 = op2.as_number().expect("Expected a number operand");
        let op1 = op1.as_number().expect("Expected a number operand");

        let result = match op {
            OpCode::Add => op1 + op2,
            OpCode::Sub => op1 - op2,
            OpCode::Mult => op1 * op2,
            OpCode::Div => op1 / op2,
            _ => panic!("Expected a binary numeric operator"),
        };

        self.push_operand(Value::Number(result))
    }
}
