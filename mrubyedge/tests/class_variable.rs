extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn a_class_variable_is_shared_by_the_instances_test() {
    let code = "
class Counter
  @@count = 0
  def self.count; @@count; end
  def bump; @@count = @@count + 1; end
end
Counter.new.bump
Counter.new.bump
Counter.count
";

    let binary = mrbc_compile("class_variable", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 2);
}

#[test]
fn a_subclass_sees_the_class_variable_of_its_parent_test() {
    let code = "
class Base
  @@shared = 5
  def shared; @@shared; end
end
class Sub < Base
end
Sub.new.shared
";

    let binary = mrbc_compile("class_variable", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 5);
}

#[test]
fn a_subclass_writes_through_to_its_parent_test() {
    let code = "
class Base2
  @@n = 1
  def self.n; @@n; end
end
class Sub2 < Base2
  def self.set; @@n = 9; end
end
Sub2.set
Base2.n
";

    let binary = mrbc_compile("class_variable", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 9);
}

#[test]
fn an_unset_class_variable_raises_test() {
    let code = "
class Empty
  def self.missing; @@nope; end
end
begin
  Empty.missing
  \"no error\"
rescue NameError => e
  \"raised\"
end
";

    let binary = mrbc_compile("class_variable", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: String = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, "raised");
}
