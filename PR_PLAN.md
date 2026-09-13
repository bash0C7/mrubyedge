# PR 再挑戦の対応表

PR #88 はクローズ。udzura さんの指示は「適切な粒度で別ブランチ」「PR #1 は RITE0400
バイトコード対応とその生成コードのテストだけ」「個別の関心事はそれぞれ別 PR で歓迎」
「`445635d` を分割点に」。

**重要な発見: いまブランチに入っているものの大半は RITE0400 を待たなくていい。**
3.3 のバイトコードでも同じく壊れている＝既存の欠陥なので、master の既存テスト基盤
（mec-mrbc-sys）のまま書ける。RITE0400 が入る前に独立してレビュー・マージできる。

出典は `bash0C7/mrubyedge` の `feature/rite0400`（= `3c79470`）。
各 PR は**そこから切り出すのではなく master から組み直す**。切り出すと後ろのコミットが
前に依存する（`PoolValue` の型変更など）ため独立しない。

---

## トラック A — RITE0400 に依存しない。いますぐ出せる

| # | 内容 | 出典コミット | 再現する Ruby |
|---|---|---|---|
| A1 | 呼び出しへの splat 渡し（**PANIC**）。`SEND` の引数個数 15 は「配列 1 個」の意味 | `da5ee82` | `def pair(a,b); a+b; end` / `s=[1,2]` / `pair(*s)` |
| A2 | `yield` のブロック探索（**PANIC**）。`BLKPUSH` の packed オペランドを無視して callinfo を unwrap していた | `da5ee82` | `def relay; [1].each { yield 3 }; end` / `relay { \|x\| x }` |
| A3 | `begin` を出る `return`（**PANIC**）。`RETURN_BLK` は env が無ければ通常 return（`vm40.c:2762`） | `ca5d031` | `def m; begin; return 1; ensure; 2; end; end` |
| A4 | Module の singleton クラス（**PANIC**）。`RClass` にある背後参照が `RModule` に無い | `ca5d031` | `module M; def self.x; 3; end; end` / `M.x` |
| A5 | クラス変数 `GETCV` / `SETCV`。`RModule` に置き場を追加 | `6611f59` | `class K; def s; @@v=1; end; def g; @@v; end; end` |
| A6 | スコープ付き定数 `SETMCNST` | `6611f59` | `class K; end` / `K::X = 5` |
| A7 | 動的シンボル `INTERN` | `6611f59` | `n=1` / `:"a#{n}"` |
| A8 | 配列の splat `ARYPUSH` / `ARYSPLAT` | `6611f59` | `[*a, 3]` / `def m(a); return *a; end` |
| A9 | hash の展開 `HASHADD` / `HASHCAT` | `6611f59` | `{**h, b: 2}` / `{**h, **i}` |
| A10 | 引数なし `super` `ARGARY` + `SUPER` の packed 引数 | `6611f59` | `class B < A; def m(x); super; end; end` |
| A11 | 未実装命令を panic ではなく `Err` に | `e46a3f9` | 実チャンクの `MOVE` のバイトを `GETSV`(23) に差し替え |
| A12 | INT64 の pool をビッグエンディアンで読む | `013f35d` | `n = 4294967296123`（3.3 の mrbc も吐く） |
| A13 | コンテナの堅牢化。`RITE` ident / minor 版 / `binary_size` / 未知セクション / 内部 NUL / 長さ破損の境界検査 | `c5aa5f4` `63e0489` `cd4d60c` | ヘッダ書き換え・末尾追加・2 バイトを 0xFFFF に掃引 |

A1〜A4 は **PANIC 修正**。テストが短く、バグとして明快。**ここから出すのを勧める。**

---

## トラック B — RITE0400 が要る

| # | 内容 | 出典 |
|---|---|---|
| **B1** | **PR #1。** 形式（ヘッダと `04` ゲート）、オペコード表の 4.0 化、追加 13 命令、`ENTER` の n1、catch handler の範囲、4.0 の codegen が踏む例外経路、**コンパイラ依存の差し替え**、docs | `cc9fc36` `a39e968` `ab90d09` `5183476` `6f1ab5c` |
| B2 | `EXT1/2/3` の前置き。udzura さんが「今回の PR では不要かな」と名指し | `445635d` + `3b41593` |
| B3 | bignum の pool レコード長。3.3 の mrbc は `MRB_USE_BIGINT` 無しなので 4.0 でのみ到達 | `4ce88e8` |
| B4 | カバレッジハーネス。彼が欲しがるか不明 | `93a1a0b` `60e864f` `3c79470` |

### B1 の前提条件（先に伝えること）

- **コンパイラ依存の差し替えは B1 から分離できない。** 4.0 を吐くコンパイラが無ければ
  RITE0400 のテストが 1 本も書けない。コミットとして先頭に独立させるのが精一杯
- `mruby-compiler2-sys` が **`bash0C7/mruby-compiler2-sys` の `rite0400-vendor` ブランチ**
  を指している。crates.io に出るまでマージできない。**B1 を出す前に release 見通しの確認が要る**

### B2 の判断材料

**実アプリ（bash0c7-homepage、221,875 バイト / 1130 irep / 95 命令）は EXT を 1 回も踏まない。**
256 レジスタ／シンボルを超えるフレームが無いため。udzura さんの「不要かな」は測定で裏が取れる。

---

## 未着手（どの PR にも入っていない）

`coverage/language.rb` の `#broken` 10 件。`cargo test -- --ignored` で追跡中。

| | 分類の見立て（**未測定**） |
|---|---|
| `def m(a,*b,c)` の post 引数 → `register 3 is not assigned` | bytecode（ENTER） |
| `a,*b,c = x` → `apost with 3 operands is not supported yet` | bytecode（APOST 部分実装） |
| `A::B::V` → NameError | 未分類 |
| `next` が ensure を飛ばす | bytecode（JMPUW / catch handler） |
| `case 2 when String` が真 | stdlib 寄り（`===` の送信） |
| ユーザー定義 `==` に届かない | bytecode（OP_EQ） |
| `!false` → NoMethodError | stdlib 寄り（`!` の送信） |
| `Array#inject` / `Array#reverse` / `Integer#between?` | stdlib |

さらに `raise <例外クラス>` が **stack overflow で process abort**（`prelude/object.rs` の
`mrb_object_raise` が master 由来の `// TODO: accept exception class` のまま）。stdlib 側の最優先。
