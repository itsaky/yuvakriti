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

use crate::ast::BinaryOp;
use std::fmt::Display;

pub type OpSize = u8;

pub trait OpCodeExt {
    fn stack_effect(&self) -> i8;
    fn get_mnemonic(&self) -> &'static str;
    fn is_jmp(&self) -> bool;
}

macro_rules! def_opcodes {
    ($({$name:ident, $code:literal, $stack_effect:literal, $mnemonic:literal $(, $jmp:expr $(,)?)? } $(,)?)+) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        #[repr(u8)]
        #[rustfmt::skip]
        pub enum OpCode {
            $($name = $code,)+
        }

        impl OpCodeExt for OpCode {
            fn stack_effect(&self) -> i8 {
                match self {
                    $(OpCode::$name => $stack_effect,)+
                }
            }

            fn get_mnemonic(&self) -> &'static str {
                match self {
                    $(OpCode::$name => $mnemonic,)+
                }
            }
            
            fn is_jmp(&self) -> bool {
                match self {
                    $($(OpCode::$name => $jmp,)*)+
                    _ => false
                }
            }
        }

        pub fn get_opcode(code: OpSize) -> OpCode {
            match code {
                $($code => OpCode::$name,)+
                _ => unreachable!("Unknown/unsupported opcode: {:?}", code),
            }
        }
    };
}

impl OpCode {
    pub fn as_op_size(self) -> OpSize {
        return self as OpSize;
    }
}

// format: {name, opcode, stack_effect, mnemonic, is_jmp}
def_opcodes!(
  {Nop,         0x00,   0,  "nop"       },
  {Halt,        0x01,   0,  "halt"      },
  {Add,         0x02,  -1,  "add"       },
  {Sub,         0x03,  -1,  "sub"       },
  {Mult,        0x04,  -1,  "mult"      },
  {Div,         0x05,  -1,  "div"       },
  {Print,       0x06,  -1,  "print"     },
  {IfEq,        0x07,   0,  "ifeq"      , true},
  {IfEqZ,       0x08,   0,  "ifeqz"     , true},
  {IfNe,        0x09,   0,  "ifne"      , true},
  {IfNeZ,       0x0A,   0,  "ifnez"     , true},
  {IfLt,        0x0B,   0,  "iflt"      , true},
  {IfLtZ,       0x0C,   0,  "ifltz"     , true},
  {IfLe,        0x0D,   0,  "ifle"      , true},
  {IfLeZ,       0x0E,   0,  "iflez"     , true},
  {IfGt,        0x0F,   0,  "ifgt"      , true},
  {IfGtZ,       0x10,   0,  "ifgtz"     , true},
  {IfGe,        0x11,   0,  "ifge"      , true},
  {IfGeZ,       0x12,   0,  "ifgez"     , true},
  {IfTruthy,    0x20,   0,  "iftruthy"  , true},
  {IfFalsy,     0x21,   0,  "iffalsy"   , true},
  {Ldc,         0x13,   1,  "ldc"       },
  {BPush0,      0x14,   1,  "bpush_0"   },
  {BPush1,      0x15,   1,  "bpush_1"   },
  {Store,       0x16,  -1,  "store"     },
  {Store0,      0x17,  -1,  "store_0"   },
  {Store1,      0x18,  -1,  "store_1"   },
  {Store2,      0x19,  -1,  "store_2"   },
  {Store3,      0x1A,  -1,  "store_3"   },
  {Load,        0x1B,   1,  "load"      },
  {Load0,       0x1C,   1,  "load_0"    },
  {Load1,       0x1D,   1,  "load_1"    },
  {Load2,       0x1E,   1,  "load_2"    },
  {Load3,       0x1F,   1,  "load_3"    },
  {Jmp,         0x22,   0,  "jmp"       , true},
  {Pop,         0x23,   -1,  "pop"      },
);

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_mnemonic())
    }
}

pub fn opcode_cmp(op: &BinaryOp) -> OpCode {
    match op {
        BinaryOp::EqEq => OpCode::IfNe,
        BinaryOp::NotEq => OpCode::IfEq,
        BinaryOp::Gt => OpCode::IfLt,
        BinaryOp::GtEq => OpCode::IfLe,
        BinaryOp::Lt => OpCode::IfGt,
        BinaryOp::LtEq => OpCode::IfGe,
        _ => unreachable!("opcode_cmp is not implemented for {:?}", op),
    }
}

pub fn opcode_cmpz(op: &BinaryOp) -> OpCode {
    match op {
        BinaryOp::EqEq => OpCode::IfNeZ,
        BinaryOp::NotEq => OpCode::IfEqZ,
        BinaryOp::Gt => OpCode::IfLtZ,
        BinaryOp::GtEq => OpCode::IfLeZ,
        BinaryOp::Lt => OpCode::IfGtZ,
        BinaryOp::LtEq => OpCode::IfGeZ,
        _ => unreachable!("opcode_cmpz is not implemented for {:?}", op),
    }
}
