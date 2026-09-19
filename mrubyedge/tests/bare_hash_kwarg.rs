extern crate mrubyedge;
mod helpers;
use helpers::*;

fn run_top_level_s(name: &'static str, code: &'static str) -> String {
    let binary = mrbc_compile(name, code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    result.as_ref().try_into().unwrap()
}

#[test]
fn a_method_with_no_keyword_parameter_receives_bare_pairs_as_one_hash() {
    let code = r##"
    def load_schema(schema_data)
      schema_data["attributes"].to_s + "," + schema_data["endpoints"].to_s
    end
    load_schema("attributes" => 1, "endpoints" => 2)
    "##;
    let result = run_top_level_s("bare_hash_kwarg", code);
    assert_eq!(&result, "1,2");
}

#[test]
fn symbol_keyword_syntax_also_folds_when_no_keyword_parameter_is_declared() {
    let code = r##"
    def opts(h)
      h[:a].to_s + "," + h[:b].to_s
    end
    opts(a: 1, b: 2)
    "##;
    let result = run_top_level_s("bare_hash_kwarg_symbol", code);
    assert_eq!(&result, "1,2");
}

// `options.merge(href: path)` — the exact shape `app/.../component.rb`'s
// `build_link_to` uses — is a bare keyword call with no positional argument
// at all. Hash#merge (a cfunc) has no ENTER instruction to fold `href:
// path` into a trailing Hash the way a Ruby callee's ENTER does, so it has
// to read the call's kwargs back out itself.
#[test]
fn hash_merge_accepts_a_bare_keyword_argument() {
    let code = r##"
    options = {class: "a"}
    merged = options.merge(href: "/x")
    merged[:class].to_s + "," + merged[:href].to_s
    "##;
    let result = run_top_level_s("cfunc_bare_kwarg", code);
    assert_eq!(&result, "a,/x");
}
