extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn times_test_c() {
    let code = "
    def test_times
      i = 0
      3.times do |j|
        i += j * 2
      end
      i
    end
    ";
    let binary = mrbc_compile("times_c", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_times", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, 6);
}

#[test]
fn range_each_test_c() {
    let code = "
    def test_each
      i = 0
      (1..10).each do |j|
        i += j * 2
      end
      i
    end
    ";
    let binary = mrbc_compile("range_each_c", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_each", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, 110);
}

#[test]
fn array_each_test_c() {
    let code = "
    def test_each
      i = 0
      [1, 10, 100, 1000, 10000].each do |j|
        i += j * 2
      end
      i
    end
    ";
    let binary = mrbc_compile("array_each_c", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_each", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, 22222);
}

#[test]
fn array_each_nested_test_c() {
    let code = "
    def do_times_nest
      result = 200
      3.times do |i|
        3.times do |j|
          result += 200
        end
      end
      
      result
    end
    ";
    let binary = mrbc_compile("array_each_nested_c", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "do_times_nest", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, 2000);
}

#[test]
fn expired_closure_test() {
    let code = "
    def do_time_block
      result = 0
      ->(i) {
        result += 100
        puts \"result = #{result}\"
      }
    end

    def do_times
      3.times(&do_time_block)
    end
    ";
    let binary = mrbc_compile("expired_closure", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "do_times", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, 3);
}
#[test]
fn a_method_can_recurse_through_a_block_that_writes_a_local() {
    // Each activation of an IREP needs its own environment. Keyed by IREP
    // alone, the inner call overwrote the outer's, and the inner RETURN then
    // captured its own registers into the outer's environment and expired it:
    // the outer block's `out` came back as "captured value not found".
    let code = "
    def flatten_ish(v)
      return [v] unless v.is_a?(Array)
      out = []
      v.each { |x| out += flatten_ish(x) }
      out
    end

    flatten_ish([1, [2, [3, 4]], 5]).join(',')
    ";
    let binary = mrbc_compile("recursive_block_local", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);

    // Assert
    let result = vm.run().unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(&result, "1,2,3,4,5");
}

#[test]
fn a_proc_still_sees_its_locals_after_the_frame_returns() {
    // The counterpart: an environment that outlives its frame keeps the
    // registers of *that* frame.
    let code = "
    def counter
      n = 0
      -> { n += 1 }
    end

    a = counter
    b = counter
    [a.call, a.call, b.call].join(',')
    ";
    let binary = mrbc_compile("closure_outlives_frame", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);

    // Assert
    let result = vm.run().unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(&result, "1,2,1");
}
