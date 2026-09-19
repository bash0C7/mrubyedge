extern crate mrubyedge;
mod helpers;
use helpers::*;

fn run_top_level_s(name: &'static str, code: &'static str) -> String {
    let binary = mrbc_compile(name, code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    result.as_ref().try_into().unwrap()
}

fn run_top_level_i(name: &'static str, code: &'static str) -> i64 {
    let binary = mrbc_compile(name, code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    result.as_ref().try_into().unwrap()
}

// The exact shape app/funicular/initializer.rb calls Funicular.start with:
// one positional argument, one of several declared keyword parameters, and
// a block. Before this fix the block landed one register short of where
// the declared keyword parameter pushed it, so `&block` bound to whatever
// value happened to sit there instead — in this VM's boot sequence, a
// keyword's own String value.
#[test]
fn a_block_survives_a_call_with_a_positional_and_one_of_several_keywords() {
    let code = r##"
    def start(component_class = nil, container: "app", props: {}, hydrate: false, &block)
      block.call("ok").to_s
    end
    start(nil, container: "app") do |x|
      x
    end
    "##;
    let result = run_top_level_s("block_with_one_keyword", code);
    assert_eq!(&result, "ok");
}

// Same shape, but every declared keyword is supplied at the call site (not
// just one), and required + optional positional parameters both precede
// the keywords. The kdict register is still exactly one register no matter
// how many keyword pairs are packed into it.
#[test]
fn a_block_survives_a_call_with_all_keywords_supplied() {
    let code = r##"
    def configure(name, size = 1, weight: 0, color: "red", &block)
      block.call(name, size, weight, color)
    end
    configure("box", 2, weight: 9, color: "blue") { |n, s, w, c| "#{n}-#{s}-#{w}-#{c}" }
    "##;
    let result = run_top_level_s("block_with_all_keywords", code);
    assert_eq!(&result, "box-2-9-blue");
}

// `**rest` occupies its own register ahead of the block, separate from the
// one-register keyword dict a declared keyword parameter would use — both
// count toward `kdict` the same way, so the block still lands past it.
#[test]
fn a_block_survives_a_call_with_a_keyword_rest_parameter() {
    let code = r##"
    def wrap(**opts, &block)
      block.call(opts[:x])
    end
    wrap(x: 5) { |v| v * 2 }
    "##;
    let result = run_top_level_i("block_with_kwrest", code);
    assert_eq!(result, 10);
}

// No keyword parameters at all: the block must still land exactly where it
// did before this fix (this VM's pre-existing, already-correct behavior for
// the common case — a regression guard, not new behavior).
#[test]
fn a_block_still_works_with_no_keywords_at_all() {
    let code = r##"
    def plain(a, b, &block)
      block.call(a + b)
    end
    plain(1, 2) { |sum| sum * 10 }
    "##;
    let result = run_top_level_i("block_with_no_keywords", code);
    assert_eq!(result, 30);
}

// A block passed through `mrb_funcall`/`call_block` (Enumerable#map calling
// a user-defined `each`, the way this crate implements Enumerable) must
// keep working — that path places its own arguments directly into
// registers and must not have this fix's register relocation applied on
// top of it. This is the regression this fix's first draft introduced.
#[test]
fn a_block_via_call_block_is_unaffected_by_a_prior_do_op_send_blocks_stale_state() {
    let code = r##"
    class MyCollection
      include Enumerable
      def each(&block)
        block.call(1)
        block.call(2)
        block.call(3)
      end
    end

    def test_it(outer_kw: 1, &outer_block)
      outer_block.call
      MyCollection.new.map { |x| x * 2 }
    end
    result = test_it(outer_kw: 1) { }
    "#{result[0]},#{result[1]},#{result[2]}"
    "##;
    let result = run_top_level_s("block_via_call_block_unaffected", code);
    assert_eq!(&result, "2,4,6");
}
