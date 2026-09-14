#![cfg(feature = "mruby-uri")]
extern crate mrubyedge;

mod helpers;
use helpers::*;

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

#[test]
fn encode_www_form_from_pairs() {
    let code = r#"
    def test_main
      URI.encode_www_form([["q", "a b"], ["page", 2], ["tag", ["x", "y"]], ["blank", nil]])
    end
    "#;
    assert_eq!(
        run_test_main_s("uri_pairs", code),
        "q=a+b&page=2&tag=x&tag=y&blank"
    );
}

#[test]
fn encode_and_decode_a_component() {
    let code = r##"
    def test_main
      encoded = URI.encode_www_form_component("a b&c=d/e")
      "#{encoded}|#{URI.decode_www_form_component(encoded)}"
    end
    "##;
    assert_eq!(
        run_test_main_s("uri_component", code),
        "a+b%26c%3Dd%2Fe|a b&c=d/e"
    );
}

#[test]
fn decode_with_multibyte_utf8_literal_percent() {
    let code = r##"
    def test_main
      URI.decode_www_form_component("100%満足")
    end
    "##;
    assert_eq!(
        run_test_main_s("uri_multibyte", code),
        "100%満足"
    );
}
