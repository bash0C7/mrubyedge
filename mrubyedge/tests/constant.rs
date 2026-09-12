extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn a_constant_assigned_at_the_top_level_reads_back_test() {
    let code = "
FOO = 41
FOO + 1
";

    let result = run_covering(code, &["SETCONST", "GETCONST"]);
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

    let result = run_covering(code, &["SETCONST"]);
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 7);
}
