# TODO（人間がやること）

- **PR に補足コメントを入れる。** `mrubyedge/coverage/*.rb` を `mrubyedge/tests/` の外に置いた
  理由を udzura さんに説明する。彼のレビュー原文は「mrubyedge/tests の配下は…Rubyのコードを
  直接記述する」「fixtureを作らないでください」。この 2 本は tests 配下ではなく、
  バイナリでもなく、読める Ruby として単体で走る。テストは実行時に読む。
  ただし「テストが外部ファイルを読む」形であることに変わりはないので、
  意図に沿うか本人に確認したい。**Claude は PR コメントを書かない。**

---

# 戻る場所（2026-09-12・実験を始める前の確定状態）

**PR の確定状態 = `cd4d60c`**（`feature/rite0400`、13 コミット、389 passed / 0 failed、
fmt・clippy・insn-limit すべて green、ツリー clean）

origin 上で 2 本がこれを指している。どちらかが残っていれば戻れる。

- `origin/feature/rite0400`
- `origin/keep/rite0400-pr` ← 実験で汚さないための固定アンカー

ローカルにタグ `rite0400-pr-cd4d60c` もあるが、**このセッションの proxy はタグの push を
拒む**（`send-pack: unexpected disconnect`）。origin にタグは無い。ブランチ 2 本で代替している。

## 戻し方

```sh
git -C /home/user/mrubyedge switch feature/rite0400
git -C /home/user/mrubyedge reset --hard origin/keep/rite0400-pr   # 念のため
git -C /home/user/mrubyedge status --short                          # 空になること
```

実験ブランチは `experiment/ruby-layer`。**`feature/rite0400` には一切コミットしない。**
homepage の `[patch.crates-io]` を実験ブランチへ向けるのも、homepage 側の作業ツリーだけ。

## 実験の目的（ユーザーの指示、一字も変えない）

> まず define_method だけ載せて動かしてみる —— 次の壁が何かを見てから決める
> ただし、これは実験的な実行なので、いつでも今のtreeの場所に立ちかえられるようにしてね。
> gitとしてもclaudeの記録としてもね

---

# HANDOFF — PR #88 (mrubyedge RITE0400)

**読む順: この節（レビュー原文とユーザーの核心）を先に読む。状態や道具はその後。**
同じ内容は `~/.claude/CLAUDE.md` にも置いてあり、毎ターン文脈に注入される。

## udzura さんのレビュー原文（一字も変えない）

> @@ -0,0 +1,67 @@
> +# Reaches every opcode mruby 4.0 added except MATCHERR; compiled into mruby40.mrb, see tests/rite0400.rs.
>
> 「ファイルを見ればわかる」という状態を維持したいので、fixtureを作らないでください。
> mrbのようなバイナリファイルもこれ以上追加しないでください。

> @@ -0,0 +1,791 @@
> +// The opcodes mruby 4.0 (RITE0400) added, as hand-built IREPs, since the harness compiles with mruby 3.3.
>
> mrubyedge/tests の配下は、以下のフォーマットのテスト形式を守ってください。
>
> Rubyのコードを直接記述する。
> そのコードをコンパイルする。
> それをロード、実行する（toplevel、関数呼び出しどちらも許容）。
> 返却値をRustのバリューにして検査する。
> 今回は、対応するバイトコードを出力するRubyコードがテストケースになると思います。
>
> IREPを直接記述するのは原則禁止です。
>
> （例外もある可能性はありますが、それらは古いテストで置き換え予定、またはしかたないエッジケースのはず）

> @@ -0,0 +1,104 @@
> +// The harness compiles with mruby 3.3 (mec-mrbc-sys), so tests/fixtures/mruby40.mrb is checked in:
> +// tests/fixtures/mruby40.rb compiled by mruby-compiler2 (PicoRuby's mrbc). Recompile it whenever
> +// mruby40.rb changes.
> +extern crate mrubyedge;
> +
> +use mrubyedge::rite::insn::{OpCode, RiteVersion};
> +
> +const MRUBY40: &[u8] = include_bytes!("fixtures/mruby40.mrb");
>
> 上のコメントの通り、直接テストコードにRubyコードを記述してください。 include_bytes! は必要ないはずです。

> ドキュメントがないのはすいません（このPRでAIの指示を追加しても構いません）

4 点目は**許可であって依頼ではない**。「かまいません、なので作れ、ではない。」

## ユーザーが伝えた核心（一字も変えない）

> 真の目的、シンプルにRITE0400対応とレビューコメントに100パーセント合致するか

> RITE04000はmruby 4.0 の形式であり対応するのが当然。手抜きの正当化は禁止

> 蛇足なこと考えすぎ。シンプルにって度々いってるだろ。

> ここで破壊的変更も全部含めるよ。メジャーバージョンアップやぞ

> PR文面やPRへのコメントは私の指示を必ず仰げ。

> あとそもそもコメント追加はclaudeの趣味であってわたしとしては不要なんよ

> 期待の振る舞いは正常系、イレギュラー系ともにテストコードで表現してまずfailすることを確認(現行と同じふるまいであればもちろんpassだね) その上でメインのコード修正をするよう徹底

> バージョン番号をあげる権限ないぞ！

> また手段と目的ぎゃくになってない？

> 正直ってなんのことよ？また手抜きの言い訳か？

> 4 ステップ規則ってなにそれ？claudeは自分で書いた謎規則に縛られてて全然ダメ

## 作業規律

- **測る前に範囲を決めない。** H0（「現状は正しく動く」）を立てて構文ごとに棄却しにいく。
  期待値は本物の mruby が返す値を先に宣言する。「元から壊れている」「別レイヤー」は
  測ったあとの分類であって、先に引く線ではない
- **範囲の判定は測定で行う。** 同じプローブを `origin/master`（3.3）でも走らせ、
  両方で同じように壊れるものはこの PR の範囲外
- **テスト先行。** 正常系・イレギュラー系をテストで表現して fail を確認してから直す。
  red ログを scratchpad に残す
- **コメントを足さない。**
- **自分で規則を作って自分で従わない。** 根拠はレビュー原文だけ
- PR 本文・PR コメント・issue の文面は必ずユーザーの指示を仰ぐ。
  Claude は PR も issue も作らない。mrubyedge org には触れない（GitHub MCP は bash0c7/* のみ）
- バージョン番号を上げない
- 履歴を勝手に書き換えない（組み直しを指示されたときだけ、最終ツリーの同一性を
  不変条件にして行う）

## コミット trailer

```
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_01FfoC7uMVr3d3VaxWFnaPFT
```

author は bash0C7 <ksb.4038.nullpointer@gmail.com>。

---

# 状態（2026-09-12）

- `/home/user/mrubyedge`、`feature/rite0400` = `origin/feature/rite0400` = **`445635d`**、
  `origin/master` (`c7dd9ae`) の上に **6 コミット**。tip で
  test / hash-fnv / insn-limit / doc / fmt / clippy / cli build が green。
  tip は **367 passed / 0 failed**（素の `cargo test`）
- **CI の実ゲートは `cargo clippy -p mrubyedge --features "mruby-random,mruby-regexp"`**
  （`.github/workflows/mrubyedge.yml:79`）。`--all-targets -D warnings` はこれより厳しく、
  `tests/object.rs` のリテラ真偽値 assert 6 件で落ちるが、これは master から既存
- 組み直し前の 23 コミットは `backup/rite0400-23commits`（= `b6e5f93`）に退避。
  組み直しの不変条件「`git diff b6e5f93 HEAD` が空」と「54 プローブが完全一致」を確認済み
- fork **bash0C7/mruby-compiler2-sys** の `rite0400-vendor` = `01b77b7`、
  clone は `/home/user/mruby-compiler2-sys`。mrubyedge の dev-dependency はここを向いている
- PR https://github.com/mrubyedge/mrubyedge/pull/88

## コミット列

| | | テスト |
|---|---|---|
| `cc9fc36` | Read and run mruby 4.0 (RITE0400) bytecode | 376 |
| `a39e968` | Refuse a block when ENTER's n1 flag is set | 379 |
| `ab90d09` | Let an exception raised in an ensure clause reach its handler | 380 |
| `5183476` | Stop shipping 3.x chunks the crate can no longer load | 380 |
| `6f1ab5c` | Document mruby 4.0 support | 380 |
| `445635d` | Read the EXT1, EXT2 and EXT3 operand-width prefixes | — |

上 5 行の数字は前セッションが feature 込みのゲートで測った値で、**再測定していない**。
今回素の `cargo test` で測り直したのは 2 点だけ: `6f1ab5c` = **363**、
`445635d` = **367**（差分 +4 は `tests/ext_operands.rs` の新規 4 本のみ、後退ゼロ）。

`cc9fc36` が大きい（1310 行）のは原子性による。4.0 の codegen は普通の Ruby にも
`TDEF` / `SSEND0` / `RET*` を吐くので、テーブル切替と新 opcode 13 個は分けられない。
例外経路の 3 修正（`GETGV $!` で未定義グローバルが nil、catch handler が範囲を持つ、
`RESCUE` が非例外に false）も同じ理由で切り離せない。

## 測定で確定した範囲

54 構文のプローブをブランチ（4.0）と master（3.3、コンパイラは同系列の
mruby-compiler2 0.3.0）で走らせた。**このブランチが壊した挙動は 1 件だけ。**

| | master (3.3) | ブランチ (4.0) |
|---|---|---|
| `while` + `break` + raise する ensure を外側の `rescue` で拾う | `Ok "p:from ensure"` | Rust の `Err` に漏れていた → `ab90d09` で修正 |

両方で同じく壊れている（範囲外、VM の元からの欠陥）: `begin; return; ensure` /
`while` の中の `return` / `begin; return; rescue` / `-> { return 1 }` /
ブロック内から呼んだメソッドの `return` / `each { begin; break; ensure }` /
`each { begin; next; ensure }` / ensure が毎周走る / lambda の arity / `-> { break 1 }` /
ブロック無し `lambda` / `@@v` / `$~` / `F::BAR = 1` / `:"a#{x}"` / `[*a, 3]` / `{**h}` /
引数なし `super`。

分類できない 2 件（規則の前提を満たさない）: `case/in` のハッシュパターン（master は
prism 1.4 でコンパイル不可、ブランチはコンパイルできて `NoMatchingPatternError`。
原因は `deconstruct_keys` が prelude に無いこと）/ ローカル変数 300 個（両側とも
コンパイラ側が落ちる。このため `EXT1-3` は 4.0 のテストで一度も踏まれていない）。

## レビュー 4 点への対応（ツリーの現状）

- fixture を作らない → `tests/fixtures/` は存在しない
- `.mrb` をこれ以上追加しない → 追加なし。読めなくなっていた `examples/{hi,simple}.mrb` を削除
- tests 配下は 4 手順 → `tests/rite0400.rs` の違反 3 本を削除、ローダ検査は `tests/rite/` へ、
  chunk テストは `tests/opcodes40.rs` へ。ファイルは削除
- `include_bytes!` → mrubyedge クレートから消滅
- ドキュメント → 許可であって依頼ではないので `CLAUDE.md` は落とした

## 残作業

**PR 本文と返信の文面だけ。投稿は人間。文面は必ずユーザーの指示を仰ぐ。**

- `scratchpad/PR_BODY.md` — 直す点: 冒頭に fork 依存をマージブロッカーとして置く /
  破壊的変更を全部並べる（3.x 拒否・`Irep::catch_target_pos` → `catch_handlers`・
  `rite::Error` に `UnsupportedVersion`）/ メジャー相当だが `Cargo.toml` は `1.1.12` のままと明記 /
  `the tests, and mrubyedge-cli's compiler, already produce 4.0` は自己矛盾なので削除
  （`mrubyedge-cli/Cargo.toml:17` は `0.3.0` = `03`）/ `every commit passes the same set` を言い直す /
  測定で範囲外にしたものを事実として並べる / `Module#===` の段落を削除
- `scratchpad/PR88_REPLY.md` — 直す点: 点 4 に謝罪と「必要でしたら出します」/
  「既存のコミットは書き換えていません」を撤回（7402f1e → 1b5ae38 で force-push 済み、
  さらに今回 b6e5f93 → 6f1ab5c でも組み直している）/ 「`.mrb` はツリーにありません」は今は真 /
  「しかも片方は deprecated」「バイトコードは中間生成物」を落とす / 「別件」節を削除
- `scratchpad/ISSUE_TRIPLE_EQ.md` はこの PR の経路外（`Object#===` が
  `prelude/object.rs:228` でレシーバ自身と比較していて常に true。master にもある）

## scratchpad の道具

- `probe/`（4.0 側 54 プローブ: `tests/sweep.rs`、`sweep2.rs`）
- `m33/` + `mst/`（3.3 側の対照。`mst` は master の worktree）
- `red_h0_sweep.log`（修正前）/ `after_fix_sweep.log`（修正後）/ `rebuilt_sweep.log`（組み直し後、一致）
- `red_ensure_raise.log`（退行修正のテスト先行 red）
- `PR_DIFF.patch` / `PR_DIFF_src.patch` / `PR_DIFF_tests.patch`
- `PR88_REVIEW.html`（この変更の図表）
