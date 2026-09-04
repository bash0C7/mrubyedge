// The `defined?` helpers mruby 4.0 compiles into.
//
// 3.3 inlined the test; 4.0 emits a call to `__defined_const?` and its
// siblings, so a chunk PicoRuby's mrbc produced needs them to exist. The
// test harness compiles with 3.3, so these call the helpers by name --
// which is exactly what the 4.0 bytecode does.
extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn a_defined_constant_answers_constant() {
    let code = "
    class Known; end
    [__defined_const?(:Known), __defined_const?(:Unknown)].map { |v| v.inspect }.join(',')
    ";
    assert_eq!(run("defined_const", code), "\"constant\",nil");
}

#[test]
fn a_constant_defined_in_an_enclosing_module_is_found() {
    let code = "
    module Outer
      INSIDE = 1
      def self.check
        __defined_const?(:INSIDE).inspect
      end
    end
    Outer.check
    ";
    assert_eq!(run("defined_const_nested", code), "\"constant\"");
}

#[test]
fn a_scoped_constant_answers_through_its_owner() {
    let code = "
    module Outer
      class Inner; end
    end
    [__defined_const_path?(Outer, :Inner),
     __defined_const_path?(Outer, :Missing)].map { |v| v.inspect }.join(',')
    ";
    assert_eq!(run("defined_const_path", code), "\"constant\",nil");
}

#[test]
fn a_method_on_self_answers_method() {
    let code = "
    class Thing
      def known; end
      def check
        [__defined_method?(:known), __defined_method?(:missing)].map { |v| v.inspect }.join(',')
      end
    end
    Thing.new.check
    ";
    assert_eq!(run("defined_method", code), "\"method\",nil");
}

#[test]
fn an_instance_variable_answers_only_once_it_is_set() {
    let code = "
    class Thing
      def check
        before = __defined_ivar?(:@v)
        @v = 1
        after = __defined_ivar?(:@v)
        [before, after].map { |x| x.inspect }.join(',')
      end
    end
    Thing.new.check
    ";
    assert_eq!(run("defined_ivar", code), "nil,\"instance-variable\"");
}

#[test]
fn a_global_answers_only_once_it_is_set() {
    let code = "
    before = __defined_gvar?(:$never_set)
    $was_set = 1
    after = __defined_gvar?(:$was_set)
    [before, after].map { |x| x.inspect }.join(',')
    ";
    assert_eq!(run("defined_gvar", code), "nil,\"global-variable\"");
}
