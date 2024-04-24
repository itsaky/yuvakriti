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

use compiler::bytecode::cp_info::NumberInfo;
use compiler::bytecode::opcode::OpCode;
use compiler::bytecode::opcode::OpSize;
use compiler::bytecode::ConstantEntry;

use crate::tests::util::create_constant_pool;
use crate::tests::util::create_vm;
use crate::tests::util::eval_arithemetic;
use crate::tests::util::push_constants;

#[test]
fn test_simple_arithemetic_evaluation() {
    let mut vm = create_vm();

    let mut cp = create_constant_pool();
    push_constants(
        &mut cp,
        vec![
            ConstantEntry::Number(NumberInfo::from(&10f64)),
            ConstantEntry::Number(NumberInfo::from(&20f64)),
        ],
    );

    #[rustfmt::skip]
    assert_eq!(30f64, eval_arithemetic(&mut vm, &cp, 2, 0,  vec![
        OpCode::Ldc as OpSize, 0x00, 0x01, // single operand, but u16
        OpCode::Ldc as OpSize, 0x00, 0x02, // single operand, but u16
        OpCode::Add as OpSize, // 10 + 20
    ]));

    #[rustfmt::skip]
    assert_eq!(10f64, eval_arithemetic(&mut vm, &cp, 2, 0, vec![
        OpCode::Ldc as OpSize, 0x00, 0x02, // single operand, but u16
        OpCode::Ldc as OpSize, 0x00, 0x01, // single operand, but u16
        OpCode::Sub as OpSize, // 20 - 10
    ]));

    #[rustfmt::skip]
    assert_eq!(200f64, eval_arithemetic(&mut vm, &cp, 2, 0,  vec![
        OpCode::Ldc as OpSize, 0x00, 0x01, // single operand, but u16
        OpCode::Ldc as OpSize, 0x00, 0x02, // single operand, but u16
        OpCode::Mult as OpSize, // 10 * 20
    ]));

    #[rustfmt::skip]
    assert_eq!(2f64, eval_arithemetic(&mut vm, &cp, 2, 0, vec![
        OpCode::Ldc as OpSize, 0x00, 0x02, // single operand, but u16
        OpCode::Ldc as OpSize, 0x00, 0x01, // single operand, but u16
        OpCode::Div as OpSize, // 20 / 10
    ]));
}
