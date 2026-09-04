#![allow(unused_imports)]
#![allow(dead_code)]
use std::rc::Rc;

use mrubyedge::yamrb::value::RObject;

pub use mrubyedge::yamrb::helpers::mrb_funcall;

pub(crate) fn mrbc_compile(_fname: &'static str, code: &'static str) -> Vec<u8> {
    unsafe {
        let mut context = mruby_compiler2_sys::MRubyCompiler2Context::new();
        context.compile(code).unwrap()
    }
}

pub(crate) fn mrbc_compile_debug(_fname: &'static str, code: &'static str) -> Vec<u8> {
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

// Compiles and runs `code`, returning the top-level result.
pub(crate) fn run(name: &'static str, code: &'static str) -> String {
    let binary = mrbc_compile(name, code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    result.as_ref().try_into().unwrap()
}

// Compiles and runs `code`, then calls `test_main`.
pub(crate) fn run_i(name: &'static str, code: &'static str) -> i64 {
    let binary = mrbc_compile(name, code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();
    mrb_funcall(&mut vm, None, "test_main", &[])
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap()
}

pub(crate) fn run_s(name: &'static str, code: &'static str) -> String {
    let binary = mrbc_compile(name, code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();
    mrb_funcall(&mut vm, None, "test_main", &[])
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap()
}

pub(crate) fn run_b(name: &'static str, code: &'static str) -> bool {
    let binary = mrbc_compile(name, code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();
    let result = mrb_funcall(&mut vm, None, "test_main", &[]).unwrap();
    matches!(
        result.as_ref().value,
        mrubyedge::yamrb::value::RValue::Bool(true)
    )
}
