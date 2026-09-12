extern crate mrubyedge;

use std::rc::Rc;

use super::helpers::*;
use mrubyedge::yamrb::value::RObject;

#[test]
fn test_eval_multiple_rites_with_classes() {
    // First code: define Foo class
    let code1 = r#"
    class Foo
      def bar
        "Hello from Foo"
      end
    end
    "#;
    let binary1 = mrbc_compile("code1", code1);
    let mut rite1 = mrubyedge::rite::load(&binary1).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite1);
    vm.run().unwrap();

    // Second code: define Bar class
    let code2 = r#"
    class Bar
      def baz
        "Hello from Bar"
      end
    end
    "#;
    let binary2 = mrbc_compile("code2", code2);
    let mut rite2 = mrubyedge::rite::load(&binary2).unwrap();
    vm.eval_rite(&mut rite2).unwrap();

    // Third code: use both Foo and Bar classes
    let code3 = r#"
    def test_both
      foo = Foo.new
      bar = Bar.new
      [foo.bar, bar.baz]
    end
    "#;
    let binary3 = mrbc_compile("code3", code3);
    let mut rite3 = mrubyedge::rite::load(&binary3).unwrap();
    vm.eval_rite(&mut rite3).unwrap();

    // Call the method that uses both classes
    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_both", &args).unwrap();
    let arr: Vec<Rc<RObject>> = result.as_ref().try_into().unwrap();
    assert_eq!(
        TryInto::<String>::try_into(arr[0].as_ref()).unwrap(),
        "Hello from Foo"
    );
    assert_eq!(
        TryInto::<String>::try_into(arr[1].as_ref()).unwrap(),
        "Hello from Bar"
    );
}

#[test]
fn test_eval_multiple_rites_accumulate_methods() {
    // First code: define initial method
    let code1 = r#"
    def greet(name)
      "Hello, #{name}!"
    end
    "#;
    let binary1 = mrbc_compile("greet", code1);
    let mut rite1 = mrubyedge::rite::load(&binary1).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite1);
    vm.run().unwrap();

    // Second code: define another method
    let code2 = r#"
    def farewell(name)
      "Goodbye, #{name}!"
    end
    "#;
    let binary2 = mrbc_compile("farewell", code2);
    let mut rite2 = mrubyedge::rite::load(&binary2).unwrap();
    vm.eval_rite(&mut rite2).unwrap();

    // Third code: use both methods
    let code3 = r#"
    def test_methods
      [greet("Alice"), farewell("Bob")]
    end
    "#;
    let binary3 = mrbc_compile("test_methods", code3);
    let mut rite3 = mrubyedge::rite::load(&binary3).unwrap();
    vm.eval_rite(&mut rite3).unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_methods", &args).unwrap();
    let arr: Vec<Rc<RObject>> = result.as_ref().try_into().unwrap();
    assert_eq!(
        TryInto::<String>::try_into(arr[0].as_ref()).unwrap(),
        "Hello, Alice!"
    );
    assert_eq!(
        TryInto::<String>::try_into(arr[1].as_ref()).unwrap(),
        "Goodbye, Bob!"
    );
}

#[test]
fn test_eval_multiple_rites_with_inheritance() {
    // First code: define base class
    let code1 = r#"
    class Animal
      def speak
        "Some sound"
      end
    end
    "#;
    let binary1 = mrbc_compile("animal", code1);
    let mut rite1 = mrubyedge::rite::load(&binary1).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite1);
    vm.run().unwrap();

    // Second code: define subclass that inherits from Animal
    let code2 = r#"
    class Dog < Animal
      def speak
        "Woof!"
      end
    end
    "#;
    let binary2 = mrbc_compile("dog", code2);
    let mut rite2 = mrubyedge::rite::load(&binary2).unwrap();
    vm.eval_rite(&mut rite2).unwrap();

    // Third code: use the subclass
    let code3 = r#"
    def test_inheritance
      dog = Dog.new
      dog.speak
    end
    "#;
    let binary3 = mrbc_compile("test_inheritance", code3);
    let mut rite3 = mrubyedge::rite::load(&binary3).unwrap();
    vm.eval_rite(&mut rite3).unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_inheritance", &args).unwrap();
    let result_str: String = result.as_ref().try_into().unwrap();
    assert_eq!(result_str, "Woof!");
}

#[test]
fn test_eval_multiple_rites_with_modules() {
    // First code: define module
    let code1 = r#"
    module Greeter
      def greet
        "Hello from #{@name}"
      end
    end
    "#;
    let binary1 = mrbc_compile("module", code1);
    let mut rite1 = mrubyedge::rite::load(&binary1).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite1);
    vm.run().unwrap();

    // Second code: define class that includes the module
    let code2 = r#"
    class Person
      include Greeter
      
      def initialize(name)
        @name = name
      end
    end
    "#;
    let binary2 = mrbc_compile("person", code2);
    let mut rite2 = mrubyedge::rite::load(&binary2).unwrap();
    vm.eval_rite(&mut rite2).unwrap();

    // Third code: use the class with included module
    let code3 = r#"
    def test_module_include
      person = Person.new("Alice")
      person.greet
    end
    "#;
    let binary3 = mrbc_compile("test_include", code3);
    let mut rite3 = mrubyedge::rite::load(&binary3).unwrap();
    vm.eval_rite(&mut rite3).unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_module_include", &args).unwrap();
    let result_str: String = result.as_ref().try_into().unwrap();
    assert_eq!(result_str, "Hello from Alice");
}

#[test]
fn an_opcode_the_vm_does_not_run_is_an_error_not_a_crash_test() {
    let code = "
def pass(n)
  m = n
  m
end
pass(1)
    ";
    let binary = mrbc_compile("unimplemented", code);

    // GETSV takes the same two operands as MOVE and mruby's compiler never
    // emits it, so swapping the opcode leaves the instruction stream aligned.
    let offset = {
        let rite = mrubyedge::rite::load(&binary).unwrap();
        let mut found = None;
        for irep in rite.irep.iter() {
            let base = irep.insn.as_ptr() as usize - binary.as_ptr() as usize;
            let mut cur = 0usize;
            let mut s: &[u8] = irep.insn;
            while !s.is_empty() {
                let before = s.len();
                let Ok((op, _f, _e)) = mrubyedge::rite::insn::fetch_next(&mut s) else {
                    break;
                };
                if matches!(op, mrubyedge::rite::insn::OpCode::MOVE) {
                    found = Some(base + cur);
                    break;
                }
                cur += before - s.len();
            }
            if found.is_some() {
                break;
            }
        }
        found.expect("the chunk should hold a MOVE")
    };

    let mut chunk = binary.clone();
    chunk[offset] = 23;

    let mut rite = mrubyedge::rite::load(&chunk).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);

    // Assert
    assert!(vm.run().is_err());
}
