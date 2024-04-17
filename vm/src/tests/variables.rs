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

use compiler::bytecode::cp_info::NumberInfo;
use compiler::bytecode::opcode::OpCode;
use compiler::bytecode::opcode::OpSize;
use compiler::bytecode::ConstantEntry;

use crate::tests::util::create_constant_pool;
use crate::tests::util::create_vm;
use crate::tests::util::eval_arithemetic;
use crate::tests::util::push_constants;

#[test]
fn test_simple_var_decls() {
    let mut vm = create_vm();
    let mut constant_pool = create_constant_pool();
    push_constants(
        &mut constant_pool,
        vec![
            ConstantEntry::Number(NumberInfo::from(&10f64)),
            ConstantEntry::Number(NumberInfo::from(&20f64)),
        ],
    );

    // var a = 10;
    // var b = 20;
    // a + b
    assert_eq!(
        30f64,
        eval_arithemetic(
            &mut vm,
            &constant_pool,
            2,
            2,
            vec![
                OpCode::Ldc as OpSize,
                0x00,
                0x01,
                OpCode::Store0 as OpSize,
                OpCode::Ldc as OpSize,
                0x00,
                0x02,
                OpCode::Store1 as OpSize,
                OpCode::Load0 as OpSize,
                OpCode::Load1 as OpSize,
                OpCode::Add as OpSize,
            ]
        )
    )
}
