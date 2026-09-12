// Rubyのソースを1本コンパイルして、その.mrbが使っているopcodeを数える。
//
//   cargo run --example opcode_histogram -- path/to/a.rb
//
// RITE0400の全命令をテストで踏めているかを測るための計器。**途中でデコードが
// 崩れたらその場で言う。** 黙って数え落とすと、網羅したつもりの穴が残る。
use std::collections::BTreeMap;

use mrubyedge::rite;
use mrubyedge::rite::insn;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("usage: opcode_histogram <file.rb>");
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
        while !insns.is_empty() {
            let at = ilen - insns.len();
            match insn::fetch_next(&mut insns) {
                Ok((op, _, ext)) => {
                    // EXTの前置きも1件として数える。命令の読み方を変えるので、
                    // 「そのチャンクが前置きを使った」は見えているほうがよい。
                    match ext {
                        1 => *seen.entry("EXT1".to_string()).or_insert(0) += 1,
                        2 => *seen.entry("EXT2".to_string()).or_insert(0) += 1,
                        3 => *seen.entry("EXT3".to_string()).or_insert(0) += 1,
                        _ => {}
                    }
                    *seen.entry(format!("{op:?}")).or_insert(0) += 1;
                }
                Err(e) => {
                    eprintln!("irep {i} +{at}: デコードできない ({e:?})");
                    broken += 1;
                    break;
                }
            }
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
