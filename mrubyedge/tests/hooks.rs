// `Module#included` and `Class#inherited`.
//
// Both fire while the VM is in the middle of a class or module definition,
// so they also stand as the check that a hook body cannot walk over the
// registers the definition is still using.
extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn included_fires_and_can_extend_the_including_class() {
    // The ClassMethods idiom: the hook is the only thing that puts `cm` on
    // the class, so reaching it proves the hook ran.
    let code = "
    module M
      def self.included(base)
        base.extend(ClassMethods)
      end
      module ClassMethods
        def cm
          'from ClassMethods'
        end
      end
    end

    class A
      include M
    end

    A.cm
    ";
    assert_eq!(run("included_hook", code), "from ClassMethods");
}

#[test]
fn what_included_puts_on_a_class_reaches_its_subclasses() {
    let code = "
    module M
      def self.included(base)
        base.extend(ClassMethods)
      end
      module ClassMethods
        def cm
          'ok'
        end
      end
    end

    class A
      include M
    end
    class B < A
    end

    B.cm
    ";
    assert_eq!(run("included_hook_inherited", code), "ok");
}

#[test]
fn including_a_module_without_the_hook_is_still_fine() {
    let code = "
    module M
      def hello
        'hi'
      end
    end
    class A
      include M
    end
    A.new.hello
    ";
    assert_eq!(run("included_hook_absent", code), "hi");
}

#[test]
fn inherited_fires_with_the_new_subclass() {
    let code = "
    class Base
      def self.registry
        @registry ||= []
      end
      def self.inherited(subclass)
        Base.registry << subclass
      end
    end

    class One < Base; end
    class Two < Base; end

    Base.registry.map { |k| k.to_s }.join(',')
    ";
    assert_eq!(run("inherited_hook", code), "One,Two");
}

#[test]
fn the_class_body_still_runs_after_the_hook() {
    // The hook is called with the new class sitting in a register OP_EXEC is
    // about to read; a body that clobbered it would take the class body with
    // it.
    let code = "
    class Base
      def self.inherited(subclass)
        # deliberately noisy: several calls, a block, an array
        [1, 2, 3].map { |i| i * 2 }.reduce(0) { |a, b| a + b }
      end
    end

    class Sub < Base
      def value
        'body ran'
      end
    end

    Sub.new.value
    ";
    assert_eq!(run("inherited_hook_body", code), "body ran");
}

#[test]
fn reopening_a_class_does_not_fire_inherited_again() {
    let code = "
    class Base
      def self.count
        @count ||= 0
      end
      def self.inherited(subclass)
        @count = count + 1
      end
    end

    class Sub < Base; end
    class Sub
      def extra; end
    end

    Base.count.to_s
    ";
    assert_eq!(run("inherited_hook_reopen", code), "1");
}
