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

// A constant assigned inside a class or module body was written to the
// global table, where only an unqualified read could reach it, and was not
// visible from that class's own instance methods.

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
    assert_eq!(run_test_main_i("const_from_instance_method", code), 4);
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
    assert_eq!(run_test_main_i("const_from_superclass", code), 7);
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
    assert_eq!(run_test_main_i("namespaced_const", code), 3);
}

#[test]
fn constant_from_a_superclass_reachable_from_a_subclass_body_test() {
    // self is the Child class itself while its body runs, so the ancestor
    // walk must start from a Class self too, not only an Instance self.
    let code = "
    class Base
      DEFAULT = 7
    end

    class Child < Base
      VALUE = DEFAULT + 1
    end

    def test_main
      Child::VALUE
    end
    ";
    assert_eq!(
        run_test_main_i("const_from_superclass_in_subclass_body", code),
        8
    );
}

#[test]
fn constant_from_a_superclass_reachable_from_a_class_method_test() {
    // self is the Child class itself while `def self.make` runs.
    let code = "
    class Base
      DEFAULT = 7
    end

    class Child < Base
      def self.make
        DEFAULT
      end
    end

    def test_main
      Child.make
    end
    ";
    assert_eq!(
        run_test_main_i("const_from_superclass_in_class_method", code),
        7
    );
}
