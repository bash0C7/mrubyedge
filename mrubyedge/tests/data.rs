extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

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
    assert_eq!(run_s("data_new", code), "1,2|1,2");
}

#[test]
fn data_compares_by_members_test() {
    let code = r##"
    Point = Data.define(:x, :y)
    Other = Data.define(:x, :y)
    def test_main
      same = Point.new(1, 2) == Point.new(1, 2)
      diff = Point.new(1, 2) == Point.new(1, 3)
      cross = Point.new(1, 2) == Other.new(1, 2)
      "#{same}|#{diff}|#{cross}"
    end
    "##;
    assert_eq!(run_s("data_eq", code), "true|false|false");
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
    assert_eq!(run_s("data_with", code), "1,9|2");
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
    assert_eq!(run_s("data_members", code), "[:x, :y]|1,2|#<data x=1, y=2>");
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
    assert_eq!(run_s("data_argerror", code), "ArgumentError");
}

#[test]
fn data_bracket_constructor_test() {
    let code = r##"
    Point = Data.define(:x, :y)
    def test_main
      (Point[1, 2] == Point.new(1, 2)).to_s
    end
    "##;
    assert_eq!(run_s("data_bracket_constructor", code), "true");
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
    assert_eq!(run_s("data_eql", code), "true|false");
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
    assert_eq!(run_s("data_to_s", code), "true");
}
