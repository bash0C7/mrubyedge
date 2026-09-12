// Rubyのファイルを1本コンパイルして走らせ、戻り値を出す。
//
//   cargo run --example run_file -- path/to/a.rb
//
// テストに載せる前のRubyを試すための道具。
fn main() {
    let path = std::env::args().nth(1).expect("usage: run_file <file.rb>");
    let code = std::fs::read_to_string(&path).expect("cannot read source");
    let binary = unsafe {
        let mut ctx = mruby_compiler2_sys::MRubyCompiler2Context::new();
        ctx.compile(&code).expect("compile failed")
    };
    let mut rite = match mrubyedge::rite::load(&binary) {
        Ok(rite) => rite,
        Err(e) => {
            println!("LOAD ERROR: {e:?}");
            return;
        }
    };
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    match vm.run() {
        Ok(v) => println!("OK {v:?}"),
        Err(e) => println!("RUN ERROR: {e}"),
    }
}
