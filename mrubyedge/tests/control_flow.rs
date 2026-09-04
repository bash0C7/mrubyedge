extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn break_ends_the_iteration_not_the_method_test() {
    let code = "
    def test_main
      [1, 2, 3].each do |x|
        break if x == 2
      end
      'after the loop'
    end
    ";
    assert_eq!(run_s("break_stays_local", code), "after the loop");
}

#[test]
fn break_value_is_the_value_of_the_call_test() {
    let code = "
    def test_main
      [1, 2, 3].each do |x|
        break x * 10 if x == 2
      end
    end
    ";
    assert_eq!(run_i("break_value", code), 20);
}

#[test]
fn break_across_a_call_test() {
    let code = "
    def scan
      [1, 2, 3].each do |x|
        break if x == 2
      end
      'scanned'
    end

    def test_main
      scan
      'caller survived'
    end
    ";
    assert_eq!(run_s("break_across_call", code), "caller survived");
}

#[test]
fn iteration_without_break_is_unchanged_test() {
    let code = "
    def test_main
      total = 0
      [1, 2, 3].each { |x| total += x }
      total
    end
    ";
    assert_eq!(run_i("no_break", code), 6);
}

#[test]
fn super_from_a_funcall_frame_test() {
    // Class#new reaches `initialize` through mrb_funcall, and `super` in it
    // has no callinfo to read.
    let code = "
    class Node
      attr_reader :type

      def initialize(type)
        @type = type
      end
    end

    class Element < Node
      attr_reader :tag

      def initialize(tag)
        super(:element)
        @tag = tag
      end
    end

    def test_main
      Element.new('div').type.to_s
    end
    ";
    assert_eq!(run_s("super_in_initialize", code), "element");
}
