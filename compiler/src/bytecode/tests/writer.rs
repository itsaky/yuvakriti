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

use std::fs::File;
use std::path::Path;

use crate::bytecode::attrs;
use crate::bytecode::bytes::ByteInput;
use crate::bytecode::cp_info::NumberInfo;
use crate::bytecode::cp_info::Utf8Info;
use crate::bytecode::opcode::OpCode;
use crate::bytecode::opcode::OpSize;
use crate::bytecode::tests::util::compile_to_bytecode;
use crate::bytecode::ConstantEntry;
use crate::bytecode::YKBFileReader;
use crate::features::CompilerFeatures;

#[test]
fn test_program_writer() {
    let path = Path::new("target/reader.ykb");
    let display = path.display();
    let ykbfile = compile_to_bytecode(
        &CompilerFeatures::default(),
        "fun main() { var str = \"str\"; var num = 123; }",
        &path,
    );

    let f = File::open(&path).unwrap();
    let readykb = match YKBFileReader::new(ByteInput::new(f)).read_file() {
        Ok(file) => file,
        Err(why) => {
            panic!("couldn't read from file {}: {}", display, why);
        }
    };

    assert_eq!(&readykb.version(), &ykbfile.version());
    assert_eq!(&readykb.constant_pool(), &ykbfile.constant_pool());
}

#[test]
fn test_arithemetic_constant_folding() {
    let path = Path::new("target/const_folding.ykb");

    #[rustfmt::skip]
    verify_top_level_insns(
        "print 1 + 2; print 2 - 3; print 3 * 4; print 4 / 5;",
        &path,
        &CompilerFeatures::default(),
        &vec![
            ConstantEntry::None, // constant at entry 0 is always None
            ConstantEntry::Number(NumberInfo::from(&3f64)), // 1+2 folded to 3
            ConstantEntry::Number(NumberInfo::from(&-1f64)), // 2-3 folded to -1
            ConstantEntry::Number(NumberInfo::from(&12f64)), // 3*4 folded to 12
            ConstantEntry::Number(NumberInfo::from(&0.8f64)), // 4/5 folded to 0.8
            ConstantEntry::Utf8(Utf8Info::from("Code")), // "Code" is the name of the "Code" attribute for the YKBFile's top-level statements
        ],
        &vec![
            OpCode::Ldc as OpSize, 0x00, 0x01, // 3
            OpCode::Print as OpSize,
            OpCode::Ldc as OpSize, 0x00, 0x02, // -1
            OpCode::Print as OpSize,
            OpCode::Ldc as OpSize, 0x00, 0x03, // 12
            OpCode::Print as OpSize,
            OpCode::Ldc as OpSize, 0x00, 0x04, // 0.8
            OpCode::Print as OpSize,
        ], 1, 0
    );
}

#[test]
fn test_disabled_arithemetic_constant_folding() {
    let path = Path::new("target/disabled_const_folding.ykb");
    let mut features = CompilerFeatures::default();
    features.const_folding = false;

    #[rustfmt::skip]
    verify_top_level_insns(
        "print 1 + 2; print 2 - 3; print 3 * 4; print 4 / 5;",
        &path,
        &features,
        &vec![
            ConstantEntry::None, // constant at entry 0 is always None
            ConstantEntry::Number(NumberInfo::from(&1f64)),
            ConstantEntry::Number(NumberInfo::from(&2f64)),
            ConstantEntry::Number(NumberInfo::from(&3f64)),
            ConstantEntry::Number(NumberInfo::from(&4f64)),
            ConstantEntry::Number(NumberInfo::from(&5f64)),
            ConstantEntry::Utf8(Utf8Info::from("Code")),
        ],
        &vec![
            OpCode::Ldc as OpSize, 0x00, 0x01, // 1
            OpCode::Ldc as OpSize, 0x00, 0x02, // 2
            OpCode::Add as OpSize,
            OpCode::Print as OpSize,
            OpCode::Ldc as OpSize, 0x00, 0x02, // 2
            OpCode::Ldc as OpSize, 0x00, 0x03, // 3
            OpCode::Sub as OpSize,
            OpCode::Print as OpSize,
            OpCode::Ldc as OpSize, 0x00, 0x03, // 3
            OpCode::Ldc as OpSize, 0x00, 0x04, // 4
            OpCode::Mult as OpSize,
            OpCode::Print as OpSize,
            OpCode::Ldc as OpSize, 0x00, 0x04, // 4
            OpCode::Ldc as OpSize, 0x00, 0x05, // 5
            OpCode::Div as OpSize,
            OpCode::Print as OpSize,
        ],2, 0
    );
}

#[test]
fn test_bpush_ops() {
    let path = Path::new("target/bpush_ops.ykb");
    verify_top_level_insns(
        "print false; print true;",
        &path,
        &CompilerFeatures::default(),
        &vec![],
        &vec![
            OpCode::BPush0 as OpSize,
            OpCode::Print as OpSize,
            OpCode::BPush1 as OpSize,
            OpCode::Print as OpSize,
        ],
        1,
        0,
    );
}

#[test]
fn verify_max_stack_size_attr() {
    let path = Path::new("target/max_stack_size.ykb");
    let mut features = CompilerFeatures::default();
    features.const_folding = false;

    for (source, stack_size) in [
        ("print 1;", 1),
        ("print 1 + 2;", 2),
        // stack size must be 2 here, reason :
        // push 1; stack=1
        // push 2; stack=1,2
        // add; stack=3
        // push 3; stack=3,3
        // add; stack=6
        // as seen above, max stack size is 2
        ("print 1 + 2 + 3;", 2),
        // this must be true, no matter how deep the binary expr is
        ("print 1 + 2 + 3 + 4 + 5 + 6;", 2),
    ] {
        let ykbfile = compile_to_bytecode(&features, source, &path);
        let attrs = ykbfile.attributes();
        let attr = attrs
            .iter()
            .find(|attr| attr.name() == attrs::CODE)
            .expect("Expected a Code attribute to be present");

        if let attrs::Attr::Code(code) = &attr {
            assert_eq!(stack_size, code.max_stack());
        }
    }
}

#[test]
fn test_max_stack_computation() {
    let path = Path::new("target/max_stack.ykb");
    let features = CompilerFeatures::default();
    let file = compile_to_bytecode(
        &features,
        "\
print 1 + 2;
print 2 * 3;
print \"hello\";
print \"world\";
print 2/3;
print 3-2;
print true;
print false;",
        &path,
    );

    let attrs = file.attributes();
    let code = attrs
        .iter()
        .find(|attr| attr.name() == attrs::CODE)
        .map(|attr| {
            if let attrs::Attr::Code(code) = attr {
                code
            } else {
                panic!("Expected a Code attribute to be present")
            }
        })
        .expect("Expected a Code attribute to be present");

    assert_eq!(1, code.max_stack());
}

#[test]
fn test_variable_decl_and_load() {
    let path = Path::new("target/variable_decl_and_load.ykb");
    let features = CompilerFeatures::default();
    verify_top_level_insns(
        "var i = 0; print i;",
        &path,
        &features,
        &vec![],
        &vec![
            OpCode::Ldc as OpSize,
            0x00,
            0x01, // const at idx 1
            OpCode::Store0 as OpSize,
            OpCode::Load0 as OpSize,
            OpCode::Print as OpSize,
        ],
        1,
        1,
    );
}

#[test]
fn test_variable_reassigning() {
    let path = Path::new("target/variable_reassigning.ykb");
    let features = CompilerFeatures::default();
    verify_top_level_insns(
        "var i = 0; i = i + 1; print i;",
        &path,
        &features,
        &vec![],
        &vec![
            OpCode::Ldc as OpSize,
            0x00,
            0x01, // const at idx 1
            OpCode::Store0 as OpSize,
            OpCode::Load0 as OpSize,
            OpCode::Ldc as OpSize,
            0x00,
            0x02, // const at idx 2
            OpCode::Add as OpSize,
            OpCode::Store0 as OpSize,
            OpCode::Load0 as OpSize,
            OpCode::Print as OpSize,
        ],
        2,
        1,
    );
}

#[test]
fn test_multi_variable_decl_and_load() {
    let path = Path::new("target/variable_decl_and_load.ykb");
    let features = CompilerFeatures::default();
    verify_top_level_insns(
        "var a = 0; var b = 1; var c = 2; print a; print b; print c; print a + b + c;",
        &path,
        &features,
        &vec![],
        &vec![
            OpCode::Ldc as OpSize,
            0x00,
            0x01, // const at idx 1
            OpCode::Store0 as OpSize,
            OpCode::Ldc as OpSize,
            0x00,
            0x02, // const at idx 2 (names for var-decls are not stored in constant pool)
            OpCode::Store1 as OpSize,
            OpCode::Ldc as OpSize,
            0x00,
            0x03, // const at idx 3
            OpCode::Store2 as OpSize,
            OpCode::Load0 as OpSize,
            OpCode::Print as OpSize,
            OpCode::Load1 as OpSize,
            OpCode::Print as OpSize,
            OpCode::Load2 as OpSize,
            OpCode::Print as OpSize,
            OpCode::Load0 as OpSize,
            OpCode::Load1 as OpSize,
            OpCode::Add as OpSize,
            OpCode::Load2 as OpSize,
            OpCode::Add as OpSize,
            OpCode::Print as OpSize,
        ],
        2,
        3,
    );
}

fn verify_top_level_insns(
    source: &str,
    out_path: &Path,
    features: &CompilerFeatures,
    exp_cps: &Vec<ConstantEntry>,
    exp_insns: &Vec<OpSize>,
    max_stack: u16,
    max_locals: u16,
) {
    let ykbfile = compile_to_bytecode(features, source, &out_path);

    if !exp_cps.is_empty() {
        assert_eq!(exp_cps, ykbfile.constant_pool().entries());
    }

    let attrs = ykbfile.attributes();
    let attr = attrs
        .iter()
        .find(|attr| attr.name() == attrs::CODE)
        .expect("Expected a Code attribute to be present");

    if let attrs::Attr::Code(code) = &attr {
        let insns = code.instructions();
        assert_eq!(max_stack, code.max_stack());
        assert_eq!(max_locals, code.max_locals());
        assert_eq!(exp_insns, insns);
    }
}
