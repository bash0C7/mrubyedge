//
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn run_returns_the_last_value_test() {
    let result = run("1 + 2");
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 3);
}

#[test]
fn try_run_hands_back_the_failure_test() {
    let result = try_run("nope_no_such_method");

    // Assert
    assert!(result.is_err(), "失敗するはずが {result:?} を返した");
}

#[test]
fn opcode_names_covers_the_whole_table_test() {
    let names = opcode_names();
    let mut sorted = names.clone();
    sorted.sort();
    sorted.dedup();

    // Assert
    assert_eq!(names.len(), 119, "mruby 4.0のopcodeは119件");
    assert_eq!(sorted.len(), names.len(), "名前が重複している");
    for added in [
        "GETIDX0", "MATCHERR", "SSEND0", "SEND0", "BLKCALL", "RETSELF", "RETNIL", "RETTRUE",
        "RETFALSE", "ADDILV", "SUBILV", "TDEF", "SDEF",
    ] {
        assert!(names.contains(&added.to_string()), "{added} が表に無い");
    }
}

#[test]
fn run_covering_passes_an_opcode_the_code_really_runs_test() {
    let result = run_covering("a = [5, 6]\na[0]", &["GETIDX0"]);
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 5);
}

#[test]
#[should_panic(expected = "opcodeではない")]
fn run_covering_rejects_a_misspelled_opcode_test() {
    run_covering("1", &["GETIDXZERO"]);
}

#[test]
#[should_panic(expected = "踏むopcodeを1つ以上")]
fn run_covering_rejects_an_empty_list_test() {
    run_covering("1", &[]);
}

#[cfg(feature = "opcode-coverage")]
#[test]
#[should_panic(expected = "を踏んでいない")]
fn run_covering_fails_when_the_opcode_is_not_reached_test() {
    run_covering("1 + 2", &["GETIDX0"]);
}

#[cfg(feature = "opcode-coverage")]
#[test]
fn local_seen_is_reset_for_each_run_test() {
    run("a = [5, 6]\na[0]");
    let after_getidx0 = mrubyedge::yamrb::coverage::local_seen();
    run("1 + 2");
    let after_add = mrubyedge::yamrb::coverage::local_seen();

    // Assert
    assert!(after_getidx0.contains(&"GETIDX0".to_string()));
    assert!(
        !after_add.contains(&"GETIDX0".to_string()),
        "前の実行の記録が残っている: {after_add:?}"
    );
}

#[cfg(feature = "opcode-coverage")]
#[test]
fn local_seen_does_not_pick_up_another_thread_test() {
    let other = std::thread::spawn(|| {
        run("a = [5, 6]\na[0]");
        mrubyedge::yamrb::coverage::local_seen()
    });
    run("1 + 2");
    let mine = mrubyedge::yamrb::coverage::local_seen();
    let theirs = other.join().unwrap();

    // Assert
    assert!(
        theirs.contains(&"GETIDX0".to_string()),
        "別スレッド側が踏んでいない"
    );
    assert!(
        !mine.contains(&"GETIDX0".to_string()),
        "別スレッドの記録が混ざっている: {mine:?}"
    );
}
