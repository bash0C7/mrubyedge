extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn a_double_splat_merges_the_other_hash_test() {
    let code = "
h = {a: 1}
g = {**h, b: 2}
g[:a] + g[:b]
";

    let binary = mrbc_compile("hash_splat", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 3);
}

#[test]
fn the_pairs_after_a_double_splat_are_added_test() {
    let code = "
h = {a: 1}
g = {**h, b: 2, c: 3}
g.size
";

    let binary = mrbc_compile("hash_splat", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 3);
}

#[test]
fn a_later_pair_wins_over_the_splatted_one_test() {
    let code = "
h = {a: 1, b: 2}
g = {**h, b: 9}
g[:b]
";

    let binary = mrbc_compile("hash_splat", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 9);
}

#[test]
fn two_double_splats_merge_in_order_test() {
    let code = "
h = {a: 1}
i = {b: 2}
g = {**h, **i}
g[:a] + g[:b]
";

    let binary = mrbc_compile("hash_splat", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 3);
}
