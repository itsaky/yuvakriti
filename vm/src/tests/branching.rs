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

use crate::tests::util::create_constant_pool;
use crate::tests::util::create_vm;
use crate::tests::util::eval_arithemetic;
use crate::tests::util::push_constants;
use compiler::bytecode::cp_info::NumberInfo;
use compiler::bytecode::opcode::OpCode;
use compiler::bytecode::opcode::OpSize;
use compiler::bytecode::ConstantEntry;

#[test]
fn test_simple_branching() {
    let mut vm = create_vm();

    let mut cp = create_constant_pool();
    push_constants(
        &mut cp,
        vec![
            ConstantEntry::Number(NumberInfo::from(&10f64)),
            ConstantEntry::Number(NumberInfo::from(&20f64)),
        ],
    );

    // if false {
    //    b + a
    // } else {
    //    b - a
    // }
    #[rustfmt::skip]
    assert_eq!(10f64, eval_arithemetic(&mut vm, &cp, 2, 0,  vec![
        OpCode::BPush0 as OpSize,
        OpCode::IfFalsy as OpSize, 0x00, 0x0E,
        OpCode::Ldc as OpSize, 0x00, 0x02,
        OpCode::Ldc as OpSize, 0x00, 0x01,
        OpCode::Add as OpSize, // 20 + 10
        OpCode::Jmp as OpSize, 0x00, 0x15,
        OpCode::Ldc as OpSize, 0x00, 0x02,
        OpCode::Ldc as OpSize, 0x00, 0x01,
        OpCode::Sub as OpSize, // 20 - 10
    ]));
}
