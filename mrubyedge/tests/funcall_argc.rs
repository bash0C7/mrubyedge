extern crate mrubyedge;
mod helpers;
use helpers::*;

fn run_top_level_i(name: &'static str, code: &'static str) -> i64 {
    let binary = mrbc_compile(name, code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    result.as_ref().try_into().unwrap()
}

// Matches Funicular::VDOM::Element.new(tag, props = {}, children = []) —
// two optional positional parameters, both supplied, reached through
// Class#new -> initialize (mrb_funcall/call_block), not a bytecode call site.
#[test]
fn class_new_with_two_optional_args_both_supplied() {
    let code = r##"
    class Foo
      def initialize(a, b = {}, c = [])
        @c = c
      end
      def c
        @c
      end
    end
    Foo.new("x", {}, [1, 2, 3]).c.size
    "##;
    let result = run_top_level_i("class_new_two_opt", code);
    assert_eq!(result, 3);
}

#[test]
fn class_new_with_one_optional_arg_supplied() {
    let code = r##"
    class Bar
      def initialize(a, b = "default")
        @b = b
      end
      def b
        @b
      end
    end
    Bar.new("x", "custom").b
    "##;
    let binary = mrbc_compile("class_new_one_opt", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let s: String = result.as_ref().try_into().unwrap();
    assert_eq!(&s, "custom");
}
