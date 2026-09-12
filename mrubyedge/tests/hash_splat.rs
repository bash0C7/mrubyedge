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

    let result = run_covering(code, &["HASHCAT"]);
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

    let result = run_covering(code, &["HASHADD"]);
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

    let result = run_covering(code, &["HASHADD"]);
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

    let result = run_covering(code, &["HASHCAT"]);
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 3);
}
