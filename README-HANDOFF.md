# ローカルへの引き継ぎ（2026-09-13）

mrubyedge の RITE0400 対応。PR #88 はクローズ済み、ここから作り直す。

## まず読む順

1. `PR_PLAN.md` — **これが本体。** どの変更をどの PR にするかの対応表
2. `REVIEW_BRIEF.md` — udzura さんのレビュー原文とユーザーの核心の言葉（一字も変えていない）
3. `PROBLEMS.md` — 洗い出した問題 22 件の全リストと、Claude 自身の思考プロセスの失敗パターン
4. `CLASSIFICATION.md` — 119 命令の確定分類
5. `LANG_SURVEY.md` — 言語要素 82 ケースのサーベイ結果

## ブランチ（すべて bash0C7/mrubyedge に push 済み）

| | |
|---|---|
| `feature/rite0400` = `3c79470` | 全部入り。**参照用。PR #88 でクローズされたもの** |
| `keep/rite0400-pr` = `cd4d60c` | 実験を始める前の固定アンカー |
| `experiment/ruby-layer` = `4d7be58` | `define_method` を載せた実験。Module singleton の修正が PR 側と重複しているので載せ直しが要る |
| `backup/metaprogramming-full` = `38212af` | 旧「雑」ブランチの全量。Ruby レイヤー 2,761 行の出典 |
| `backup/rite0400-23commits` = `b6e5f93` | 組み直し前の 23 コミット |

`origin/master` = `c7dd9ae`（v1.1.12）。

## 環境の再現

### テストが使うコンパイラ

`mrubyedge/Cargo.toml` の dev-dependency が
`git = "https://github.com/bash0C7/mruby-compiler2-sys", branch = "rite0400-vendor"` を指す。
**crates.io に出るまでこのままではマージできない。**

### CI のゲート（これと同じコマンドで確認する）

```sh
cargo test --manifest-path mrubyedge/Cargo.toml
cargo fmt --manifest-path mrubyedge/Cargo.toml --check
cargo clippy -p mrubyedge --features "mruby-random,mruby-regexp"   # これが CI の実ゲート
MRUBYEDGE_INSN_LIMIT=10000 cargo test --features insn-limit --test insn_limit
```

`--all-targets -D warnings` は CI より厳しく、`tests/object.rs` のリテラル真偽値 assert 6 件で
落ちる。これは master から既存。

`3c79470` 時点で **403 passed / 0 failed / 10 ignored**。

### 実アプリ（bash0c7-homepage）を VM に載せる手順

1. PicoRuby 4.0.3 を取る
   `git clone --depth 1 --branch 4.0.3 https://github.com/picoruby/picoruby.git`
2. Funicular の submodule を初期化
   `git -C picoruby submodule update --init --depth 1 mrbgems/picoruby-funicular`
3. `workers/uzumibi/build.rs` と同じ 39 ソースを同じ順で `mrbc.js` に食わせる
   （Funicular の mrblib を `MRBLIB_LOAD_ORDER` 順 → `layout.rb` → `ssr.rb` →
   `app/funicular/{models,stores,components}` 名前順 → `initializer.rb` →
   `hearts.rb` → `pages.rb`）
   `node picoruby/mrbgems/picoruby-funicular/lib/funicular/vendor/mrbc/mrbc.js -o app.mrb <39 files>`
4. 出来た `app.mrb`（221,875 バイト、先頭 8 バイトが `RITE0400`）を `rite::load` → `VM::run`

この手順で作った `app.mrb` は `app.mrb` として同梱してある。

### 測定データ

| | |
|---|---|
| `app_opcodes.txt` | 実アプリが踏む 95 命令 |
| `app_sent.txt` | 実アプリのバイトコードが送るメソッド名 720 件 |
| `lang_survey.txt` / `lang_measured.tsv` | 言語要素サーベイの生結果 |
| `sections_measured.tsv` | `coverage/opcodes.rb` 51 セクションの命令集合と値 |
| `ops.h` | mruby 4.0 の 119 命令定義（vendor の `mrc_ops.h` と md5 一致） |

## いま分かっている事実（測り直さなくていいもの）

- 119 命令中 **112 が実 Ruby で到達可能**、7 が到達不能
  （`GETSV` `SETSV` `CALL` `ASET` `SYMBOL` `DEBUG` `ERR`）
- **実アプリは EXT1/2/3 を 1 回も踏まない**
- 実アプリは形式層を完全に通過する（1130 irep、デコード失敗 0）。
  止まるのは `Module#define_method` が無いところ＝Ruby レイヤー
- `raise <例外クラス>` はどの形でも **stack overflow で process abort**
- master + 3.3 でも壊れているもの（＝この PR の退行ではない）:
  `return` in begin/ensure・while・rescue / `module M; def self.x` /
  class 変数 / `SETMCNST` / splat / `**` 展開 / zsuper / bignum の pool 長

## 作業規律（このセッションで効いたもの）

- **代理指標を実測と取り違えない。** 「`nregs > 255` だから EXT3 が出るはず」で 2 回失敗した。
  目的の性質そのもの（前置きバイトが在るか）を数える
- **範囲の線を測定より先に議論で引かない。** 「形式 vs 意味論」で 9 命令を外して、
  実アプリが 3 つ使っていて撤回した
- **「master も壊れている」は範囲外の理由にならない。** 基準は「RITE0400 に必要か」
- テスト先行。赤を確認してから直す
- **コメントを足さない**
- PR 文面・PR コメント・issue は必ず人間が書く

## 未完

- `coverage/language.rb` の `#broken` 10 件の bytecode / stdlib 分類（`PR_PLAN.md` 末尾）
- `experiment/ruby-layer` の載せ直し（Module singleton が重複）
- Ruby レイヤー 50 メソッドの別セッション化
