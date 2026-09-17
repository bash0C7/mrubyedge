extern crate mrubyedge;

mod helpers;
use helpers::*;

// Compiles and runs `code`, then calls `test_main` and converts the result
// to a String.
fn run_test_main_s(name: &'static str, code: &'static str) -> String {
    let binary = mrbc_compile(name, code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();
    mrb_funcall(&mut vm, None, "test_main", &[])
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap()
}

#[test]
fn data_define_positional_and_keyword_test() {
    let code = r##"
    Point = Data.define(:x, :y)
    def test_main
      a = Point.new(1, 2)
      b = Point.new(x: 1, y: 2)
      "#{a.x},#{a.y}|#{b.x},#{b.y}"
    end
    "##;
    assert_eq!(run_test_main_s("data_new", code), "1,2|1,2");
}

#[test]
fn data_compares_by_members_test() {
    // `.eql?` (a dot-call) reaches Data#eql?/#== directly. The `==`
    // OPERATOR compiles to OP_EQ, which does not yet dispatch to a
    // user-defined == for an instance -- a separate, already-completed
    // fix in this decomposition series, not part of this PR.
    let code = r##"
    Point = Data.define(:x, :y)
    Other = Data.define(:x, :y)
    def test_main
      same = Point.new(1, 2).eql?(Point.new(1, 2))
      diff = Point.new(1, 2).eql?(Point.new(1, 3))
      cross = Point.new(1, 2).eql?(Other.new(1, 2))
      "#{same}|#{diff}|#{cross}"
    end
    "##;
    assert_eq!(run_test_main_s("data_eq", code), "true|false|false");
}

#[test]
fn data_with_copies_and_leaves_the_original_test() {
    let code = r##"
    Point = Data.define(:x, :y)
    def test_main
      a = Point.new(1, 2)
      b = a.with(y: 9)
      "#{b.x},#{b.y}|#{a.y}"
    end
    "##;
    assert_eq!(run_test_main_s("data_with", code), "1,9|2");
}

#[test]
fn data_members_and_to_h_and_inspect_test() {
    let code = r##"
    Point = Data.define(:x, :y)
    def test_main
      a = Point.new(1, 2)
      "#{Point.members}|#{a.to_h[:x]},#{a.to_h[:y]}|#{a.inspect}"
    end
    "##;
    assert_eq!(
        run_test_main_s("data_members", code),
        "[:x, :y]|1,2|#<data x=1, y=2>"
    );
}

#[test]
fn data_rejects_too_many_arguments_test() {
    let code = r##"
    Point = Data.define(:x, :y)
    def test_main
      begin
        Point.new(1, 2, 3)
        "no error"
      rescue ArgumentError => e
        "ArgumentError"
      end
    end
    "##;
    assert_eq!(run_test_main_s("data_argerror", code), "ArgumentError");
}

#[test]
fn data_bracket_constructor_test() {
    let code = r##"
    Point = Data.define(:x, :y)
    def test_main
      Point[1, 2].eql?(Point.new(1, 2)).to_s
    end
    "##;
    assert_eq!(run_test_main_s("data_bracket_constructor", code), "true");
}

#[test]
fn data_eql_test() {
    let code = r##"
    Point = Data.define(:x, :y)
    def test_main
      same = Point.new(1, 2).eql?(Point.new(1, 2))
      diff = Point.new(1, 2).eql?(Point.new(1, 3))
      "#{same}|#{diff}"
    end
    "##;
    assert_eq!(run_test_main_s("data_eql", code), "true|false");
}

#[test]
fn data_to_s_test() {
    let code = r##"
    Point = Data.define(:x, :y)
    def test_main
      a = Point.new(1, 2)
      (a.to_s == a.inspect).to_s
    end
    "##;
    assert_eq!(run_test_main_s("data_to_s", code), "true");
}

#[test]
fn data_bracket_constructor_ignores_callers_rest_kwargs_test() {
    // `One[7]` (a single positional argument) compiles to OP_GETIDX, which
    // reaches mrb_data_new through mrb_funcall without a fresh kwarg frame.
    // A `**rest` parameter in the calling method leaves the caller's own
    // keyword frame live for the whole method body, so a naive
    // `vm.get_kwargs()` inside `mrb_data_new` would see the caller's `v: 99`
    // and mistake it for this call's keywords. The positional argument must
    // win.
    let code = r##"
    One = Data.define(:v)
    def build(**opts)
      One[7].v.to_s
    end
    def test_main
      build(v: 99)
    end
    "##;
    assert_eq!(run_test_main_s("data_bracket_rest_kwargs", code), "7");
}

#[test]
fn data_subclass_inherits_members_test() {
    let code = r##"
    Point = Data.define(:x, :y)
    class Point3 < Point
    end
    def test_main
      a = Point3.new(1, 2)
      "#{Point3.members}|#{a.x},#{a.y}"
    end
    "##;
    assert_eq!(
        run_test_main_s("data_subclass_members", code),
        "[:x, :y]|1,2"
    );
}
