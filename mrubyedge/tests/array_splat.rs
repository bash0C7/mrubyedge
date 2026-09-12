extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn a_splat_followed_by_more_elements_pushes_them_test() {
    let code = "
a = [1, 2]
b = [*a, 3]
b.join(\",\")
";

    let binary = mrbc_compile("array_splat", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: String = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, "1,2,3");
}

#[test]
fn a_splat_in_the_middle_keeps_the_order_test() {
    let code = "
a = [2, 3]
b = [1, *a, 4, 5]
b.join(\",\")
";

    let binary = mrbc_compile("array_splat", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: String = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, "1,2,3,4,5");
}

#[test]
fn returning_a_splat_returns_the_array_test() {
    let code = "
def spread
  a = [1, 2]
  return *a
end
spread.join(\",\")
";

    let binary = mrbc_compile("array_splat", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: String = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, "1,2");
}

#[test]
fn returning_a_splat_of_one_value_wraps_it_test() {
    let code = "
def spread_one
  n = 5
  return *n
end
spread_one.join(\",\")
";

    let binary = mrbc_compile("array_splat", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: String = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, "5");
}
