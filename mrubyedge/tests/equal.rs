extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;
use mrubyedge::yamrb::value::RObject;

#[test]
fn equal_test() {
    let code = "
    def check_eq_1(a, b)
      3 == a + b
    end

    def check_eq_2(a, b)
      \"foobar\" == a + b
    end

    def check_eq_3(a, b)
      [:foo, :bar] == [a, b]
    end

    def check_eq_4(a, b, c, d)
      ha = {}
      ha[a] = b
      ha[c] = d

      {foo: 1, bar: \"str\"} == ha
    end
    ";
    let binary = mrbc_compile("eq", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![
        RObject::integer(1).to_refcount_assigned(),
        RObject::integer(2).to_refcount_assigned(),
    ];
    let result: bool = mrb_funcall(&mut vm, None, "check_eq_1", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert!(result);

    let args = vec![
        RObject::string("foo".into()).to_refcount_assigned(),
        RObject::string("bar".into()).to_refcount_assigned(),
    ];
    let result: bool = mrb_funcall(&mut vm, None, "check_eq_2", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert!(result);

    let args = vec![
        RObject::symbol("foo".into()).to_refcount_assigned(),
        RObject::symbol("bar".into()).to_refcount_assigned(),
    ];
    let result: bool = mrb_funcall(&mut vm, None, "check_eq_3", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert!(result);

    let args = vec![
        RObject::symbol("foo".into()).to_refcount_assigned(),
        RObject::integer(1).to_refcount_assigned(),
        RObject::symbol("bar".into()).to_refcount_assigned(),
        RObject::string("str".into()).to_refcount_assigned(),
    ];
    let result: bool = mrb_funcall(&mut vm, None, "check_eq_4", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert!(result);
}

// OP_EQ compared identity and never asked the object, so a class that
// defines its own == had it quietly ignored.

#[test]
fn user_defined_equals_is_asked_test() {
    let code = r##"
    class Point
      attr_reader :x, :y
      def initialize(x, y)
        @x = x
        @y = y
      end
      def ==(other)
        return false unless other.is_a?(Point)
        @x == other.x && @y == other.y
      end
    end

    def test_main
      same = Point.new(1, 2) == Point.new(1, 2)
      diff = Point.new(1, 2) == Point.new(1, 3)
      other = Point.new(1, 2) == "not a point"
      "#{same}|#{diff}|#{other}"
    end
    "##;
    // Assert
    assert_eq!(run_s("user_defined_equals", code), "true|false|false");
}

#[test]
fn dispatching_equals_leaves_the_caller_registers_alone_test() {
    // The callee needs a register window of its own; sharing the caller's
    // frame overwrites whatever the surrounding expression was holding.
    let code = r##"
    class Point
      attr_reader :x
      def initialize(x)
        @x = x
      end
      def ==(other)
        @x == other.x
      end
    end

    def test_main
      label = "kept"
      flag = Point.new(1) == Point.new(1)
      "#{label}:#{flag}"
    end
    "##;
    // Assert
    assert_eq!(run_s("equals_registers", code), "kept:true");
}

#[test]
fn an_object_without_its_own_equals_still_compares_by_identity_test() {
    let code = r##"
    class Plain
    end

    def test_main
      a = Plain.new
      "#{a == a}|#{a == Plain.new}"
    end
    "##;
    // Assert
    assert_eq!(run_s("plain_equals", code), "true|false");
}

// `!=` is `!(self == other)`, as mruby's BasicObject#!= defines it, so a
// user-defined == decides != too.

#[test]
fn not_eq_honors_a_user_defined_double_eq_that_says_true_test() {
    let code = r##"
    class AlwaysEqual
      def ==(other)
        true
      end
    end

    def test_main
      "#{AlwaysEqual.new != AlwaysEqual.new}"
    end
    "##;
    // Assert
    assert_eq!(run_s("not_eq_always_equal", code), "false");
}

#[test]
fn not_eq_honors_a_user_defined_double_eq_that_says_false_test() {
    let code = r##"
    class NeverEqual
      def ==(other)
        false
      end
    end

    def test_main
      a = NeverEqual.new
      "#{a != a}"
    end
    "##;
    // Assert
    assert_eq!(run_s("not_eq_never_equal", code), "true");
}

#[test]
fn not_eq_without_a_user_defined_double_eq_still_compares_by_identity_test() {
    let code = r##"
    class Plain
    end

    def test_main
      a = Plain.new
      "#{a != a}|#{a != Plain.new}"
    end
    "##;
    // Assert
    assert_eq!(run_s("plain_not_equals", code), "false|true");
}

#[test]
fn not_eq_on_a_data_class_compares_by_members_test() {
    let code = r##"
    Point = Data.define(:x, :y)

    def test_main
      "#{Point.new(1, 2) != Point.new(1, 2)}|#{Point.new(1, 2) != Point.new(1, 3)}"
    end
    "##;
    // Assert
    assert_eq!(run_s("not_eq_data", code), "false|true");
}
