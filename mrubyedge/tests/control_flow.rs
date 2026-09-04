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

#[test]
fn constant_reachable_from_an_instance_method_test() {
    // The constant belongs to the class's namespace; the method body reads
    // it unqualified, with self being an instance rather than the class.
    let code = "
    module Markup
      SAFE = ['div', 'span']

      class Element
        LIMIT = 2

        def safe_count
          SAFE.size + LIMIT
        end
      end
    end

    def test_main
      Markup::Element.new.safe_count
    end
    ";
    assert_eq!(run_i("const_from_instance_method", code), 4);
}

#[test]
fn constant_from_a_superclass_test() {
    let code = "
    class Base
      DEFAULT = 7

      def value
        DEFAULT
      end
    end

    class Child < Base
    end

    def test_main
      Child.new.value
    end
    ";
    assert_eq!(run_i("const_from_superclass", code), 7);
}

#[test]
fn constant_belongs_to_its_namespace_test() {
    let code = "
    module Outer
      module Inner
        VALUE = 3
      end
    end

    def test_main
      Outer::Inner::VALUE
    end
    ";
    assert_eq!(run_i("namespaced_const", code), 3);
}
