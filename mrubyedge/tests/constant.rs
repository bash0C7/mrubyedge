extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn a_constant_assigned_at_the_top_level_reads_back_test() {
    let code = "
FOO = 41
FOO + 1
";

    let binary = mrbc_compile("constant", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 42);
}

#[test]
fn a_constant_assigned_in_a_class_reads_back_test() {
    let code = "
class C
  BAR = 7
  def bar; BAR; end
end
C.new.bar
";

    let binary = mrbc_compile("constant", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 7);
}
