extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn a_bare_super_forwards_the_argument_test() {
    let code = "
class Base
  def f(x); x * 2; end
end
class Sub < Base
  def f(x); super; end
end
Sub.new.f(4)
";

    let result = run_covering(code, &["ARGARY"]);
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 8);
}

#[test]
fn a_bare_super_forwards_every_argument_test() {
    let code = "
class Base2
  def g(x, y); x - y; end
end
class Sub2 < Base2
  def g(x, y); super; end
end
Sub2.new.g(9, 4)
";

    let result = run_covering(code, &["ARGARY"]);
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 5);
}

#[test]
fn a_bare_super_forwards_a_rest_argument_test() {
    let code = "
class Base3
  def h(*xs); xs.join(\",\"); end
end
class Sub3 < Base3
  def h(*xs); super; end
end
Sub3.new.h(1, 2, 3)
";

    let result = run_covering(code, &["ARGARY"]);
    let result: String = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, "1,2,3");
}

#[test]
fn a_super_with_arguments_written_out_still_works_test() {
    let code = "
class Base4
  def k(x); x + 1; end
end
class Sub4 < Base4
  def k(x); super(x * 10); end
end
Sub4.new.k(2)
";

    let result = run(code);
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 21);
}
