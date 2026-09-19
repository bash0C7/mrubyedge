extern crate mrubyedge;

mod helpers;
use helpers::*;

// Compiles and runs `code`, then calls `test_main` and converts the result
// to an i64.
fn run_test_main_i(name: &'static str, code: &'static str) -> i64 {
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

// Compiles and runs `code`, then calls `test_main` and converts the result
// to a bool.
fn run_test_main_b(name: &'static str, code: &'static str) -> bool {
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
    assert_eq!(run_test_main_s("define_method", code), "world");
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
    assert_eq!(run_test_main_s("define_method_closure", code), "span");
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
    assert_eq!(run_test_main_i("define_method_args", code), 15);
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
    assert_eq!(run_test_main_s("module_eval", code), "HI");
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
      helpers.instance_methods.include?(:answer)
    end
    ";
    assert!(run_test_main_b("module_new", code));
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
    assert!(run_test_main_b("instance_methods", code));
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
    assert!(run_test_main_b("method_defined", code));
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
    assert!(run_test_main_b("const_defined", code));
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
    assert_eq!(run_test_main_i("const_get", code), 5);
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
    assert_eq!(run_test_main_i("const_set", code), 6);
}
