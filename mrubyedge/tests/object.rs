extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use std::rc::Rc;

use helpers::*;
use mrubyedge::yamrb::value::RObject;

#[test]
fn object_test() {
    let code = "
    class Hello
      def world
        puts \"hello world\"
        1
      end
    end

    def test_main
      Hello.new.world
    end
    ";
    let binary = mrbc_compile("add", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_main", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, 1);
}

#[test]
fn object_extend_test() {
    let code = r#"
    module Greeter
      def greet
        "Hello from module"
      end
    end

    module Farewell
      def bye
        "Goodbye from module"
      end
    end

    def test_extend
      obj = Object.new
      obj.extend(Greeter)
      result1 = obj.greet

      obj.extend(Farewell)
      result2 = obj.bye

      [result1, result2]
    end
    "#;
    let binary = mrbc_compile("extend", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_extend", &args).unwrap();
    let arr: Vec<Rc<RObject>> = result.as_ref().try_into().unwrap();
    assert_eq!(
        TryInto::<String>::try_into(arr[0].as_ref()).unwrap(),
        "Hello from module"
    );
    assert_eq!(
        TryInto::<String>::try_into(arr[1].as_ref()).unwrap(),
        "Goodbye from module"
    );
}

#[test]
fn object_extend_multiple_modules_test() {
    let code = r#"
    module M1
      def m1_method
        "from M1"
      end
    end

    module M2
      def m2_method
        "from M2"
      end
    end

    def test_extend_multiple
      obj = Object.new
      obj.extend(M1, M2)
      [obj.m1_method, obj.m2_method]
    end
    "#;
    let binary = mrbc_compile("extend_multiple", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_extend_multiple", &args).unwrap();
    let arr: Vec<Rc<RObject>> = result.as_ref().try_into().unwrap();
    assert_eq!(
        TryInto::<String>::try_into(arr[0].as_ref()).unwrap(),
        "from M1"
    );
    assert_eq!(
        TryInto::<String>::try_into(arr[1].as_ref()).unwrap(),
        "from M2"
    );
}

#[test]
fn object_extend_overrides_class_method_test() {
    let code = r#"
    class MyClass
      def greet
        "from class"
      end
    end

    module MyModule
      def greet
        "from module"
      end
    end

    def test_extend_override
      obj = MyClass.new
      obj.extend(MyModule)
      obj.greet
    end
    "#;
    let binary = mrbc_compile("extend_override_class", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_extend_override", &args).unwrap();
    let result_str: String = result.as_ref().try_into().unwrap();
    assert_eq!(result_str, "from module");
}

#[test]
fn object_extend_singleton_method_priority_test() {
    let code = r#"
    module MyModule
      def greet
        "from module"
      end
    end

    def test_singleton_priority
      obj = Object.new
      obj.extend(MyModule)
      
      def obj.greet
        "from singleton"
      end
      
      obj.greet
    end
    "#;
    let binary = mrbc_compile("extend_singleton_priority", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_singleton_priority", &args).unwrap();
    let result_str: String = result.as_ref().try_into().unwrap();
    assert_eq!(result_str, "from singleton");
}

#[test]
fn object_extend_multiple_priority_test() {
    let code = r#"
    module M1
      def greet
        "from M1"
      end
    end

    module M2
      def greet
        "from M2"
      end
    end

    module M3
      def greet
        "from M3"
      end
    end

    def test_multiple_priority
      obj = Object.new
      obj.extend(M1)
      result1 = obj.greet
      
      obj.extend(M2)
      result2 = obj.greet
      
      obj.extend(M3)
      result3 = obj.greet
      
      [result1, result2, result3]
    end
    "#;
    let binary = mrbc_compile("extend_multiple_priority", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_multiple_priority", &args).unwrap();
    let arr: Vec<Rc<RObject>> = result.as_ref().try_into().unwrap();
    assert_eq!(
        TryInto::<String>::try_into(arr[0].as_ref()).unwrap(),
        "from M1"
    );
    assert_eq!(
        TryInto::<String>::try_into(arr[1].as_ref()).unwrap(),
        "from M2"
    );
    assert_eq!(
        TryInto::<String>::try_into(arr[2].as_ref()).unwrap(),
        "from M3"
    );
}

#[test]
fn object_extend_multiple_arguments_priority_test() {
    let code = r#"
    module M1
      def greet
        "from M1"
      end
      
      def m1_only
        "M1 only"
      end
    end

    module M2
      def greet
        "from M2"
      end
      
      def m2_only
        "M2 only"
      end
    end

    def test_args_priority
      obj = Object.new
      # extend(M1, M2) extends in order: M2, then M1, so M1 has priority
      obj.extend(M1, M2)
      [obj.greet, obj.m1_only, obj.m2_only]
    end
    "#;
    let binary = mrbc_compile("extend_args_priority", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_args_priority", &args).unwrap();
    let arr: Vec<Rc<RObject>> = result.as_ref().try_into().unwrap();
    // M1 is extended last, so greet calls M1's method
    assert_eq!(
        TryInto::<String>::try_into(arr[0].as_ref()).unwrap(),
        "from M1"
    );
    assert_eq!(
        TryInto::<String>::try_into(arr[1].as_ref()).unwrap(),
        "M1 only"
    );
    assert_eq!(
        TryInto::<String>::try_into(arr[2].as_ref()).unwrap(),
        "M2 only"
    );
}

#[test]
fn object_loop_basic_test() {
    let code = r#"
    def test_loop
      i = 0
      loop do
        i += 1
        break if i >= 5
      end
      i
    end
    "#;
    let binary = mrbc_compile_debug("loop_basic", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_loop", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, 5);
}

#[test]
fn object_block_given_with_block_test() {
    let code = r#"
    def method_with_block
      block_given?
    end

    def test_block_given_with_block
      method_with_block { }
    end
    "#;
    let binary = mrbc_compile("block_given_with_block", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result: bool = mrb_funcall(&mut vm, None, "test_block_given_with_block", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, true);
}

#[test]
fn object_block_given_without_block_test() {
    let code = r#"
    def method_with_block
      block_given?
    end

    def test_block_given_without_block
      method_with_block
    end
    "#;
    let binary = mrbc_compile("block_given_without_block", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result: bool = mrb_funcall(&mut vm, None, "test_block_given_without_block", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, false);
}

#[test]
fn object_block_given_with_args_and_block_test() {
    let code = r#"
    def method_with_args(a, b, c)
      block_given?
    end

    def test_block_given_with_args_and_block
      method_with_args(1, 2, 3) { }
    end
    "#;
    let binary = mrbc_compile("block_given_with_args_and_block", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result: bool = mrb_funcall(&mut vm, None, "test_block_given_with_args_and_block", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, true);
}

#[test]
fn object_block_given_with_args_without_block_test() {
    let code = r#"
    def method_with_args(a, b, c)
      block_given?
    end

    def test_block_given_with_args_without_block
      method_with_args(1, 2, 3)
    end
    "#;
    let binary = mrbc_compile("block_given_with_args_without_block", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result: bool = mrb_funcall(
        &mut vm,
        None,
        "test_block_given_with_args_without_block",
        &args,
    )
    .unwrap()
    .as_ref()
    .try_into()
    .unwrap();
    assert_eq!(result, false);
}

#[test]
fn object_respond_to_existing_method_test() {
    let code = r#"
    def test_respond_to_existing
      obj = Object.new
      obj.respond_to?("to_s")
    end
    "#;
    let binary = mrbc_compile("respond_to_existing", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result: bool = mrb_funcall(&mut vm, None, "test_respond_to_existing", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, true);
}

#[test]
fn object_respond_to_non_existing_method_test() {
    let code = r#"
    def test_respond_to_non_existing
      obj = Object.new
      obj.respond_to?("non_existing_method")
    end
    "#;
    let binary = mrbc_compile("respond_to_non_existing", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result: bool = mrb_funcall(&mut vm, None, "test_respond_to_non_existing", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, false);
}

#[test]
fn object_public_send_test() {
    let code = r#"
    class TestClass
      def hello(name)
        "Hello, #{name}!"
      end
    end

    def test_public_send
      obj = TestClass.new
      obj.public_send("hello", "World")
    end
    "#;
    let binary = mrbc_compile("public_send", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result: String = mrb_funcall(&mut vm, None, "test_public_send", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, "Hello, World!");
}

#[test]
fn object_public_send_no_args_test() {
    let code = r#"
    class TestClass
      def greet
        "Hi!"
      end
    end

    def test_public_send_no_args
      obj = TestClass.new
      obj.public_send("greet")
    end
    "#;
    let binary = mrbc_compile("public_send_no_args", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result: String = mrb_funcall(&mut vm, None, "test_public_send_no_args", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, "Hi!");
}

// BasicObject is the root: a class inheriting from it starts with nothing
// but what dispatch itself needs.

#[test]
fn basic_object_is_the_root_of_the_hierarchy_test() {
    let code = "
    class Bare < BasicObject; end

    def test_main
      Bare.ancestors.map { |k| k.to_s }.join(',')
    end
    ";
    // Assert
    assert_eq!(run_s("basic_object_ancestors", code), "Bare,BasicObject");
}

#[test]
fn a_basic_object_subclass_has_no_object_methods_test() {
    let code = "
    class Bare < BasicObject
      def initialize
        @x = 1
      end
    end

    def test_main
      begin
        Bare.new.to_s
        'answered'
      rescue NoMethodError
        'no to_s'
      end
    end
    ";
    // Assert
    assert_eq!(run_s("basic_object_no_to_s", code), "no to_s");
}

// Object#equal? is identity, with immediates compared by value.

#[test]
fn different_objects_are_not_identical_test() {
    let code = "
    def test_main
      a = 'x'
      b = []
      \"#{a.equal?(b)}\"
    end
    ";
    // Assert
    assert_eq!(run_s("equal_identity_diff", code), "false");
}

#[test]
fn an_object_is_identical_to_itself_test() {
    let code = "
    def test_main
      a = 'x'
      a.equal?(a)
    end
    ";
    // Assert
    assert!(run_b("equal_identity_self", code));
}

#[test]
fn string_literals_are_not_identical_test() {
    let code = "
    def test_main
      \"#{'same'.equal?('same')}\"
    end
    ";
    // Assert
    assert_eq!(run_s("equal_identity_string_literals", code), "false");
}

#[test]
fn symbols_and_nil_are_identical_across_references_test() {
    let code = "
    def test_main
      :sym.equal?(:sym) && nil.equal?(nil)
    end
    ";
    // Assert
    assert!(run_b("equal_identity_singletons", code));
}

// BasicObject#! is true for nil and false only.

#[test]
fn bang_negates_false_test() {
    let code = "
    def test_main
      !false
    end
    ";
    // Assert
    assert!(run_b("bang_false", code));
}

#[test]
fn bang_negates_nil_test() {
    let code = "
    def test_main
      !nil
    end
    ";
    // Assert
    assert!(run_b("bang_nil", code));
}

#[test]
fn bang_of_a_truthy_string_is_false_test() {
    let code = "
    def test_main
      \"#{!'a string'}\"
    end
    ";
    // Assert
    assert_eq!(run_s("bang_string", code), "false");
}

#[test]
fn bang_of_zero_is_false_test() {
    let code = "
    def test_main
      \"#{!0}\"
    end
    ";
    // Assert
    assert_eq!(run_s("bang_zero", code), "false");
}

// `Klass === obj` is `obj.is_a?(Klass)`, which is what `case x when Hash`
// compiles to.

#[test]
fn case_when_a_class_asks_whether_the_value_is_one_test() {
    let code = "
    def kind(v)
      case v
      when Hash then 'hash'
      when Array then 'array'
      when String then 'string'
      when Integer then 'int'
      when nil then 'nil'
      else 'other'
      end
    end

    def test_main
      [{}, [], 's', 1, nil, 1.5].map { |v| kind(v) }.join(',')
    end
    ";
    // Assert
    assert_eq!(
        run_s("case_when_class", code),
        "hash,array,string,int,nil,other"
    );
}

#[test]
fn case_when_a_class_follows_the_ancestors_test() {
    let code = "
    class Animal; end
    class Cat < Animal; end
    module Loud; end
    class Siren; include Loud; end

    def kind(v)
      case v
      when Loud then 'loud'
      when Animal then 'animal'
      else 'other'
      end
    end

    def test_main
      [Cat.new, Siren.new, 1].map { |v| kind(v) }.join(',')
    end
    ";
    // Assert
    assert_eq!(run_s("case_when_ancestors", code), "animal,loud,other");
}

#[test]
fn a_symbol_asked_for_a_symbol_is_itself_test() {
    let code = "
    def test_main
      [:a.to_sym, 'b'.to_sym].map { |s| s.to_s }.join(',')
    end
    ";
    // Assert
    assert_eq!(run_s("symbol_to_sym", code), "a,b");
}
