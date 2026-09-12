// begin/rescue/ensureを1本コンパイルして、catch handlerのbegin/end/targetが
// 命令境界に乗っているかを見る。乗っていないと load_irep_1 の index_of が
// code.len() へ落ちる。
use mrubyedge::rite;
use mrubyedge::rite::insn;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("usage: catch_handlers <file.rb>");
    let code = std::fs::read_to_string(&path).expect("cannot read source");
    let binary = unsafe {
        let mut ctx = mruby_compiler2_sys::MRubyCompiler2Context::new();
        ctx.compile(&code).expect("compile failed")
    };
    let rite = rite::load(&binary).expect("load failed");

    for (i, irep) in rite.irep.iter().enumerate() {
        if irep.catch_handlers.is_empty() {
            continue;
        }
        // 命令の開始バイト位置を集める
        let ilen = irep.insn.len();
        let mut starts = Vec::new();
        let mut insns = irep.insn;
        while !insns.is_empty() {
            starts.push(ilen - insns.len());
            if insn::fetch_next(&mut insns).is_err() {
                break;
            }
        }
        println!("irep {i}: ilen={ilen} 命令数={}", starts.len());
        for ch in &irep.catch_handlers {
            let on = |p: usize| {
                if starts.contains(&p) {
                    format!("命令{}", starts.iter().position(|s| *s == p).unwrap())
                } else if p == ilen {
                    "ilen(末尾の1つ先)".to_string()
                } else {
                    "境界に乗っていない".to_string()
                }
            };
            println!(
                "  type={} begin={} [{}] end={} [{}] target={} [{}]",
                ch.type_,
                ch.start,
                on(ch.start),
                ch.end,
                on(ch.end),
                ch.target,
                on(ch.target),
            );
        }
    }
}
