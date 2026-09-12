// `def`はふだん1命令(TDEF)に畳まれる。畳めるのはメソッド本体のirepの番号が
// 255までのときで、それを超えるとTCLASS・METHOD・DEFの3命令に分かれる。
//
// **Rubyを直接書く。** ただし256本目のirepまで積むには量が要るので、ソースは
// 組み立てる。
extern crate mrubyedge;

mod helpers;
use helpers::*;

/// メソッドを`count`本持つクラスを書く。1本につきirepが1つ増える。
fn a_class_of(count: usize) -> String {
    let mut code = String::from("class Many\n");
    for i in 0..count {
        code.push_str(&format!("  def m{i}; {i}; end\n"));
    }
    code
}

#[test]
fn a_def_folds_into_one_instruction_test() {
    let code = "
class Few
  def only; 3; end
end
Few.new.only
";

    let result = run_covering(code, &["TDEF"]);
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 3);
}

#[test]
fn a_def_past_the_irep_limit_splits_into_three_test() {
    let mut code = a_class_of(260);
    code.push_str("  def last_one; 7; end\nend\nMany.new.last_one\n");

    let result = run_covering(&code, &["TCLASS", "METHOD", "DEF"]);
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 7);
}

#[test]
fn the_methods_before_the_limit_still_work_test() {
    let mut code = a_class_of(260);
    code.push_str("end\nm = Many.new\n[m.m0, m.m259].join(\",\")\n");

    let result = run(&code);
    let result: String = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, "0,259");
}
