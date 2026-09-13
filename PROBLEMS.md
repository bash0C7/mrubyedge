# 問題の洗い出し（feature/rite0400 = 445635d 時点）

## 範囲（ユーザーが引いた線・2026-09-12）

> Module#define_method のようなRubyレイヤーの問題は、すまんが別にしよう。
> まずRITO0400 mruby4.0バイトコードだけやる。
> そうしたらhomepageで必要なrubyのクラスやメソッド不足が次に明確になるよね。

- **やる**: A（形式層の読み）、B（命令の実装）、C（テストとハーネス）、D（規律・蛇足）
- **やらない**: E-2 / E-3（Ruby のクラス・メソッド不足）。バイトコード層が完全になった時点で
  残る失敗は全部 Ruby レイヤーだと一意に読めるようになる。それが次の仕事
- E-1（形式層が実アプリを通すか）と E-4（worker の lock）は測定・事実として残す

すべて実測。未確認の推測には「未測定」と書く。
**この文書の目的は root cause と思考プロセスの問い直しであって、修正手順書ではない。**

---

## A. RITE0400 の形式層

一次資料: mruby 4.0 の `mrc_dump.h` / `mrc_irep.h` / `mrc_ops.h` / `dump.c`（vendor `01b77b7`）、
`load40.c` / `vm40.c` / `codegen40.c`、mruby 3.3 の `dump.h` / `irep.h` / `proc.h` / `opcode.h`。

### A-1. BIGINT の pool レコード長を 2 バイト BE で読んでいる（正しくは 1 バイト + 2）
- 事実: `x = 123456789012345678901234567890` の 1 行で `rite::load` が **panic**
  （`rite.rs:250` `range end index 7732 out of range for slice of length 118`）。
  `Result` を返す API が panic する。
- 一次資料: `dump.c:216-221` `len = (unsigned char)str[0]; memcpy(cur, str, len+2)`、
  `load40.c:266-276` `pool_data_len = bin_to_uint8(src) + 2`
- 範囲: master + compiler2 0.3.0 (RITE0300) でも同じく **PANIC**。VM の退行ではない。
  ただし master のテストが使う mec-mrbc-sys 3.3.1 は `MRB_USE_BIGINT` を定義しないので
  **これまで一度も踏めなかった**。コンパイラを compiler2 に替えたこの PR で初めて到達可能になる。
- 状態: **未コミットで修正済み**（保留中）

### A-2. `RITE` ident を検証していない
- 事実: ident を `ZITE` に書き換えたチャンクが `load` を通る
- 一次資料: `load40.c:656` が `memcmp(binary_ident, RITE_BINARY_IDENT, 4)` で弾く
- 範囲: master も同じコード

### A-3. `minor_version` を検証していない
- 事実: minor を `99` にしたチャンクが通る
- 一次資料: `load40.c:665` は minor が現行より大きければ拒否（上位互換の規約）
- 範囲: master も同じ

### A-4. `binary_size` を使っていない
- 事実: 末尾に 8 バイト足したチャンクが `InvalidFormat` になる
- 一次資料: `load40.c:670-676` はヘッダの size でセクション走査を閉じる
- `rite.rs:97` に `// let binsize` とコメントアウトされた痕跡がある
- 範囲: master も同じ

### A-5. 未知セクションでエラーにし、かつバッファを stderr に吐く
- 事実: 未知セクションを挿むと `InvalidFormat` + `eprintln!` でバイト列がそのまま出る
- 一次資料: `load40.c:720` は未知セクションで**ループを抜けて読めた分を返す**（エラーにしない）
- 範囲: master も同じ（`rite.rs:126-129`）

### A-6. INT64 のエンディアンが逆
- 事実: `4294967296123` が `8863084070557646848` になる
- 一次資料: `dump.c` は hi/lo の順で BE u32 を 2 つ、`load40.c` も同じ順で読む。
  `rite.rs:232` は `i64::from_le_bytes`
- 範囲: master も同じ。master の `test_rite_parse_pool_values` は `d = 9999999999` で
  **型だけ assert して値を見ていない**ので通っていた

### A-7. pool 文字列・シンボルに NUL が埋まっていると読めない
- 事実: `"a\0b".bytesize` が `InvalidFormat`
- 一次資料: mruby は長さで読む。`rite.rs:229/266` は `CStr::from_bytes_with_nul`
- 範囲: master も同じ

### A-8. 空シンボル（`MRB_DUMP_NULL_SYM_LEN` = 0xFFFF）未対応
- 一次資料: `dump.c` は sym が 0 のとき 0xFFFF だけ書く。`rite.rs:266` は `be16 + 1` で読みに行く
- **未測定**: 8 パターンの Ruby で空シンボルを出せなかった。到達 Ruby 未発見

---

## B. 命令の実装

### B-1. 119 命令のうち 14 が未実装
```
GETSV SETSV GETCV SETCV SETMCNST ARGARY ARYPUSH ARYSPLAT
ASET  INTERN HASHADD HASHCAT DEBUG ERR
```
（腕が無いのは 20 だが、`LOADI8`/`LOADTRUE`/`LOADFALSE` は旧名 `LOADI`/`LOADT`/`LOADF` で
生きており、`EXT1/2/3` はデコーダ側で処理される）

- 範囲: master + 3.3 対照でも `C1 class variable` `C3 SETMCNST` `C6 array splat`
  `C7 hash double splat` `C11 zsuper` はすべて PANIC。**元から無い**
- **うち実アプリが使うのは `ARGARY` `INTERN` `HASHCAT` の 3**

### B-2. 未実装命令に当たると `unimplemented!` で panic する
- `optable.rs:532` `unimplemented!("{:?}: Not supported yet", code)`
- `Result` を返す関数の中で panic する。A-1 と同じ形の問題

---

## C. テストの問題

### C-1. `ext3_widens_both_operands_test` が EXT3 を 1 つも出していない
- 事実: そのテストの Ruby を実際にコンパイルして前置きを数えると
  `EXT1=6 EXT2=45 EXT3=0`
- **私が書いたテスト。** `nregs > 255` と `nsyms > 255` という**代理指標**で
  「EXT3 を強制した」と判断し、実際の前置きバイトを数えなかった
- EXT3 自体は到達可能（ivar 320 + ローカル 250 で出る）で、実装は正しい

### C-2. `test_rite_parse_pool_values` が pool の値を検査していない
- 型（`has_int64` など）だけ assert している。A-6 が master で見逃されていた原因

### C-3. `tests/rite/rite.rs` は VM を走らせない
- 4 件とも `load` のパース結果だけ見る。レビュー原文の 4 手順の後半 2 つを踏まない
- 3 件は master 由来、1 件（バージョン拒否）はこの PR の追加

### C-4. `tests/module.rs:154` が Ruby を一切コンパイルせず Rust から直叩き
- master 由来

### C-5. 命令カバレッジを証明する仕組みが無い
- 119 命令のうちテストが実際に踏んでいるものが何かを示す手段が現状ゼロ

---

## D. 規律違反・蛇足

### D-1. コメントを 19 行足している
- src 14 行 + tests 5 行（`optable.rs:124-141` の 18 行は master の既存書式の維持なので除く）
- ユーザーの明言:「コメント追加はclaudeの趣味であってわたしとしては不要なんよ」

### D-2. `docs/table.html:173` が存在しないシンボルを指している
- `// 並び順はsrc/rite/insn.rsのENUM_TABLE / FETCH_TABLEのとおり。`
- `FETCH_TABLE` は私の EXT コミットが削除。master は 1 行の
  `// オペコード名, 引数タイプ, 日本語説明のリスト` だった
- **足さなければ間違えようがなかった行**

### D-3. `COVERAGE.md:20` の `| tests |` 行
- 「A list of currently supported classes and methods」に自分のテスト構成の説明を書いている

### D-4. `Cargo.toml:25-26` のコメント 2 行

### D-5. `lib.rs:16` の doctest から `no_run` を外した
- RITE0400 対応と無関係

### D-6. `Cargo.lock` の itertools 揺れ
- dev-dependency 差し替えの副作用。意図した変更ではない

### D-7. dev-dependency が個人 fork の git ブランチを指している
- `bash0C7/mruby-compiler2-sys` branch `rite0400-vendor`
- upstream PR に個人ブランチ依存が入る

---

## E. 実アプリ（bash0c7-homepage）

### E-1. 形式層は通過する
- 221,875 バイト / 1130 irep / 95 命令、**デコード失敗 0**
- 4.0 が追加した 13 命令のうち 12 を実際に踏む（`MATCHERR` 以外）
- EXT1/2/3 は 1 度も踏まない

### E-2. 実行は `Module#define_method` が無くて止まる
- `NoMethodError("undefined method 'define_method' for Module")`
- master + 3.3 バイトコードでも同じ所で止まる（メソッドが無いだけ）

### E-3. その先に何個あるかは未測定
- 静的にバイトコードから送信メソッド名 720 個を抜いて prelude と突き合わせたが、
  DOM/JS 名・実行時生成メソッド・`method_missing` 経由が混ざって**ノイズが多すぎ**、
  数として使えない。動的に 1 個ずつ潰して数えるしかない

### E-4. worker の `Cargo.lock` が `feature/metaprogramming` を `a92a15f` に固定している
- ブランチ先端は `73820a5` に動いており `mruby-uri` feature が無い
- `cargo update` した瞬間に壊れる

---

## F. 私の思考プロセスの問題

### F-1. 代理指標を実測と取り違える（再発）
- C-1: `nregs > 255` で「EXT3 を強制した」と判断。実際の前置きバイトを数えなかった
- 同じ誤りを EXT1 でも 1 度やっている（`nregs=256` なら最大レジスタ番号は 255 で
  8bit に収まる、を見落とした）
- **測っているのは目的の性質そのものか、それに相関する別のものか**を毎回問うていない

### F-2. 範囲の線を測定より先に、議論で引く（再発）
- 「形式 vs 意味論」で 9 命令を範囲外にした → 実アプリが 3 つ使っていた
- 「RITE0400 の外」→ 線を引くのはユーザーの仕事
- 規律に「測る前に範囲を決めない」と自分で書いておきながら、
  **命令の分類という形で範囲を先に決めていた**

### F-3. 相手が知らない語彙を、確立せずに略語として使う
- 「patch 0001」——ユーザー自身のリポジトリのファイル名を、共有された概念のように使った

### F-4. 見つけたら即座に直す（場当たり）
- A-1 を見つけた瞬間に修正に入った。洗い出しを終える前に手を動かした

### F-5. ノイズの多い出力を答えとして出しかけた
- E-3 の 200 名リスト

### F-6. 他人の測定結果を引用するとき、条件が同じか確認しない
- 監査が挙げた「退行の疑い」は別ハーネスの別テストだった（これは測り直して否定できた）

---

## 未完

- 敵対的レビュー C（119 命令ハーネスの調査と設計）は実行中
