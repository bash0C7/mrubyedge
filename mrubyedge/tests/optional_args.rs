extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn default_arg_array_reused() {
    let code = r#"
def incr(times, state=[0])
  return state if times == 0
  state[0] += 1
  incr(times - 1, state)
end

result = incr(3)
result[0]
    "#;
    let binary = mrbc_compile("default_arg_array_reused", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_int: i32 = result.as_ref().try_into().unwrap();
    assert_eq!(result_int, 3);
}

#[test]
fn default_arg_simple() {
    let code = r##"
def greet(name, greeting="Hello")
  "#{greeting}, #{name}!"
end

greet("Alice")
    "##;
    let binary = mrbc_compile("default_arg_simple", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_str: String = result.as_ref().try_into().unwrap();
    assert_eq!(result_str, "Hello, Alice!");
}

#[test]
fn default_arg_multiple() {
    let code = r#"
def create_point(x=0, y=3, z=6)
  [x, y, z]
end

result = create_point()
result[0] + result[1] + result[2]
    "#;
    let binary = mrbc_compile("default_arg_multiple", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_int: i32 = result.as_ref().try_into().unwrap();
    assert_eq!(result_int, 9);
}

#[test]
fn default_arg_override() {
    let code = r#"
def add(a, b=10)
  a + b
end

add(5, 20)
    "#;
    let binary = mrbc_compile("default_arg_override", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_int: i32 = result.as_ref().try_into().unwrap();
    assert_eq!(result_int, 25);
}

#[test]
fn default_arg_partial_override() {
    let code = r#"
def calc(a, b=2, c=3)
  a * b + c
end

calc(5, 4)
    "#;
    let binary = mrbc_compile("default_arg_partial_override", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_int: i32 = result.as_ref().try_into().unwrap();
    assert_eq!(result_int, 23);
}

#[test]
fn default_arg_hash_reused() {
    let code = r#"
def update_counter(times, state={count: 0, other: 2})
  return state if times == 0
  state[:count] += 1
  update_counter(times - 1, state)
end

result = update_counter(5)
result[:count] + result[:other]
    "#;
    let binary = mrbc_compile("default_arg_hash_reused", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_int: i32 = result.as_ref().try_into().unwrap();
    assert_eq!(result_int, 5 + 2);
}

#[test]
fn default_arg_mixed_types() {
    let code = r##"
def format_message(msg, prefix="Info", level=1, enabled=true)
  return "disabled" unless enabled
  "[#{prefix}:#{level}] #{msg}"
end

format_message("Test")
    "##;
    let binary = mrbc_compile("default_arg_mixed_types", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_str: String = result.as_ref().try_into().unwrap();
    assert_eq!(result_str, "[Info:1] Test");
}

// Class#new reaches initialize through mrb_funcall, which builds a frame with
// no callinfo for OP_ENTER to read the argument count off.

#[test]
fn optional_args_survive_class_new() {
    let code = r##"
class Element
  attr_reader :tag, :props, :children
  def initialize(tag, props = {}, children = [])
    @tag = tag
    @props = props
    @children = children
  end
end

e = Element.new("div", {"class" => "probe"}, ["a", "b"])
"#{e.tag}/#{e.props.size}/#{e.children.size}"
    "##;
    let result = run("optional_args_class_new", code);

    // Assert
    assert_eq!(result, "div/1/2");
}

#[test]
fn optional_args_default_when_omitted_in_class_new() {
    let code = r##"
class Element
  attr_reader :props, :children
  def initialize(tag, props = {}, children = [])
    @props = props
    @children = children
  end
end

e = Element.new("div")
"#{e.props.size}/#{e.children.size}"
    "##;
    let result = run("optional_args_class_new_default", code);

    // Assert
    assert_eq!(result, "0/0");
}

#[test]
fn rest_arg_survives_class_new() {
    let code = r#"
class Bag
  attr_reader :items
  def initialize(*items)
    @items = items
  end
end

Bag.new(1, 2, 3).items.join(",")
    "#;
    let result = run("rest_arg_class_new", code);

    // Assert
    assert_eq!(result, "1,2,3");
}

#[test]
fn block_arg_still_reaches_a_method_called_through_funcall() {
    // Enumerable#map calls `each` through mrb_funcall with the block as the
    // trailing argument. The count must not mistake it for a positional.
    let code = r#"
class MyCollection
  def each(&block)
    block.call(1)
    block.call(2)
  end
  include Enumerable
end

MyCollection.new.map { |x| x * 3 }.join(",")
    "#;
    let result = run("block_arg_via_funcall", code);

    // Assert
    assert_eq!(result, "3,6");
}

// The caller leaves the block right after the arguments it passed; the
// declared `&block` lives at a slot derived from the signature.

#[test]
fn block_argument_after_rest_is_nil_when_no_block_given() {
    let code = "
    class Dispatch
      def call(name, *args, &block)
        return name.to_s if block.nil?
        block.call(name)
      end
    end

    def test_main
      Dispatch.new.call(:plain)
    end
    ";
    // Assert
    assert_eq!(run_s("block_after_rest_no_block", code), "plain");
}

#[test]
fn block_argument_after_rest_receives_the_block() {
    let code = "
    class Dispatch
      def call(name, *args, &block)
        return name.to_s if block.nil?
        block.call(name)
      end
    end

    def test_main
      Dispatch.new.call(:blocky) { |n| n.to_s + '!' }
    end
    ";
    // Assert
    assert_eq!(run_s("block_after_rest_with_block", code), "blocky!");
}
