extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn a_constant_set_under_a_class_reads_back_test() {
    let code = "
class Holder
end
Holder::LIMIT = 12
Holder::LIMIT
";

    let binary = mrbc_compile("scoped_constant", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 12);
}

#[test]
fn a_constant_set_under_a_module_reads_back_test() {
    let code = "
module Space
end
Space::NAME = \"outer\"
Space::NAME
";

    let binary = mrbc_compile("scoped_constant", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: String = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, "outer");
}

#[test]
fn a_constant_set_on_a_nested_module_reads_back_test() {
    let code = "
module Outer
  module Inner
  end
end
Outer::Inner::DEPTH = 2
Outer::Inner::DEPTH
";

    let binary = mrbc_compile("scoped_constant", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 2);
}
