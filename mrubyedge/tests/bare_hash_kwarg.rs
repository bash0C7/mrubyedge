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
