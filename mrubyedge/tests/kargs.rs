extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use std::rc::Rc;

use helpers::*;
use mrubyedge::Error;
use mrubyedge::yamrb::helpers::mrb_define_cmethod;
use mrubyedge::yamrb::value::RObject;
use mrubyedge::yamrb::vm::VM;

#[test]
fn basic_keyword_args_test() {
    let code = "
    def greet(name, greeting: 'Hello')
      greeting + ', ' + name
    end

    greet('Bob', greeting: 'Hi')
    ";
    let binary = mrbc_compile("basic_kargs", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_str: String = result.as_ref().try_into().unwrap();
    assert_eq!(&result_str, "Hi, Bob");
}

#[test]
fn multiple_keyword_args_test() {
    let code = "
    def destruct_it(x, foo: 42, bar: 99)
      x + foo + bar
    end

    destruct_it(10, foo: 20, bar: 30)
    ";
    let binary = mrbc_compile("multiple_kargs", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_int: i32 = result.as_ref().try_into().unwrap();
    assert_eq!(result_int, 10 + 20 + 30);
}

#[test]
fn keyword_args_string_symbol_test() {
    let code = "
    def format_text(text, prefix: '', suffix: '')
      prefix + text + suffix
    end

    result1 = format_text('Hello')
    result2 = format_text('Hello', prefix: '>> ')
    result3 = format_text('Hello', suffix: ' <<')
    result4 = format_text('Hello', prefix: '[', suffix: ']')
    [result1, result2, result3, result4]
    ";
    let binary = mrbc_compile("string_kargs", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_array: Vec<Rc<RObject>> = result.as_ref().try_into().unwrap();
    let mut expected_array = vec!["Hello", ">> Hello", "Hello <<", "[Hello]"];
    for obj in result_array {
        let s: String = obj.as_ref().try_into().unwrap();
        let expected = expected_array.remove(0);
        assert_eq!(&s, expected);
    }
}

#[test]
fn keyword_args_nested_call_test() {
    let code = "
    def inner(value, multiplier: 2)
      value * multiplier
    end

    def outer(x, factor: 3)
      result1 = inner(x)
      result2 = inner(x + 1, multiplier: factor)
      result1 + result2
    end

    [
      outer(5),
      outer(5, factor: 4)
    ]
    ";
    let binary = mrbc_compile("nested_kargs", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let (got1, got2): (i32, i32) = result.as_ref().try_into().unwrap();
    assert_eq!(got1, 28); // 5 * 2 + 6 * 3
    assert_eq!(got2, 34); // 5 * 2 + 6 * 4
}

#[test]
fn keyword_args_c_definition_test() {
    fn test_mrb_multiply(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
        let a: i32 = args
            .first()
            .ok_or_else(|| Error::ArgumentError("missing positional argument 'a'".to_string()))?
            .as_ref()
            .try_into()?;
        let kwargs = vm.get_kwargs();
        match kwargs {
            Some(kargs) => {
                let b_obj = kargs.get("b").ok_or_else(|| {
                    Error::ArgumentError("missing keyword argument 'b'".to_string())
                })?;
                let c_obj = kargs.get("c").ok_or_else(|| {
                    Error::ArgumentError("missing keyword argument 'c'".to_string())
                })?;
                let b: i32 = b_obj.as_ref().try_into()?;
                let c: i32 = c_obj.as_ref().try_into()?;
                Ok(Rc::new(RObject::integer((a * b * c) as i64)))
            }
            None => Err(Error::ArgumentError(
                "missing keyword arguments".to_string(),
            )),
        }
    }

    let code = "multiply(7, b: 3, c: 11)";
    let binary = mrbc_compile("cdef_kargs", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let kernel = vm.object_class.clone();
    mrb_define_cmethod(&mut vm, kernel, "multiply", Box::new(test_mrb_multiply));

    let result = vm.run().unwrap();
    let result_int: i32 = result.as_ref().try_into().unwrap();
    assert_eq!(result_int, 7 * 3 * 11);
}

#[test]
fn keyword_splat_basic_test() {
    let code = "
def process_options(**options)
  options.length
end

process_options(foo: 1, bar: 2, baz: 3)
    ";
    let binary = mrbc_compile("kwsplat_basic", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_int: i32 = result.as_ref().try_into().unwrap();
    assert_eq!(result_int, 3);
}

#[test]
fn keyword_splat_empty_test() {
    let code = "
def process_options(**options)
  options.length
end

process_options()
    ";
    let binary = mrbc_compile("kwsplat_empty", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_int: i32 = result.as_ref().try_into().unwrap();
    assert_eq!(result_int, 0);
}

#[test]
fn keyword_splat_access_test() {
    let code = "
def get_value(**options)
  options[:name]
end

get_value(name: 'Alice', age: 30)
    ";
    let binary = mrbc_compile("kwsplat_access", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_str: String = result.as_ref().try_into().unwrap();
    assert_eq!(&result_str, "Alice");
}

#[test]
fn keyword_splat_with_regular_kwargs_test() {
    let code = "
def configure(mode: 'default', **options)
  result = mode + ':'
  options.each do |key, value|
    result = result + ' ' + key.to_s + '=' + value.to_s
  end
  result
end

configure(mode: 'production', host: 'localhost', port: 8080)
    ";
    let binary = mrbc_compile("kwsplat_mixed", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_str: String = result.as_ref().try_into().unwrap();
    assert!(result_str.starts_with("production:"));
    assert!(result_str.contains("host=localhost"));
    assert!(result_str.contains("port=8080"));
}

#[test]
fn keyword_splat_with_positional_and_splat_args_test() {
    let code = "
def complex_method(x, *args, required: 100, **kwargs)
  sum = x + required
  args.each { |a| sum = sum + a * 10 }
  kwargs.each { |k, v| sum = sum + v * 15 }
  sum
end

complex_method(10, 20, 30, required: 5, foo: 15, bar: 25)
    ";
    let binary = mrbc_compile("kwsplat_complex", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result_int: i32 = result.as_ref().try_into().unwrap();
    assert_eq!(result_int, 10 + 5 + 20 * 10 + 30 * 10 + 15 * 15 + 25 * 15); // 1050
}

// A call site cannot know whether the callee declares keyword parameters, so
// the compiler emits keyword pairs either way. When the signature has none,
// they belong in one trailing Hash argument.

#[test]
fn keywords_become_a_trailing_hash_when_the_method_takes_none_test() {
    let code = r##"
def tag(name, props = {})
  "#{name}:#{props.size}:#{props[:class]}"
end

tag("div", class: "probe", id: "main")
    "##;
    let result = run("kwargs_to_positional_hash", code);

    // Assert
    assert_eq!(result, "div:2:probe");
}

#[test]
fn keywords_become_a_trailing_hash_for_a_required_parameter_test() {
    let code = r#"
def only(props)
  props.size
end

only(a: 1, b: 2, c: 3).to_s
    "#;
    let result = run("kwargs_to_required_hash", code);

    // Assert
    assert_eq!(result, "3");
}

#[test]
fn keywords_reach_initialize_through_class_new_test() {
    let code = r##"
class Widget
  attr_reader :props
  def initialize(name, props = {})
    @props = props
  end
end

Widget.new("w", class: "probe").props[:class]
    "##;
    let result = run("keywords_reach_initialize_through_class_new", code);

    // Assert
    assert_eq!(result, "probe");
}

#[test]
fn keywords_still_bind_to_declared_keyword_parameters_test() {
    let code = r#"
def sized(width: 1, height: 2)
  width * height
end

sized(width: 5, height: 6).to_s
    "#;
    let result = run("keywords_still_bind_to_declared_keyword_parameters", code);

    // Assert
    assert_eq!(result, "30");
}

#[test]
fn a_bare_hash_argument_keeps_its_string_keys_test() {
    // `m("a" => 1)` is a Hash argument, not a keyword call. The compiler
    // emits it through the same keyword pairs either way, so the callee has
    // to be handed back what was written.
    let code = "
    def take(h)
      # sorted: this VM's Hash does not keep insertion order
      h.keys.sort.join(',')
    end

    take('attributes' => 1, 'endpoints' => 2)
    ";
    let result = run("a_bare_hash_argument_keeps_its_string_keys", code);

    // Assert
    assert_eq!(&result, "attributes,endpoints");
}

#[test]
fn a_declared_keyword_is_not_also_in_the_kwrest_test() {
    let code = "
    def f(attributes:, **rest)
      \"#{attributes}/#{rest.keys.join(',')}\"
    end

    f(attributes: 1, z: 2)
    ";
    let result = run("a_declared_keyword_is_not_also_in_the_kwrest", code);

    // Assert
    assert_eq!(&result, "1/z");
}

#[test]
fn kwrest_sits_past_the_optional_parameters_test() {
    let code = "
    def f(a, b = 9, **rest)
      \"#{a}/#{b}/#{rest.keys.join(',')}\"
    end

    f(1, x: 2)
    ";
    let result = run("kwrest_sits_past_the_optional_parameters", code);

    // Assert
    assert_eq!(&result, "1/9/x");
}

#[test]
fn a_block_reaches_a_method_declaring_several_keywords_test() {
    // Keyword arguments take one register between the positional parameters
    // and the block, not one per keyword.
    let code = "
    def start(c = nil, container: 'app', props: {}, hydrate: false, &blk)
      return 'no block' unless blk
      blk.call(container)
    end

    start(nil, container: 'root') { |v| \"got #{v}\" }
    ";
    let result = run("a_block_reaches_a_method_declaring_several_keywords", code);

    // Assert
    assert_eq!(&result, "got root");
}

#[test]
fn keyword_arguments_reach_a_method_written_in_rust_test() {
    // Hash#merge is a Rust method: it has no OP_ENTER to fold the pairs into
    // a trailing Hash, so the send has to do it.
    let code = "
    { a: 1 }.merge(b: 2).keys.map { |k| k.to_s }.sort.join(',')
    ";
    let result = run("keyword_arguments_reach_a_method_written_in_rust", code);

    // Assert
    assert_eq!(&result, "a,b");
}

// Packed arguments: mruby's CALL_MAXARGS in OP_SEND's n and k nibbles.

#[test]
fn a_splatted_array_is_spread_over_the_parameters_test() {
    let code = "
    def f(a, b, c)
      \"#{a}-#{b}-#{c}\"
    end

    args = [2, 3]
    f(1, *args)
    ";
    let result = run("a_splatted_array_is_spread_over_the_parameters", code);

    // Assert
    assert_eq!(&result, "1-2-3");
}

#[test]
fn a_double_splatted_hash_arrives_as_keywords_test() {
    let code = "
    def f(x:, y: 0)
      \"#{x},#{y}\"
    end

    opts = { x: 1, y: 2 }
    f(**opts)
    ";
    let result = run("a_double_splatted_hash_arrives_as_keywords", code);

    // Assert
    assert_eq!(&result, "1,2");
}

#[test]
fn splats_and_a_block_can_all_appear_at_once_test() {
    let code = "
    def f(a, *rest, **kw, &blk)
      \"#{a}/#{rest.join(',')}/#{kw.keys.join(',')}/#{blk.call}\"
    end

    args = [1, 2, 3]
    opts = { x: 9 }
    f(*args, **opts) { 'B' }
    ";
    let result = run("splats_and_a_block_can_all_appear_at_once", code);

    // Assert
    assert_eq!(&result, "1/2,3/x/B");
}
