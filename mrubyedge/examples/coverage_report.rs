// Reports which opcodes the test suite actually executed.
//
//   COV=/tmp/cov && rm -rf $COV && mkdir -p $COV
//   MRUBYEDGE_OPCODE_COVERAGE_DIR=$COV cargo test -p mrubyedge \
//     --features opcode-coverage,mruby-regexp,mruby-random
//   MRUBYEDGE_OPCODE_COVERAGE_DIR=$COV cargo run --example coverage_report \
//     --features opcode-coverage
use std::collections::BTreeSet;

use mrubyedge::rite::insn::OpCode;

/// Opcodes no Ruby source reaches with the compiler this crate's tests use.
/// They are implemented; nothing compiles to them, so no test runs them.
const NO_RUBY_REACHES: &[(&str, &str)] = &[
    ("CALL", "neither codegen references OP_CALL"),
    ("SETSV", "a special variable cannot be assigned from Ruby"),
    ("ASET", "SETIDX took its place"),
    ("DEBUG", "not emitted by an ordinary compile"),
    ("GETSV", "$~ $& $1 $` $' $+ all compile to GETGV"),
    ("SYMBOL", "a peephole on a node prism does not build"),
    (
        "ERR",
        "prism rejects the bare break, redo and retry it comes from",
    ),
    (
        "STOP",
        "emitted, but the top-level RETURN leaves the run loop first",
    ),
];

fn main() {
    let dir =
        std::env::var("MRUBYEDGE_OPCODE_COVERAGE_DIR").expect("set MRUBYEDGE_OPCODE_COVERAGE_DIR");

    let mut covered: BTreeSet<String> = BTreeSet::new();
    let entries = std::fs::read_dir(&dir).expect("cannot read the coverage directory");
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
    assert!(files > 0, "{dir} holds no coverage; run the tests first");

    let all: Vec<String> = (0..OpCode::NumberOfOpcode as usize)
        .filter_map(|wire| OpCode::try_from(wire as u8).ok())
        .map(|code| format!("{code:?}"))
        .collect();

    let unknown: Vec<&String> = covered.iter().filter(|n| !all.contains(n)).collect();
    assert!(
        unknown.is_empty(),
        "recorded a name not in the table: {unknown:?}"
    );

    let missing: Vec<&String> = all.iter().filter(|n| !covered.contains(*n)).collect();

    println!("collected from {files} processes");
    println!("  opcodes      {}", all.len());
    println!("  run by tests {}", covered.len());
    println!("  never run    {}", missing.len());

    let mut unexplained = Vec::new();
    if !missing.is_empty() {
        println!();
        for name in &missing {
            match NO_RUBY_REACHES.iter().find(|(n, _)| *n == name.as_str()) {
                Some((_, why)) => println!("  {name:<8} {why}"),
                None => {
                    println!("  {name:<8} no Ruby written for it yet");
                    unexplained.push(name.as_str());
                }
            }
        }
    }
    if !unexplained.is_empty() {
        println!();
        println!("{} of those have no reason recorded", unexplained.len());
        std::process::exit(1);
    }
}
