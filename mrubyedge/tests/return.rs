extern crate mrubyedge;

mod helpers;
use std::rc::Rc;

use helpers::*;
use mrubyedge::yamrb::value::RObject;

#[test]
fn return_test() {
    let code = "
def fib(n)
  return 1 if n <= 1
  fib(n - 1) + fib(n - 2)
end
    ";
    let binary = mrbc_compile("return_fib", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![Rc::new(RObject::integer(10))];
    let result = mrb_funcall(&mut vm, None, "fib", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 89);

    // Assert 2
    let args = vec![Rc::new(RObject::integer(1))];
    let result = mrb_funcall(&mut vm, None, "fib", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 1);
}

#[test]
fn return_test_toplevel_1() {
    let code = "
def fib(n)
  return 1 if n <= 1
  fib(n - 1) + fib(n - 2)
end

fib(10)
    ";
    let binary = mrbc_compile("return_fib_t1", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();

    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 89);
}

#[test]
fn return_test_toplevel_2() {
    let code = "
def fib(n)
  return 1 if n <= 1
  fib(n - 1) + fib(n - 2)
end

fib(1)
    ";
    let binary = mrbc_compile("return_fib_t2", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();

    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 1);
}

#[test]
fn return_from_inside_an_ensure_body_test() {
    let code = "
def m
  begin
    return 1
  ensure
    2
  end
end

m
    ";
    let binary = mrbc_compile("ret_ensure", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 1);
}

#[test]
fn return_from_inside_a_rescue_body_test() {
    let code = "
def m
  begin
    return 1
  rescue
    2
  end
end

m
    ";
    let binary = mrbc_compile("ret_rescue", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 1);
}

#[test]
fn return_from_inside_a_while_loop_test() {
    let code = "
def m
  while true
    return 1
  end
end

m
    ";
    let binary = mrbc_compile("ret_while", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 1);
}
