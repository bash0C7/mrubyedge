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
fn a_double_splat_at_a_call_site_reaches_a_declared_rest_parameter() {
    let code = r##"
    def opts(**rest)
      rest[:x].to_s + "," + rest[:y].to_s
    end
    extra = {y: 2}
    opts(x: 1, **extra)
    "##;
    let result = run_top_level_s("kwarg_pack_rest", code);
    assert_eq!(&result, "1,2");
}
