extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn attr_reader_test() {
    let code = "
    class Hello
      attr_reader :world

      def update_world
        @world = 123
      end
    end

    def test_main
      w = Hello.new
      w.update_world
      w.world
    end
    ";
    let binary = mrbc_compile("attr_reader", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_main", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, 123);
}

#[test]
fn attr_reader_2_test() {
    let code = "
    class Hello
      attr_reader :world
    end

    def test_main
      w = Hello.new
      w.world
    end
    ";
    let binary = mrbc_compile("attr_reader_2", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_main", &args).unwrap();
    assert!(result.as_ref().is_nil());
}

#[test]
fn attr_accessor_test() {
    let code = "
    class Hello
      attr_accessor :world
    end

    def test_main
      w = Hello.new
      w.world = \"Hola, attr\"
      w.world
    end
    ";
    let binary = mrbc_compile("attr_accessor", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: String = mrb_funcall(&mut vm, None, "test_main", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(&result, "Hola, attr");
}

#[test]
fn class_definition_isolation_test() {
    let code = "
    class Test1
      def hello
        123
      end
    end

    class Test2
      def hello
        456
      end
    end

    def test_main1
      Test1.new.hello
    end

    def test_main2
      Test2.new.hello
    end
    ";
    let binary = mrbc_compile("class_definition_isolation", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let val1: i32 = mrb_funcall(&mut vm, None, "test_main1", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    let val2: i32 = mrb_funcall(&mut vm, None, "test_main2", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(val1, 123);
    assert_eq!(val2, 456);
}

#[test]
fn class_inheritance_super_test() {
    let code = "
    class Test1
      def hello
        123
      end
    end

    class Test3 < Test1
      def hello
        super + 1
      end
    end

    def test_main
      Test3.new.hello
    end
    ";
    let binary = mrbc_compile("class_inheritance_super", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_main", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, 124);
}

#[test]
fn class_define_class_method_test() {
    let code = "
    class Test
      def self.hello
        123
      end
    end

    def test_main
      Test.hello
    end
    ";
    let binary = mrbc_compile("class_define_class_method", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_main", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, 123);
}

#[test]
fn class_inheritance_class_method_test() {
    let code = "
    class Test1
      def self.hello
        123
      end
    end

    class Test2 < Test1
      def self.hello
        super + 1
      end
    end

    def test_main
      Test2.hello
    end
    ";
    let binary = mrbc_compile("class_inheritance_class_method", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_main", &args)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();
    assert_eq!(result, 124);
}

#[test]
fn class_can_have_singleton_instance_variables() {
    let code = r#"
    class Hello
      def self.set_world(value)
        @world = value
      end

      def self.get_world
        @world
      end
    end

    def test_main_0
      Hello.get_world
    end

    def test_main_1
      Hello.set_world("hello")
      Hello.get_world
    end
    "#;
    let binary = mrbc_compile("class_singleton_ivar", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();
    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_main_0", &args).unwrap();
    assert!(result.as_ref().is_nil());

    let result = mrb_funcall(&mut vm, None, "test_main_1", &args).unwrap();
    let value: String = result
        .as_ref()
        .try_into()
        .expect("get_world should return string");
    assert_eq!(value, "hello");
}

#[test]
fn scoped_constant_assignment_test() {
    let code = "
class Config
end

Config::LIMIT = 12
Config::LIMIT
    ";
    let binary = mrbc_compile("setmcnst", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 12);
}

#[test]
fn super_without_arguments_forwards_them_test() {
    let code = "
class Base
  def twice(n)
    n * 2
  end
end

class Sub < Base
  def twice(n)
    super
  end
end

Sub.new.twice(21)
    ";
    let binary = mrbc_compile("zsuper", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 42);
}

#[test]
fn toplevel_constant_assignment_test() {
    let code = "
LIMIT = 9
LIMIT
    ";
    let binary = mrbc_compile("setconst", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 9);
}

#[test]
fn singleton_class_body_defines_a_class_method_test() {
    let code = "
class Widget
  class << self
    def build
      5
    end
  end
end

Widget.build
    ";
    let binary = mrbc_compile("sclass", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 5);
}

#[test]
fn a_method_defined_past_the_symbol_limit_test() {
    // Up to 255 symbols the compiler folds the definition into TDEF; past that
    // it emits TCLASS, METHOD and DEF instead.
    let code = r#"
[1].each { |q0| q0 }
[1].each { |q1| q1 }
[1].each { |q2| q2 }
[1].each { |q3| q3 }
[1].each { |q4| q4 }
[1].each { |q5| q5 }
[1].each { |q6| q6 }
[1].each { |q7| q7 }
[1].each { |q8| q8 }
[1].each { |q9| q9 }
[1].each { |q10| q10 }
[1].each { |q11| q11 }
[1].each { |q12| q12 }
[1].each { |q13| q13 }
[1].each { |q14| q14 }
[1].each { |q15| q15 }
[1].each { |q16| q16 }
[1].each { |q17| q17 }
[1].each { |q18| q18 }
[1].each { |q19| q19 }
[1].each { |q20| q20 }
[1].each { |q21| q21 }
[1].each { |q22| q22 }
[1].each { |q23| q23 }
[1].each { |q24| q24 }
[1].each { |q25| q25 }
[1].each { |q26| q26 }
[1].each { |q27| q27 }
[1].each { |q28| q28 }
[1].each { |q29| q29 }
[1].each { |q30| q30 }
[1].each { |q31| q31 }
[1].each { |q32| q32 }
[1].each { |q33| q33 }
[1].each { |q34| q34 }
[1].each { |q35| q35 }
[1].each { |q36| q36 }
[1].each { |q37| q37 }
[1].each { |q38| q38 }
[1].each { |q39| q39 }
[1].each { |q40| q40 }
[1].each { |q41| q41 }
[1].each { |q42| q42 }
[1].each { |q43| q43 }
[1].each { |q44| q44 }
[1].each { |q45| q45 }
[1].each { |q46| q46 }
[1].each { |q47| q47 }
[1].each { |q48| q48 }
[1].each { |q49| q49 }
[1].each { |q50| q50 }
[1].each { |q51| q51 }
[1].each { |q52| q52 }
[1].each { |q53| q53 }
[1].each { |q54| q54 }
[1].each { |q55| q55 }
[1].each { |q56| q56 }
[1].each { |q57| q57 }
[1].each { |q58| q58 }
[1].each { |q59| q59 }
[1].each { |q60| q60 }
[1].each { |q61| q61 }
[1].each { |q62| q62 }
[1].each { |q63| q63 }
[1].each { |q64| q64 }
[1].each { |q65| q65 }
[1].each { |q66| q66 }
[1].each { |q67| q67 }
[1].each { |q68| q68 }
[1].each { |q69| q69 }
[1].each { |q70| q70 }
[1].each { |q71| q71 }
[1].each { |q72| q72 }
[1].each { |q73| q73 }
[1].each { |q74| q74 }
[1].each { |q75| q75 }
[1].each { |q76| q76 }
[1].each { |q77| q77 }
[1].each { |q78| q78 }
[1].each { |q79| q79 }
[1].each { |q80| q80 }
[1].each { |q81| q81 }
[1].each { |q82| q82 }
[1].each { |q83| q83 }
[1].each { |q84| q84 }
[1].each { |q85| q85 }
[1].each { |q86| q86 }
[1].each { |q87| q87 }
[1].each { |q88| q88 }
[1].each { |q89| q89 }
[1].each { |q90| q90 }
[1].each { |q91| q91 }
[1].each { |q92| q92 }
[1].each { |q93| q93 }
[1].each { |q94| q94 }
[1].each { |q95| q95 }
[1].each { |q96| q96 }
[1].each { |q97| q97 }
[1].each { |q98| q98 }
[1].each { |q99| q99 }
[1].each { |q100| q100 }
[1].each { |q101| q101 }
[1].each { |q102| q102 }
[1].each { |q103| q103 }
[1].each { |q104| q104 }
[1].each { |q105| q105 }
[1].each { |q106| q106 }
[1].each { |q107| q107 }
[1].each { |q108| q108 }
[1].each { |q109| q109 }
[1].each { |q110| q110 }
[1].each { |q111| q111 }
[1].each { |q112| q112 }
[1].each { |q113| q113 }
[1].each { |q114| q114 }
[1].each { |q115| q115 }
[1].each { |q116| q116 }
[1].each { |q117| q117 }
[1].each { |q118| q118 }
[1].each { |q119| q119 }
[1].each { |q120| q120 }
[1].each { |q121| q121 }
[1].each { |q122| q122 }
[1].each { |q123| q123 }
[1].each { |q124| q124 }
[1].each { |q125| q125 }
[1].each { |q126| q126 }
[1].each { |q127| q127 }
[1].each { |q128| q128 }
[1].each { |q129| q129 }
[1].each { |q130| q130 }
[1].each { |q131| q131 }
[1].each { |q132| q132 }
[1].each { |q133| q133 }
[1].each { |q134| q134 }
[1].each { |q135| q135 }
[1].each { |q136| q136 }
[1].each { |q137| q137 }
[1].each { |q138| q138 }
[1].each { |q139| q139 }
[1].each { |q140| q140 }
[1].each { |q141| q141 }
[1].each { |q142| q142 }
[1].each { |q143| q143 }
[1].each { |q144| q144 }
[1].each { |q145| q145 }
[1].each { |q146| q146 }
[1].each { |q147| q147 }
[1].each { |q148| q148 }
[1].each { |q149| q149 }
[1].each { |q150| q150 }
[1].each { |q151| q151 }
[1].each { |q152| q152 }
[1].each { |q153| q153 }
[1].each { |q154| q154 }
[1].each { |q155| q155 }
[1].each { |q156| q156 }
[1].each { |q157| q157 }
[1].each { |q158| q158 }
[1].each { |q159| q159 }
[1].each { |q160| q160 }
[1].each { |q161| q161 }
[1].each { |q162| q162 }
[1].each { |q163| q163 }
[1].each { |q164| q164 }
[1].each { |q165| q165 }
[1].each { |q166| q166 }
[1].each { |q167| q167 }
[1].each { |q168| q168 }
[1].each { |q169| q169 }
[1].each { |q170| q170 }
[1].each { |q171| q171 }
[1].each { |q172| q172 }
[1].each { |q173| q173 }
[1].each { |q174| q174 }
[1].each { |q175| q175 }
[1].each { |q176| q176 }
[1].each { |q177| q177 }
[1].each { |q178| q178 }
[1].each { |q179| q179 }
[1].each { |q180| q180 }
[1].each { |q181| q181 }
[1].each { |q182| q182 }
[1].each { |q183| q183 }
[1].each { |q184| q184 }
[1].each { |q185| q185 }
[1].each { |q186| q186 }
[1].each { |q187| q187 }
[1].each { |q188| q188 }
[1].each { |q189| q189 }
[1].each { |q190| q190 }
[1].each { |q191| q191 }
[1].each { |q192| q192 }
[1].each { |q193| q193 }
[1].each { |q194| q194 }
[1].each { |q195| q195 }
[1].each { |q196| q196 }
[1].each { |q197| q197 }
[1].each { |q198| q198 }
[1].each { |q199| q199 }
[1].each { |q200| q200 }
[1].each { |q201| q201 }
[1].each { |q202| q202 }
[1].each { |q203| q203 }
[1].each { |q204| q204 }
[1].each { |q205| q205 }
[1].each { |q206| q206 }
[1].each { |q207| q207 }
[1].each { |q208| q208 }
[1].each { |q209| q209 }
[1].each { |q210| q210 }
[1].each { |q211| q211 }
[1].each { |q212| q212 }
[1].each { |q213| q213 }
[1].each { |q214| q214 }
[1].each { |q215| q215 }
[1].each { |q216| q216 }
[1].each { |q217| q217 }
[1].each { |q218| q218 }
[1].each { |q219| q219 }
[1].each { |q220| q220 }
[1].each { |q221| q221 }
[1].each { |q222| q222 }
[1].each { |q223| q223 }
[1].each { |q224| q224 }
[1].each { |q225| q225 }
[1].each { |q226| q226 }
[1].each { |q227| q227 }
[1].each { |q228| q228 }
[1].each { |q229| q229 }
[1].each { |q230| q230 }
[1].each { |q231| q231 }
[1].each { |q232| q232 }
[1].each { |q233| q233 }
[1].each { |q234| q234 }
[1].each { |q235| q235 }
[1].each { |q236| q236 }
[1].each { |q237| q237 }
[1].each { |q238| q238 }
[1].each { |q239| q239 }
[1].each { |q240| q240 }
[1].each { |q241| q241 }
[1].each { |q242| q242 }
[1].each { |q243| q243 }
[1].each { |q244| q244 }
[1].each { |q245| q245 }
[1].each { |q246| q246 }
[1].each { |q247| q247 }
[1].each { |q248| q248 }
[1].each { |q249| q249 }
[1].each { |q250| q250 }
[1].each { |q251| q251 }
[1].each { |q252| q252 }
[1].each { |q253| q253 }
[1].each { |q254| q254 }
[1].each { |q255| q255 }
def late
  42
end
late
"#;
    let binary = mrbc_compile("late_def", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 42);
}
