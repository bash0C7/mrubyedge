extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn raise_test() {
    let code = "
    def test_raise
      raise \"Intentional Error\"
    end
    ";
    let binary = mrbc_compile("raise", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_raise", &args).err();
    assert_eq!(&result.unwrap().message(), "Intentional Error");
}

#[test]
fn raise_nest_test() {
    let code = "
    def do_raise
      raise \"Intentional Error 2\"
      p :HOGE
    end

    def test_raise
      do_raise
      p :NG
    end
    ";
    let binary = mrbc_compile("raise_nest", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_raise", &args).err();
    assert_eq!(&result.unwrap().message(), "Intentional Error 2");
}

#[test]
fn raise_nest_test_toplevel() {
    let code = "
    def do_raise
      raise \"Intentional Error 0\"
      p :HOGE
    end

    def shim
      do_raise
      p :NG_NG
    end

    shim
    p :NG
    ";
    let binary = mrbc_compile("raise_nest_toplevel", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);

    // Assert
    let result = vm.run().err();
    assert_eq!(
        &result
            .unwrap()
            .downcast_ref::<mrubyedge::Error>()
            .unwrap()
            .message(),
        "Intentional Error 0",
    );
}

#[test]
fn raise_nest_nest_test() {
    let code = "
    def do_raise
      raise \"Intentional Error 2b\"
      p :HOGE
    end

    def shim
      do_raise
      p :NG_1
    end

    def test_raise
      shim
      p :NG_2
    end
    ";
    let binary = mrbc_compile("raise_nest_nest", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_raise", &args).err();
    assert_eq!(&result.unwrap().message(), "Intentional Error 2b");
}

#[test]
fn rescue_test() {
    let code = "
    def test_raise
      begin
        raise \"Intentional Error 3\"
      rescue => e
        puts \"rescue: #{e.message}\"
        \"rescue: #{e.message}\"
      end
    end
    ";
    let binary = mrbc_compile("rescue", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: String = mrb_funcall(&mut vm, None, "test_raise", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(&result, "rescue: Intentional Error 3");
}

#[test]
fn rescue_nest_test() {
    let code = "
    def test_raise
      raise \"Intentional Error 4\"
    end

    def test_raise_parent
      begin
        test_raise
        \"NG\"
      rescue => e
        puts \"rescue: #{e.message}\"
        \"rescue: #{e.message}\"
      end
    end
    ";
    let binary = mrbc_compile("rescue_nest", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: String = mrb_funcall(&mut vm, None, "test_raise_parent", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(&result, "rescue: Intentional Error 4");
}

#[test]
fn rescue_nest_nest_test() {
    let code = "
    def test_raise
      raise \"Intentional Error 4b\"
    end

    def shim
      test_raise
      \"NG_1\"
    end

    def test_raise_parent
      begin
        shim
        \"NG_2\"
      rescue => e
        puts \"rescue: #{e.message}\"
        \"rescue: #{e.message}\"
      end
    end
    ";
    let binary = mrbc_compile("rescue_nest_nest", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: String = mrb_funcall(&mut vm, None, "test_raise_parent", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(&result, "rescue: Intentional Error 4b");
}

// A catch handler guards only its own region: begin < pc <= end, the
// innermost (last declared) one winning, as mruby's catch_handler_find does.

#[test]
fn raise_outside_begin_is_not_caught_by_a_later_rescue_test() {
    let code = r#"
    def test_uncaught
      raise "boom"
      begin
        raise "second"
      rescue => e
        "rescued"
      end
    end
    "#;
    let binary = mrbc_compile("raise_outside_begin_is_not_caught_by_a_later_rescue", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let err = mrb_funcall(&mut vm, None, "test_uncaught", &[]).unwrap_err();
    assert!(
        format!("{:?}", err).contains("boom"),
        "expected the first raise to escape, got {:?}",
        err
    );
}

#[test]
fn raise_after_a_rescue_block_is_not_caught_by_it_test() {
    let code = r#"
    def test_after
      begin
        1
      rescue => e
        "rescued"
      end
      raise "later"
    end
    "#;
    let binary = mrbc_compile("raise_after_a_rescue_block_is_not_caught_by_it", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let err = mrb_funcall(&mut vm, None, "test_after", &[]).unwrap_err();
    assert!(
        format!("{:?}", err).contains("later"),
        "expected the raise to escape, got {:?}",
        err
    );
}

#[test]
fn the_innermost_rescue_wins_test() {
    let code = r#"
    def test_nested
      begin
        begin
          raise "inner"
        rescue => e
          "inner:#{e.message}"
        end
      rescue => e
        "outer:#{e.message}"
      end
    end
    "#;
    let binary = mrbc_compile("the_innermost_rescue_wins", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let result = mrb_funcall(&mut vm, None, "test_nested", &[]).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "inner:inner");
}

#[test]
fn a_rescue_still_catches_what_is_inside_it_test() {
    let code = r#"
    def test_inside
      begin
        raise "boom"
      rescue => e
        "rescued:#{e.message}"
      end
    end
    "#;
    let binary = mrbc_compile("a_rescue_still_catches_what_is_inside_it", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let result = mrb_funcall(&mut vm, None, "test_inside", &[]).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "rescued:boom");
}
