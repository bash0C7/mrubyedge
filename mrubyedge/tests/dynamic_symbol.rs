extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn an_interpolated_symbol_equals_the_written_one_test() {
    let code = "
x = \"ab\"
if :\"k#{x}\" == :kab
  1
else
  0
end
";

    let binary = mrbc_compile("dynamic_symbol", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 1);
}

#[test]
fn an_interpolated_symbol_becomes_its_string_test() {
    let code = "
n = 7
:\"key#{n}\".to_s
";

    let binary = mrbc_compile("dynamic_symbol", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: String = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, "key7");
}

#[test]
fn an_interpolated_symbol_works_as_a_hash_key_test() {
    let code = "
h = {}
part = \"id\"
h[:\"user_#{part}\"] = 5
h[:user_id]
";

    let binary = mrbc_compile("dynamic_symbol", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 5);
}
