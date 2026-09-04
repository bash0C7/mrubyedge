#![cfg(feature = "mruby-uri")]
// Runs in CI: the workflow enables mruby-uri.
extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn require_uri_succeeds_when_the_build_carries_it() {
    let code = r#"
    def test_main
      begin
        require 'uri'
        "loaded"
      rescue LoadError
        "LoadError"
      end
    end
    "#;
    assert_eq!(run_s("require_uri", code), "loaded");
}

#[test]
fn require_still_raises_for_something_that_is_not_linked_in() {
    let code = r#"
    def test_main
      begin
        require 'js'
        "loaded"
      rescue LoadError
        "LoadError"
      end
    end
    "#;
    assert_eq!(run_s("require_js", code), "LoadError");
}

#[test]
fn encode_www_form_from_pairs() {
    let code = r#"
    def test_main
      URI.encode_www_form([["q", "a b"], ["page", 2], ["tag", ["x", "y"]], ["blank", nil]])
    end
    "#;
    assert_eq!(run_s("uri_pairs", code), "q=a+b&page=2&tag=x&tag=y&blank");
}

#[test]
fn encode_and_decode_a_component() {
    let code = r##"
    def test_main
      encoded = URI.encode_www_form_component("a b&c=d/e")
      "#{encoded}|#{URI.decode_www_form_component(encoded)}"
    end
    "##;
    assert_eq!(run_s("uri_component", code), "a+b%26c%3Dd%2Fe|a b&c=d/e");
}
