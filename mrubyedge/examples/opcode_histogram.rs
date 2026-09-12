// Rubyのソースを1本コンパイルして、その.mrbが使っているopcodeを数える。
//
//   cargo run --example opcode_histogram -- path/to/a.rb
//
// RITE0400の全命令をテストで踏めているかを測るための計器。**途中でデコードが
// 崩れたらその場で言う。** 黙って数え落とすと、網羅したつもりの穴が残る。
use std::collections::BTreeMap;

use mrubyedge::rite;
use mrubyedge::rite::insn::{FETCH_TABLE, OpCode};

fn main() {
    let path = std::env::args().nth(1).expect("usage: opcode_histogram <file.rb>");
    let code = std::fs::read_to_string(&path).expect("cannot read source");

    let binary = unsafe {
        let mut ctx = mruby_compiler2_sys::MRubyCompiler2Context::new();
        ctx.compile(&code).expect("compile failed")
    };
    let rite = rite::load(&binary).expect("load failed");

    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    let mut broken = 0usize;
    for (i, irep) in rite.irep.iter().enumerate() {
        let ilen = irep.insn.len();
        let mut insns = irep.insn;
        // FETCH_TABLEの関数がopcodeバイトごと消費する(vm.rsのinterpret_insnと同じ)
        while !insns.is_empty() {
            let at = ilen - insns.len();
            let byte = insns[0];
            let Ok(op) = OpCode::try_from(byte) else {
                eprintln!("irep {i} +{at}: byte {byte} is not an opcode");
                broken += 1;
                break;
            };
            if FETCH_TABLE[byte as usize](&mut insns).is_err() {
                eprintln!("irep {i} +{at}: cannot fetch operands of {op:?}");
                broken += 1;
                break;
            }
            *seen.entry(format!("{op:?}")).or_insert(0) += 1;
        }
    }
    for (name, n) in &seen {
        println!("{name}\t{n}");
    }
    eprintln!(
        "ireps={} distinct={} total={} broken={}",
        rite.irep.len(),
        seen.len(),
        seen.values().sum::<usize>(),
        broken
    );
}
