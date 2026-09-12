//! 実行したopcodeを記録する。テストのときだけ使う計測用で、既定では入らない。
//!
//!   MRUBYEDGE_OPCODE_COVERAGE_DIR=/path cargo test --features opcode-coverage
//!
//! プロセスごとに`<pid>.txt`を書く。テストバイナリが何本に分かれても、後で
//! 全部の和を取れば「どのopcodeを実際に踏んだか」が出る。
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::rite::insn::OpCode;

const N: usize = OpCode::NumberOfOpcode as usize;

static SEEN: [AtomicBool; N] = [const { AtomicBool::new(false) }; N];

pub(crate) fn record(code: OpCode) {
    SEEN[code as usize].store(true, Ordering::Relaxed);
}

/// 踏んだopcodeの名前を1行ずつ書き出す。同じプロセスで何度呼んでもよい。
pub fn flush() {
    let Ok(dir) = std::env::var("MRUBYEDGE_OPCODE_COVERAGE_DIR") else {
        return;
    };
    // ワイヤ順で回す。OpCodeの判別子は宣言順なので、SEENはそちらで引く。
    let mut out = String::new();
    for wire in 0..N {
        let Ok(code) = OpCode::try_from(wire as u8) else {
            continue;
        };
        if SEEN[code as usize].load(Ordering::Relaxed) {
            out.push_str(&format!("{code:?}\n"));
        }
    }

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
