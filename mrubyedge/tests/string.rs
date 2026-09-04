extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;

use helpers::*;
// use mrubyedge::yamrb::value::RObject;
// use std::rc::Rc;

#[test]
fn string_new_test() {
    let code = "
    def test_string_new
      string = String.new
      string.size
    end
    ";
    let binary = mrbc_compile("string_new", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_new", &args).unwrap();
    let result: i32 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 0);
}

#[test]
fn string_add_test() {
    let code = r#"
    def test_string_add
      "hello" + " " + "world"
    end
    "#;
    let binary = mrbc_compile("string_add", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_add", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn string_mul_test() {
    let code = r#"
    def test_string_mul
      "ab" * 3
    end
    "#;
    let binary = mrbc_compile("string_mul", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_mul", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "ababab");
}

#[test]
fn string_append_test() {
    let code = r#"
    def test_string_append
      s = "hello"
      s << " world"
      s
    end
    "#;
    let binary = mrbc_compile("string_append", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_append", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn string_b_test() {
    let code = r#"
    def test_string_b
      s = "hello"
      s.b
    end
    "#;
    let binary = mrbc_compile("string_b", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_b", &args).unwrap();
    let result: Vec<u8> = result.as_ref().try_into().unwrap();
    assert_eq!(result, b"hello");
}

#[test]
fn string_clear_test() {
    let code = r#"
    def test_string_clear
      str = "hello"
      str.clear
      str
    end
    "#;
    let binary = mrbc_compile("string_clear", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_clear", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "");
}

#[test]
fn string_chomp_test() {
    let code = r#"
    def test_string_chomp
      "hello\n".chomp
    end
    "#;
    let binary = mrbc_compile("string_chomp", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_chomp", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn string_chomp_self_test() {
    let code = r#"
    def test_string_chomp_self
      s = "hello\n"
      s.chomp!
      s
    end
    "#;
    let binary = mrbc_compile("string_chomp_self", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_chomp_self", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn string_dup_test() {
    let code = r#"
    def test_string_dup
      s = "hello"
      s.dup == s
    end
    "#;
    let binary = mrbc_compile("string_dup", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_dup", &args).unwrap();
    let result: bool = result.as_ref().try_into().unwrap();
    assert!(result);
}

#[test]
fn string_empty_test() {
    let code = r#"
    def test_string_empty
      "".empty?
    end
    "#;
    let binary = mrbc_compile("string_empty", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_empty", &args).unwrap();
    let result: bool = result.as_ref().try_into().unwrap();
    assert!(result);
}

#[test]
fn string_getbyte_test() {
    let code = r#"
    def test_string_getbyte
      "hello".getbyte(1)
    end
    "#;
    let binary = mrbc_compile("string_getbyte", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_getbyte", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 101); // 'e'
}

#[test]
fn string_setbyte_test() {
    let code = r#"
    def test_string_setbyte
      s = "hello"
      s.setbyte(0, 72)
      s
    end
    "#;
    let binary = mrbc_compile("string_setbyte", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_setbyte", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "Hello");
}

#[test]
fn string_index_test() {
    let code = r#"
    def test_string_index
      "hello".index("l")
    end
    "#;
    let binary = mrbc_compile("string_index", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_index", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 2);
}

#[test]
fn string_ord_test() {
    let code = r#"
    def test_string_ord
      "A".ord
    end
    "#;
    let binary = mrbc_compile("string_ord", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_ord", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 65);
}

#[test]
fn string_slice_test() {
    let code = r#"
    def test_string_slice
      s = "hello"
      s.slice(1, 2)
    end
    "#;
    let binary = mrbc_compile("string_slice", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_slice", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "el");
}

#[test]
fn string_slice_self_test() {
    let code = r#"
    def test_string_slice_self
      s = "hello"
      s.slice!(1, 2)
      s
    end
    "#;
    let binary = mrbc_compile("string_slice_self", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_slice_self", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hlo");
}

#[test]
fn string_split_test() {
    let code = r#"
    def test_string_split
      "a,b,c".split(",").size
    end
    "#;
    let binary = mrbc_compile("string_split", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_split", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 3);
}

#[test]
fn string_lstrip_test() {
    let code = r#"
    def test_string_lstrip
      "  hello  ".lstrip
    end
    "#;
    let binary = mrbc_compile("string_lstrip", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_lstrip", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hello  ");
}

#[test]
fn string_lstrip_self_test() {
    let code = r#"
    def test_string_lstrip_self
      s = "  hello  "
      s.lstrip!
      s
    end
    "#;
    let binary = mrbc_compile("string_lstrip_self", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_lstrip_self", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hello  ");
}

#[test]
fn string_rstrip_test() {
    let code = r#"
    def test_string_rstrip
      "  hello  ".rstrip
    end
    "#;
    let binary = mrbc_compile("string_rstrip", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_rstrip", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "  hello");
}

#[test]
fn string_rstrip_self_test() {
    let code = r#"
    def test_string_rstrip_self
      s = "  hello  "
      s.rstrip!
      s
    end
    "#;
    let binary = mrbc_compile("string_rstrip_self", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_rstrip_self", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "  hello");
}

#[test]
fn string_strip_test() {
    let code = r#"
    def test_string_strip
      "  hello  ".strip
    end
    "#;
    let binary = mrbc_compile("string_strip", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_strip", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn string_strip_self_test() {
    let code = r#"
    def test_string_strip_self
      s = "  hello  "
      s.strip!
      s
    end
    "#;
    let binary = mrbc_compile("string_strip_self", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_strip_self", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn string_to_sym_test() {
    let code = r#"
    def test_string_to_sym
      "hello".to_sym.to_s
    end
    "#;
    let binary = mrbc_compile("string_to_sym", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_to_sym", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn string_start_with_test() {
    let code = r#"
    def test_string_start_with
      "hello".start_with?("hel")
    end
    "#;
    let binary = mrbc_compile("string_start_with", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_start_with", &args).unwrap();
    let result: bool = result.as_ref().try_into().unwrap();
    assert!(result);
}

#[test]
fn string_end_with_test() {
    let code = r#"
    def test_string_end_with
      "hello".end_with?("lo")
    end
    "#;
    let binary = mrbc_compile("string_end_with", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_end_with", &args).unwrap();
    let result: bool = result.as_ref().try_into().unwrap();
    assert!(result);
}

#[test]
fn string_include_test() {
    let code = r#"
    def test_string_include
      "hello".include?("ll")
    end
    "#;
    let binary = mrbc_compile("string_include", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_include", &args).unwrap();
    let result: bool = result.as_ref().try_into().unwrap();
    assert!(result);
}

#[test]
fn string_bytes_test() {
    let code = r#"
    def test_string_bytes
      "AB".bytes
    end
    "#;
    let binary = mrbc_compile("string_bytes", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_bytes", &args).unwrap();
    let result = result.as_vec_owned().unwrap();
    assert_eq!(result.len(), 2);
    let first: i64 = result[0].as_ref().try_into().unwrap();
    let second: i64 = result[1].as_ref().try_into().unwrap();
    assert_eq!(first, 65);
    assert_eq!(second, 66);
}

#[test]
fn string_chars_test() {
    let code = r#"
    def test_string_chars
      "hello".chars
    end
    "#;
    let binary = mrbc_compile("string_chars", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_chars", &args).unwrap();
    let result = result.as_vec_owned().unwrap();
    assert_eq!(result.len(), 5);
    let first: String = result[0].as_ref().try_into().unwrap();
    assert_eq!(first, "h");
    let last: String = result[4].as_ref().try_into().unwrap();
    assert_eq!(last, "o");
}

#[test]
fn string_upcase_test() {
    let code = r#"
    def test_string_upcase
      "hello".upcase
    end
    "#;
    let binary = mrbc_compile("string_upcase", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_upcase", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "HELLO");
}

#[test]
fn string_upcase_self_test() {
    let code = r#"
    def test_string_upcase_self
      s = "hello"
      s.upcase!
      s
    end
    "#;
    let binary = mrbc_compile("string_upcase_self", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_upcase_self", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "HELLO");
}

#[test]
fn string_downcase_test() {
    let code = r#"
    def test_string_downcase
      "HELLO".downcase
    end
    "#;
    let binary = mrbc_compile("string_downcase", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_downcase", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn string_downcase_self_test() {
    let code = r#"
    def test_string_downcase_self
      s = "HELLO"
      s.downcase!
      s
    end
    "#;
    let binary = mrbc_compile("string_downcase_self", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_downcase_self", &args).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn string_to_i_test() {
    let code = r#"
    def test_string_to_i
      "123".to_i
    end
    "#;
    let binary = mrbc_compile("string_to_i", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_to_i", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 123);
}

#[test]
fn string_to_f_test() {
    let code = r#"
    def test_string_to_f
      "54.71".to_f
    end
    "#;
    let binary = mrbc_compile("string_to_f", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_to_f", &args).unwrap();
    let result: f64 = result.as_ref().try_into().unwrap();
    assert!((result - 54.71).abs() < f64::EPSILON);
}

#[test]
fn string_size_test() {
    let code = r#"
    def test_string_size
      "hello".size
    end
    "#;
    let binary = mrbc_compile("string_size", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_string_size", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 5);
}

#[test]
fn string_gsub_test() {
    let code = r#"
    def test_gsub
      "banana".gsub("an", "-")
    end
    "#;
    let binary = mrbc_compile("string_gsub", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let result = mrb_funcall(&mut vm, None, "test_gsub", &[]).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "b--a");
}

#[test]
fn string_gsub_escape_html_test() {
    let code = r#"
    def test_escape
      "x&y<z>".gsub("&", "&amp;").gsub("<", "&lt;").gsub(">", "&gt;")
    end
    "#;
    let binary = mrbc_compile("string_gsub_escape", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let result = mrb_funcall(&mut vm, None, "test_escape", &[]).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "x&amp;y&lt;z&gt;");
}

#[test]
fn string_gsub_empty_pattern_test() {
    let code = r#"
    def test_gsub_empty
      "abc".gsub("", "-")
    end
    "#;
    let binary = mrbc_compile("string_gsub_empty", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let result = mrb_funcall(&mut vm, None, "test_gsub_empty", &[]).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "-a-b-c-");
}

#[test]
fn string_gsub_backreference_test() {
    let code = r#"
    def test_gsub_backref
      "aaa".gsub("a", '\0\0')
    end
    "#;
    let binary = mrbc_compile("string_gsub_backref", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let result = mrb_funcall(&mut vm, None, "test_gsub_backref", &[]).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "aaaaaa");
}

#[test]
fn string_gsub_block_test() {
    let code = r#"
    def test_gsub_block
      "hello".gsub("l") { |m| m.upcase }
    end
    "#;
    let binary = mrbc_compile("string_gsub_block", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let result = mrb_funcall(&mut vm, None, "test_gsub_block", &[]).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "heLLo");
}

#[test]
fn string_sub_replaces_only_the_first_test() {
    let code = r#"
    def test_sub
      "banana".sub("an", "-")
    end
    "#;
    let binary = mrbc_compile("string_sub", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let result = mrb_funcall(&mut vm, None, "test_sub", &[]).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "b-ana");
}

#[test]
fn string_gsub_self_test() {
    let code = r#"
    def test_gsub_self
      s = "aaa"
      s.gsub!("a", "b")
      s
    end
    "#;
    let binary = mrbc_compile("string_gsub_self", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let result = mrb_funcall(&mut vm, None, "test_gsub_self", &[]).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "bbb");
}

#[test]
fn string_sub_self_returns_nil_without_a_match_test() {
    let code = r#"
    def test_sub_self_nil
      "nope".sub!("z", "!").nil?
    end
    "#;
    let binary = mrbc_compile("string_sub_self_nil", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let result = mrb_funcall(&mut vm, None, "test_sub_self_nil", &[]).unwrap();
    assert!(matches!(
        result.value,
        mrubyedge::yamrb::value::RValue::Bool(true)
    ));
}

#[test]
fn string_slice_with_range_test() {
    let code = r#"
    def test_slice_range
      s = "abcdef"
      [s[1..-1], s[0..1], s[0...2], s[6..-1], s[7..-1].nil?, s[1..], s[..1], s[-2..-1]]
    end
    "#;
    let binary = mrbc_compile("string_slice_range", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let result = mrb_funcall(&mut vm, None, "test_slice_range", &[]).unwrap();
    let items = match &result.value {
        mrubyedge::yamrb::value::RValue::Array(a) => a.borrow().clone(),
        _ => panic!("not an array"),
    };
    let text = |i: usize| -> String { items[i].as_ref().try_into().unwrap() };
    assert_eq!(text(0), "bcdef");
    assert_eq!(text(1), "ab");
    assert_eq!(text(2), "ab");
    assert_eq!(text(3), "");
    assert!(matches!(
        items[4].value,
        mrubyedge::yamrb::value::RValue::Bool(true)
    ));
    assert_eq!(text(5), "bcdef");
    assert_eq!(text(6), "ab");
    assert_eq!(text(7), "ef");
}

#[test]
fn string_slice_self_with_range_test() {
    let code = r##"
    def test_slice_self_range
      s = "hello world"
      removed = s.slice!(0..5)
      "#{removed}|#{s}"
    end
    "##;
    let binary = mrbc_compile("string_slice_self_range", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    let result = mrb_funcall(&mut vm, None, "test_slice_self_range", &[]).unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(result, "hello |world");
}

#[test]
fn capitalize_test() {
    let code = "
    ['ruby', 'RUBY', 'rUbY', ''].map { |s| s.capitalize }.join(',')
    ";
    let binary = mrbc_compile("string_capitalize", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: String = result.as_ref().try_into().unwrap();
    assert_eq!(&result, "Ruby,Ruby,Ruby,");
}

#[test]
fn string_each_byte_test() {
    let code = "
    def test_main
      total = 0
      'abc'.each_byte { |b| total += b }
      total
    end
    ";
    assert_eq!(run_i("each_byte", code), 294);
}

#[test]
fn string_each_char_test() {
    let code = "
    def test_main
      out = ''
      'abc'.each_char { |c| out = c + out }
      out
    end
    ";
    assert_eq!(run_s("each_char", code), "cba");
}
