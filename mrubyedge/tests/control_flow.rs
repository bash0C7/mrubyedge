extern crate mrubyedge;

mod helpers;
use helpers::*;
use mrubyedge::yamrb::helpers::mrb_funcall;

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

// Compiles and runs `code`, then converts the program's own value to a String.
fn run_top_s(name: &'static str, code: &'static str) -> String {
    let binary = mrbc_compile(name, code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap().as_ref().try_into().unwrap()
}

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
    assert_eq!(run_test_main_s("break_stays_local", code), "after the loop");
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
    assert_eq!(run_test_main_i("break_value", code), 20);
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
    assert_eq!(
        run_test_main_s("break_across_call", code),
        "caller survived"
    );
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
    assert_eq!(run_test_main_i("no_break", code), 6);
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
    assert_eq!(run_test_main_s("super_in_initialize", code), "element");
}

#[test]
fn break_ends_only_the_call_the_caller_keeps_running_test() {
    // `scan` survives its own `break` (break only ends the `each` call) and
    // returns its own value; a coarser fix that unwinds `scan`'s whole call
    // instead would smuggle the break value into test_main's `scan_result`
    // and skip the rest of `scan`, returning nil here instead.
    let code = "
    def scan
      [1, 2, 3].each do |x|
        break if x == 2
      end
      'scanned'
    end

    def test_main
      scan_result = scan
      scan_result
    end
    ";
    assert_eq!(run_test_main_s("break_ends_only_the_call", code), "scanned");
}

#[test]
fn break_ends_the_iteration_at_toplevel_test() {
    // On unmodified v2.0.0 this returns nil; the same shape called through
    // mrb_funcall (as `run_test_main_s` does) passes either way, which is
    // why this needs the top-level harness to actually expose the bug.
    let code = "
    def m
      [1, 2, 3].each { break }
      'after'
    end
    m
    ";
    assert_eq!(
        run_top_s("break_ends_the_iteration_at_toplevel", code),
        "after"
    );
}

#[test]
fn break_in_a_forwarded_block_ends_the_call_it_was_written_for_test() {
    // The block was written at the `wrapper` call, so the break ends
    // `wrapper`, not the `each` that merely forwarded it.
    let code = "
    def wrapper(&b)
      [1, 2].each(&b)
      'wrapper end'
    end

    def test_main
      wrapper { break 'B' }
    end
    ";
    assert_eq!(
        run_test_main_s(
            "break_in_a_forwarded_block_ends_the_call_it_was_written_for",
            code
        ),
        "B"
    );
}

#[test]
fn super_break_and_return_in_one_chain_test() {
    let code = "
    class A
      def scan(list)
        list.each { |x| break x if x > 1 }
      end
    end

    class B < A
      def scan(list)
        v = super(list)
        return 'got:' + v.to_s
      end
    end

    def test_main
      B.new.scan([1, 2, 3])
    end
    ";
    assert_eq!(
        run_test_main_s("super_break_and_return_in_one_chain", code),
        "got:2"
    );
}

#[test]
fn super_after_a_block_in_a_funcall_entered_method_test() {
    // The plain block clears the method frame, so `super` only finds the
    // identity again because `call_block` restores it afterward.
    let code = "
    class A
      def initialize
        @v = 'base'
      end
      def v
        @v
      end
    end

    class B < A
      def initialize
        [1].each { |x| x }
        super
        @v = @v + '+derived'
      end
    end

    def test_main
      B.new.v
    end
    ";
    assert_eq!(
        run_test_main_s("super_after_a_block_in_a_funcall_entered_method", code),
        "base+derived"
    );
}
