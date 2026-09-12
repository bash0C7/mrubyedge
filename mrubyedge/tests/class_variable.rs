extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn class_variable_read_and_write_test() {
    let code = "
class Counter
  def bump
    @@count = 41
  end

  def count
    @@count
  end
end

c = Counter.new
c.bump
c.count
    ";
    let binary = mrbc_compile("classvar", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 41);
}

#[test]
fn class_variable_is_shared_between_instances_test() {
    let code = "
class Shared
  def set(n)
    @@v = n
  end

  def get
    @@v
  end
end

Shared.new.set(7)
Shared.new.get
    ";
    let binary = mrbc_compile("classvar_shared", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 7);
}
