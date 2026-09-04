extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn define_method_test() {
    let code = "
    class Greeter
      define_method(:hello) do
        'world'
      end
    end

    def test_main
      Greeter.new.hello
    end
    ";
    assert_eq!(run_s("define_method", code), "world");
}
#[test]
fn define_method_keeps_its_closure_test() {
    let code = "
    class Tags
      ['div', 'span'].each do |name|
        define_method(name) do
          name
        end
      end
    end

    def test_main
      Tags.new.span
    end
    ";
    assert_eq!(run_s("define_method_closure", code), "span");
}
#[test]
fn define_method_takes_arguments_test() {
    let code = "
    class Calc
      define_method(:add) do |a, b = 10|
        a + b
      end
    end

    def test_main
      Calc.new.add(5)
    end
    ";
    assert_eq!(run_i("define_method_args", code), 15);
}
#[test]
fn module_eval_defines_on_the_module_test() {
    let code = "
    module Helpers
    end

    Helpers.module_eval do
      define_method(:shout) do
        'HI'
      end
    end

    class Speaker
      include Helpers
    end

    def test_main
      Speaker.new.shout
    end
    ";
    assert_eq!(run_s("module_eval", code), "HI");
}
#[test]
fn module_new_test() {
    let code = "
    def test_main
      helpers = Module.new
      helpers.module_eval do
        define_method(:answer) do
          42
        end
      end
      helpers.instance_methods.include?(:answer) ? 1 : 0
    end
    ";
    assert_eq!(run_i("module_new", code), 1);
}
#[test]
fn instance_methods_includes_a_defined_method_test() {
    let code = "
    class Widget
      def draw
      end
    end

    def test_main
      Widget.instance_methods.include?(:draw)
    end
    ";
    assert!(run_b("instance_methods", code));
}
#[test]
fn method_defined_reports_a_defined_method_test() {
    let code = "
    class Widget
      def draw
      end
    end

    def test_main
      Widget.method_defined?(:draw)
    end
    ";
    assert!(run_b("method_defined", code));
}
#[test]
fn const_defined_reports_a_defined_constant_test() {
    let code = "
    module Config
      LIMIT = 5
    end

    def test_main
      Config.const_defined?(:LIMIT)
    end
    ";
    assert!(run_b("const_defined", code));
}
#[test]
fn const_get_reads_the_constant_test() {
    let code = "
    module Config
      LIMIT = 5
    end

    def test_main
      Config.const_get(:LIMIT)
    end
    ";
    assert_eq!(run_i("const_get", code), 5);
}
#[test]
fn const_set_defines_a_new_constant_test() {
    let code = "
    module Config
      LIMIT = 5
    end

    def test_main
      Config.const_set(:OTHER, Config.const_get(:LIMIT) + 1)
      Config.const_get(:OTHER)
    end
    ";
    assert_eq!(run_i("const_set", code), 6);
}

#[test]
fn instance_variable_defined_after_it_is_set_test() {
    let code = "
    class Box
      def initialize
        @value = 7
      end
    end

    def test_main
      Box.new.instance_variable_defined?(:@value)
    end
    ";
    assert!(run_b("instance_variable_defined", code));
}

#[test]
fn instance_variables_lists_the_set_ivar_test() {
    let code = "
    class Box
      def initialize
        @value = 7
      end
    end

    def test_main
      Box.new.instance_variables.include?(:@value)
    end
    ";
    assert!(run_b("instance_variables_include", code));
}

#[test]
fn instance_variable_get_reads_the_value_test() {
    let code = "
    class Box
      def initialize
        @value = 7
      end
    end

    def test_main
      Box.new.instance_variable_get(:@value)
    end
    ";
    assert_eq!(run_i("instance_variable_get", code), 7);
}

#[test]
fn instance_variable_set_updates_the_value_test() {
    let code = "
    class Box
      def initialize
        @value = 7
      end
    end

    def test_main
      box = Box.new
      box.instance_variable_set(:@value, box.instance_variable_get(:@value) + 1)
      box.instance_variable_get(:@value)
    end
    ";
    assert_eq!(run_i("instance_variable_set", code), 8);
}

#[test]
fn send_test() {
    let code = "
    class Adder
      def add(a, b)
        a + b
      end
    end

    def test_main
      Adder.new.send(:add, 3, 4)
    end
    ";
    assert_eq!(run_i("send", code), 7);
}
