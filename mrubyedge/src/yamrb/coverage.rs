//! 実行したopcodeを記録する。テストのときだけ使う計測用で、既定では入らない。
//!
//!   MRUBYEDGE_OPCODE_COVERAGE_DIR=/path cargo test --features opcode-coverage
//!
//! 記録は2本立て。
//!
//! - プロセス全体の和。`flush`が`<pid>.txt`へ書く。テストバイナリが何本に
//!   分かれても、後で全部の和を取れば「どのopcodeを実際に踏んだか」が出る
//! - スレッドごとの記録。テストは1プロセスの中を複数スレッドで走るので、
//!   1つのテストが踏んだものだけを見るにはこちらを使う
use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::rite::insn::OpCode;

const N: usize = OpCode::NumberOfOpcode as usize;

static SEEN: [AtomicBool; N] = [const { AtomicBool::new(false) }; N];

thread_local! {
    static LOCAL: RefCell<[bool; N]> = const { RefCell::new([false; N]) };
}

pub(crate) fn record(code: OpCode) {
    SEEN[code as usize].store(true, Ordering::Relaxed);
    LOCAL.with(|l| l.borrow_mut()[code as usize] = true);
}

/// このスレッドの記録を捨てる。1回のRubyを走らせる直前に呼ぶ。
pub fn reset_local() {
    LOCAL.with(|l| *l.borrow_mut() = [false; N]);
}

/// `reset_local`以降にこのスレッドが踏んだopcodeを、ワイヤ順の名前で返す。
pub fn local_seen() -> Vec<String> {
    let flags = LOCAL.with(|l| *l.borrow());
    names_of(&flags)
}

/// 全opcodeの名前をワイヤ順で返す。網羅の分母を数えるのに使う。
pub fn all_names() -> Vec<String> {
    names_of(&[true; N])
}

fn names_of(flags: &[bool; N]) -> Vec<String> {
    // ワイヤ順で回す。OpCodeの判別子は宣言順なので、flagsはそちらで引く。
    let mut out = Vec::new();
    for wire in 0..N {
        let Ok(code) = OpCode::try_from(wire as u8) else {
            continue;
        };
        if flags[code as usize] {
            out.push(format!("{code:?}"));
        }
    }
    out
}

/// 踏んだopcodeの名前を1行ずつ書き出す。同じプロセスで何度呼んでもよい。
pub fn flush() {
    let Ok(dir) = std::env::var("MRUBYEDGE_OPCODE_COVERAGE_DIR") else {
        return;
    };
    let mut flags = [false; N];
    for (i, seen) in SEEN.iter().enumerate() {
        flags[i] = seen.load(Ordering::Relaxed);
    }
    let out = names_of(&flags).join("\n") + "\n";

    // テストは1プロセスの中を複数スレッドで走る。truncateして書くのを並行で
    // やると中身が混ざる(ARRAYの尻尾のRAYが1行として残る、という形で出た)。
    // ロックを取り、一時ファイルへ書いてからrenameする。
    static WRITING: Mutex<()> = Mutex::new(());
    let _guard = WRITING.lock();
    let dir = std::path::Path::new(&dir);
    let _ = std::fs::create_dir_all(dir);
    let tmp = dir.join(format!("{}.tmp", std::process::id()));
    if std::fs::write(&tmp, out).is_ok() {
        let _ = std::fs::rename(&tmp, dir.join(format!("{}.txt", std::process::id())));
    }
}
