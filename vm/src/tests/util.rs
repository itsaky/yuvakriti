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

use std::io::Cursor;

use compiler::bytecode::attrs::Code;
use compiler::bytecode::opcode::OpSize;
use compiler::bytecode::ConstantEntry;
use compiler::bytecode::ConstantPool;
use compiler::comp::YKCompiler;
use compiler::features::CompilerFeatures;

use crate::{Value, YKVM};

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

pub fn eval_arithemetic(
    vm: &mut YKVM,
    cp: &ConstantPool,
    max_stack: u16,
    max_locals: u16,
    insns: Vec<OpSize>,
) -> f64 {
    eval(vm, cp, max_stack, max_locals, insns)
        .Number()
        .unwrap()
        .clone()
}

pub fn eval(
    vm: &mut YKVM,
    cp: &ConstantPool,
    max_stack: u16,
    max_locals: u16,
    insns: Vec<OpSize>,
) -> Value {
    let code = Code::with_insns(max_stack, max_locals, insns);
    vm.run_code(&code, cp).unwrap().expect("Expected result")
}

pub fn eval_arithmetic_src(src: &str) -> f64 {
    eval_src(src).Number().unwrap().clone()
}

pub fn eval_src(src: &str) -> Value {
    let mut vm = YKVM::new();
    let mut compiler = YKCompiler::new();
    let mut features = CompilerFeatures::default();
    features.const_folding = false;

    let (mut program, has_errors) = compiler
        .parse(Cursor::new(src))
        .expect("Failed to parse source");
    assert!(!has_errors);
    assert!(!compiler.attr(&mut program, &features));

    // let mut out = String::new();
    // let mut printer = ASTPrinter::new(&mut out, true);
    // program.accept(&mut printer, &mut 0);
    // println!("Evaluating with VM: {}", out);

    let file = compiler.ir(&mut program, &features);
    vm.run(&file).unwrap().expect("Expected result")
}
