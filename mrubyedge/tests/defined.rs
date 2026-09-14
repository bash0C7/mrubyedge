// The `defined?` helpers mruby 4.0 compiles `defined?(...)` into.
//
// The dev-dependency `mruby-compiler2-sys` is the prism-based mruby 4.0
// compiler; `vendor/mruby-compiler2/include/mrc_presym.inc` defines all
// nine `__defined_*?` symbols it can emit a call to. These tests write
// plain `defined?(...)` Ruby and let that real compiler produce the
// bytecode, so they exercise the actual calling convention rather than an
// assumed one.
extern crate mrubyedge;

mod helpers;
use helpers::*;

// Compiles and runs `code`, returning the top-level result as a String.
fn run_top_level_s(name: &'static str, code: &'static str) -> String {
    let binary = mrbc_compile(name, code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    result.as_ref().try_into().unwrap()
}

#[test]
fn a_defined_constant_answers_constant() {
    let code = "
    class Known; end
    [defined?(Known).inspect, defined?(Unknown).inspect].join(',')
    ";
    assert_eq!(run_top_level_s("defined_const", code), "\"constant\",nil");
}

#[test]
fn a_constant_defined_in_an_enclosing_module_is_found() {
    // def self.x inside a module panics in this VM at this base commit
    // (an unrelated, pre-existing limitation), so the enclosing-namespace
    // walk is exercised from an instance method of a nested class instead.
    let code = "
    module Outer
      INSIDE = 1
      class Inner
        def check
          defined?(INSIDE).inspect
        end
      end
    end
    Outer::Inner.new.check
    ";
    assert_eq!(
        run_top_level_s("defined_const_nested", code),
        "\"constant\""
    );
}

#[test]
fn a_scoped_constant_answers_through_its_owner() {
    let code = "
    module Outer
      class Inner; end
    end
    [defined?(Outer::Inner).inspect, defined?(Outer::Missing).inspect].join(',')
    ";
    assert_eq!(
        run_top_level_s("defined_const_path", code),
        "\"constant\",nil"
    );
}

#[test]
fn a_deeper_const_path_is_walked_segment_by_segment() {
    let code = "
    module Outer
      class Inner
        X = 1
      end
    end
    [defined?(Outer::Inner::X).inspect, defined?(Outer::Inner::Y).inspect].join(',')
    ";
    assert_eq!(
        run_top_level_s("defined_const_path_deep", code),
        "\"constant\",nil"
    );
}

#[test]
fn a_top_level_rooted_constant_is_found() {
    let code = "
    Known = 1
    [defined?(::Known).inspect, defined?(::Unknown).inspect].join(',')
    ";
    assert_eq!(
        run_top_level_s("defined_const_path_root", code),
        "\"constant\",nil"
    );
}

#[test]
fn a_path_from_an_evaluated_root_is_walked() {
    // The third root kind: `expr::NAME` hands the helper the evaluated
    // receiver, rather than nil for a lexical path or the Object class for
    // a `::`-rooted one.
    let code = "
    module Outer
      class Inner; end
    end
    o = Outer
    [defined?(o::Inner).inspect, defined?(o::Nope).inspect].join(',')
    ";
    assert_eq!(
        run_top_level_s("defined_const_path_expr_root", code),
        "\"constant\",nil"
    );
}

#[test]
fn a_method_on_self_answers_method() {
    let code = "
    class Thing
      def known; end
      def check
        [defined?(known).inspect, defined?(missing).inspect].join(',')
      end
    end
    Thing.new.check
    ";
    assert_eq!(run_top_level_s("defined_method", code), "\"method\",nil");
}

#[test]
fn a_method_on_an_explicit_receiver_answers_method() {
    // Only the "method" half constrains the helper. The compiler puts the
    // call to __defined_method_on? inside a rescue that discards what it
    // raises, so the nil half would pass even with no helper registered.
    let code = "
    class Thing
      def known; end
    end
    obj = Thing.new
    [defined?(obj.known).inspect, defined?(obj.missing).inspect].join(',')
    ";
    assert_eq!(run_top_level_s("defined_method_on", code), "\"method\",nil");
}

#[test]
fn an_instance_variable_answers_only_once_it_is_set() {
    let code = "
    class Thing
      def check
        before = defined?(@v)
        @v = 1
        after = defined?(@v)
        [before.inspect, after.inspect].join(',')
      end
    end
    Thing.new.check
    ";
    assert_eq!(
        run_top_level_s("defined_ivar", code),
        "nil,\"instance-variable\""
    );
}

#[test]
fn a_global_answers_only_once_it_is_set() {
    let code = "
    before = defined?($never_set)
    $was_set = 1
    after = defined?($was_set)
    [before.inspect, after.inspect].join(',')
    ";
    assert_eq!(
        run_top_level_s("defined_gvar", code),
        "nil,\"global-variable\""
    );
}
