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
fn class_new_forwards_its_keyword_arguments_to_initialize() {
    let code = r##"
    class Config
      def initialize(attributes:)
        @attributes = attributes
      end
      def report
        @attributes.to_s
      end
    end
    Config.new(attributes: 42).report
    "##;
    let result = run_top_level_s("class_new_kwarg", code);
    assert_eq!(&result, "42");
}
