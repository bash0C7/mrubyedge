# mruby 4.0 の言語要素サーベイ（feature/rite0400 = ca5d031）

命令サーベイ（`CLASSIFICATION.md`）の兄弟。**119 命令の表が「実装の有無」しか示さないのに対し、
こちらは「挙動が正しいか」を測る。** 実際、この 1 本で bytecode 層のバグが 3 件出た。

- 82 ケース。Ruby を直書き → mruby 4.0 でコンパイル → ロード・実行 → Rust の値で検査
- **1 ケース 1 プロセス**で走らせる。stack overflow は abort なので `catch_unwind` で拾えず、
  1 プロセスに詰めると以降が全部測れなくなる（最初の版はそれで 19 件目で止まった）
- 生の結果は `lang_survey.txt`
- 再現: `mrubyedge/tests/zzme_lang.rs`（作業ツリーには残さない）を置き、
  `LANG_CASE=<n> ./target/debug/deps/zzme_lang-*`

## 結果

| | 件数 |
|---|---|
| 期待どおり | 73 |
| **bytecode 層のバグ** | **3** |
| stdlib の穴 | 4 |
| 既知の制限（文書済み） | 2 |

## bytecode 層のバグ — この PR の範囲

| ケース | Ruby | 症状 |
|---|---|---|
| post arg | `def m(a, *b, c); a + c; end` + `m(1,2,3,4)` | `Internal("register 3 is not assigned")`。`ENTER` の post 引数 |
| splat call | `def m(a, b); a + b; end` + `args = [1,2]` + `m(*args)` | **PANIC**（`optable.rs:1313`）。`SEND` の `n=15`（packed args）を `do_send` が扱わない。`SUPER` 側だけ直してあった |
| double splat call | `def m(a:, b:); a + b; end` + `h = {a:1, b:2}` + `m(**h)` | `TypeMismatch`。キーワード引数の packed 渡し |

3 件とも引数渡し。`ENTER` と `SEND` の operand 解釈なので bytecode 層。

## stdlib の穴 — 別セッション

| ケース | 症状 | 原因 |
|---|---|---|
| `raise ArgumentError, 'b'` | **stack overflow で process abort** | `mrb_object_raise`（`prelude/object.rs:339`）が `// TODO: accept exception class` のまま。クラスを文字列へ変換しようとして再帰。**master 由来** |
| `begin; raise TypeError; rescue ArgumentError; ...; rescue TypeError` | 同上 | 同上 |
| `"abc".frozen?` | `NoMethodError` | `String#frozen?` が無い |
| `defined?(String)` | `NoMethodError: __defined_const?` | 4.0 は `defined?` をこの呼び出しにコンパイルする。受け手が無い |

`raise <例外クラス>` は**どの形でも落ちる**（`raise C, 'm'` / `raise C.new('m')` / `raise C`）。
`raise 'string'` だけが通る。`ArgumentError.new('b')` も `RValue::Exception` にならない。
派生セッションの最優先候補。

## 既知の制限（`COVERAGE.md` に記載済み・新規ではない）

| ケース | 結果 |
|---|---|
| `case x in [a, b]` | 束縛されず else へ |
| `case x in {a: v}` | 同上 |

`deconstruct` / `deconstruct_keys` が無いため。値パターンのみ通る。

## 通った 73 件（抜粋）

制御構造（`if`/`unless`/`case when`/`while`/`until`/`for`/`loop`/`break`/`next`/`retry`/
`begin rescue else ensure`）、メソッド（optional・rest・keyword・`**opts`・block・`yield`・
`block_given?`・`&:sym`・lambda・`l.()`・`Proc.new`）、クラス（継承・`super`・module の
`self.` メソッド・`include`・`extend`・`attr_accessor`・クラス変数・定数・`class << self`・
`method_missing`・`alias`・`to_s` と `==` の上書き）、リテラル（式展開・heredoc・動的シンボル・
splat・`**` 展開・range・多重代入・`||=`・添字代入・巨大整数）。

## この道具の使い方

命令の表が埋まっても「実装済みだが間違っている」は残る。今日それで
`RETURN_BLK` と `SDEF` が出て、このサーベイでさらに 3 件出た。
**アプリを走らせる前に、言語要素を先に潰すほうが速い。**
