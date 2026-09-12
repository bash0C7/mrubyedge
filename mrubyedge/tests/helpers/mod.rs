#![allow(unused_imports)]
#![allow(dead_code)]
use std::rc::Rc;

use mrubyedge::yamrb::value::RObject;

pub use mrubyedge::yamrb::helpers::mrb_funcall;

pub(crate) fn mrbc_compile(_fname: &str, code: &str) -> Vec<u8> {
    unsafe {
        let mut context = mruby_compiler2_sys::MRubyCompiler2Context::new();
        context.compile(code).unwrap()
    }
}

pub(crate) fn mrbc_compile_debug(_fname: &str, code: &str) -> Vec<u8> {
    unsafe {
        let mut context = mruby_compiler2_sys::MRubyCompiler2Context::new();
        context.dump_bytecode(code).unwrap();
        context.compile(code).unwrap()
    }
}

pub(crate) fn int(n: i64) -> Rc<RObject> {
    Rc::new(RObject::integer(n))
}

pub(crate) fn string(s: &str) -> Rc<RObject> {
    Rc::new(RObject::string(s.to_string()))
}

//

pub(crate) fn run(code: &str) -> Rc<RObject> {
    try_run(code).unwrap_or_else(|e| panic!("実行が失敗した: {e:?}\n--- source ---\n{code}"))
}

pub(crate) fn try_run(code: &str) -> Result<Rc<RObject>, String> {
    let binary = mrbc_compile("compiled", code);
    #[cfg(feature = "opcode-coverage")]
    mrubyedge::yamrb::coverage::reset_local();
    let mut rite = mrubyedge::rite::load(&binary).map_err(|e| format!("{e:?}"))?;
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().map_err(|e| format!("{e}"))
}

///
pub(crate) fn run_covering(code: &str, opcodes: &[&str]) -> Rc<RObject> {
    assert!(!opcodes.is_empty(), "踏むopcodeを1つ以上挙げること");
    for name in opcodes {
        assert!(
            opcode_names().iter().any(|n| n == name),
            "{name} はmruby 4.0のopcodeではない(綴りの間違い)"
        );
    }
    let value = run(code);
    #[cfg(feature = "opcode-coverage")]
    {
        let seen = mrubyedge::yamrb::coverage::local_seen();
        let missed: Vec<&&str> = opcodes
            .iter()
            .filter(|n| !seen.contains(&n.to_string()))
            .collect();
        assert!(
            missed.is_empty(),
            "{missed:?} を踏んでいない。踏んだのは {seen:?}\n--- source ---\n{code}"
        );
    }
    value
}

pub(crate) fn opcode_names() -> Vec<String> {
    use mrubyedge::rite::insn::OpCode;
    (0..OpCode::NumberOfOpcode as usize)
        .filter_map(|wire| OpCode::try_from(wire as u8).ok())
        .map(|code| format!("{code:?}"))
        .collect()
}
