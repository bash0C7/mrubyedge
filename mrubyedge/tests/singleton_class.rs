extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn test_singleton_class() {
    let code = "
    obj = Object.new
    def obj.my_singleton_method
      123
    end

    obj.my_singleton_method
    ";
    let binary = mrbc_compile("singleton_class", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i32 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 123);
}

#[test]
fn class_self_opens_the_singleton_class_test() {
    let code = "
class C
  class << self
    def built; 11; end
  end
end
C.built
";

    let binary = mrbc_compile("singleton_class", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 11);
}
