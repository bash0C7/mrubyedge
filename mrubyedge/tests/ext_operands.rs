// EXT1・EXT2・EXT3は、後続の命令のオペランドを16bitへ広げる前置き。
//
// レジスタ・シンボル・定数プールのどれかが255を超えると、コンパイラがこれを
// 挟む。前置きを読めないと、そこから先の命令をまるごと取り違える。
//
// **Rubyを直接書く。** ただし255を超えさせるには量が要るので、ソースは
// 組み立てる。組み立てたものもRubyのソースで、コンパイルして実行して戻り値を
// 検査する形は変わらない。
extern crate mrubyedge;

mod helpers;
use helpers::*;

/// ローカル変数を`count`本置く。レジスタ番号をそのぶん押し上げる。
fn locals(count: usize) -> String {
    (0..count).map(|i| format!("a{i} = {i}\n")).collect()
}

#[test]
fn ext1_widens_the_first_operand_test() {
    // ローカル250本のあとに配列を作ると、要素を並べるレジスタが255を超える
    let mut code = locals(250);
    code.push_str("x = [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9]\n");
    code.push_str("x[9]\n");

    let result = run_covering(&code, &["EXT1"]);
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 9);
}

#[test]
fn ext2_widens_the_second_operand_test() {
    // シンボルを300種類使うと、Syms[b]のbが255を超える
    let mut code = String::from("h = {}\n");
    for i in 0..300 {
        code.push_str(&format!("h[:k{i}] = {i}\n"));
    }
    code.push_str("h[:k299]\n");

    let result = run_covering(&code, &["EXT2"]);
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 299);
}

#[test]
fn ext3_widens_both_operands_test() {
    // レジスタもシンボルも255を超える1つの命令を出す。ローカルでレジスタを
    // 押し上げ、シンボルを300種類作ってから、後ろのほうのシンボルを並べる
    let mut code = locals(250);
    code.push_str("h = {}\n");
    for i in 0..300 {
        code.push_str(&format!("h[:k{i}] = {i}\n"));
    }
    code.push_str("x = [");
    for i in 270..300 {
        code.push_str(&format!(":k{i},"));
    }
    code.push_str("]\nx.size\n");

    let result = run_covering(&code, &["EXT3"]);
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 30);
}

#[test]
fn a_chunk_without_ext_still_reads_test() {
    // 前置きの扱いを足しても、ふつうのコードの読み方は変わらない
    let result = run("a = [5, 6]\na[1]");
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 6);
}
