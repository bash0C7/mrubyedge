// テストが実際に踏んだopcodeを集計して、網羅できていないものを挙げる。
//
//   COV=/tmp/cov && rm -rf $COV && mkdir -p $COV
//   MRUBYEDGE_OPCODE_COVERAGE_DIR=$COV cargo test -p mrubyedge \
//     --features opcode-coverage,mruby-regexp,mruby-random
//   MRUBYEDGE_OPCODE_COVERAGE_DIR=$COV cargo run --example coverage_report
//
// 踏めていないものが残っていたら終了コード1。CIの門にできる。
use std::collections::BTreeSet;

use mrubyedge::rite::insn::OpCode;

/// mruby 4.0のcodegenがどちらも吐かないopcode。
///
/// mruby-compiler2(PicoRubyの`mrbc`とこのcrateのテストが使う)とmruby本家の
/// `codegen.c`の両方で`OP_<名前>`の参照が0件だったもの。Rubyのソースからは
/// 到達できないので、網羅の分母から外す。
const NOT_EMITTED: &[(&str, &str)] = &[
    ("CALL", "どちらのcodegenにもOP_CALLの参照が無い"),
    ("SETSV", "$~などはRubyから代入できない"),
    ("ASET", "SETIDXに置き換わっている"),
    ("DEBUG", "通常のコンパイルでは出ない"),
];

fn main() {
    let dir = std::env::var("MRUBYEDGE_OPCODE_COVERAGE_DIR")
        .expect("MRUBYEDGE_OPCODE_COVERAGE_DIR を指定すること");

    let mut covered: BTreeSet<String> = BTreeSet::new();
    let entries = std::fs::read_dir(&dir).expect("カバレッジの出力先が読めない");
    let mut files = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "txt") {
            files += 1;
            let body = std::fs::read_to_string(&path).unwrap_or_default();
            covered.extend(
                body.lines()
                    .map(str::trim)
                    .filter(|l| !l.is_empty())
                    .map(String::from),
            );
        }
    }
    assert!(
        files > 0,
        "{dir} にカバレッジの出力が無い。テストを先に走らせること"
    );

    let all: Vec<String> = (0..OpCode::NumberOfOpcode as usize)
        .filter_map(|wire| OpCode::try_from(wire as u8).ok())
        .map(|code| format!("{code:?}"))
        .collect();

    // 計測器が壊れていたら、そう言う。黙って数字を出さない。
    let unknown: Vec<&String> = covered.iter().filter(|n| !all.contains(n)).collect();
    assert!(
        unknown.is_empty(),
        "表に無い名前が記録されている: {unknown:?}"
    );

    let not_emitted: BTreeSet<&str> = NOT_EMITTED.iter().map(|(n, _)| *n).collect();
    let reachable: Vec<&String> = all
        .iter()
        .filter(|n| !not_emitted.contains(n.as_str()))
        .collect();
    let missing: Vec<&&String> = reachable
        .iter()
        .filter(|n| !covered.contains(**n))
        .collect();

    println!("プロセス {files} 本ぶんを集計");
    println!("  全opcode          {}", all.len());
    println!("  codegenが吐かない {}", not_emitted.len());
    println!("  到達できる        {}", reachable.len());
    println!("  踏んだ            {}", covered.len());
    println!("  踏めていない      {}", missing.len());
    println!();
    for (name, why) in NOT_EMITTED {
        println!("  除外 {name:<10} {why}");
    }
    if !missing.is_empty() {
        println!();
        println!("踏めていない {}件:", missing.len());
        for chunk in missing.chunks(7) {
            println!(
                "  {}",
                chunk
                    .iter()
                    .map(|n| n.as_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
        std::process::exit(1);
    }
    println!();
    println!("到達できる{}件をすべて踏んだ", reachable.len());
}
