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

use bytecode::attrs::Code;
use bytecode::opcode::OpSize;
use bytecode::ConstantEntry;
use bytecode::ConstantPool;

use crate::YKVM;

pub fn create_vm<'a>() -> YKVM<'a> {
    YKVM::new()
}

pub fn create_constant_pool() -> ConstantPool {
    ConstantPool::new()
}

pub fn push_constants(constant_pool: &mut ConstantPool, constants: Vec<ConstantEntry>) {
    for constant in constants {
        constant_pool.push(constant);
    }
}

pub fn eval_arithemetic(vm: &mut YKVM, cp: &ConstantPool, insns: Vec<OpSize>) -> f64 {
    let code = Code::with_insns(insns);
    let result = vm.run_code(&code, cp).unwrap().expect("Expected result");
    result.as_number().unwrap().clone()
}
