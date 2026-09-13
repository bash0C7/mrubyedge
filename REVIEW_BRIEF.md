# 敵対的レビュー ブリーフ — mrubyedge feature/rite0400

## 真の目的（ユーザーの言葉、一字も変えない）

> 真の目的、シンプルにRITE0400対応とレビューコメントに100パーセント合致するか

> RITE04000はmruby 4.0 の形式であり対応するのが当然。手抜きの正当化は禁止

> 蛇足なこと考えすぎ。シンプルにって度々いってるだろ。

> ここで破壊的変更も全部含めるよ。メジャーバージョンアップやぞ

> あとそもそもコメント追加はclaudeの趣味であってわたしとしては不要なんよ

> バージョン番号をあげる権限ないぞ！

## udzura さん(upstream メンテナ)のレビュー原文、一字も変えない

> 「ファイルを見ればわかる」という状態を維持したいので、fixtureを作らないでください。
> mrbのようなバイナリファイルもこれ以上追加しないでください。

> mrubyedge/tests の配下は、以下のフォーマットのテスト形式を守ってください。
> Rubyのコードを直接記述する。
> そのコードをコンパイルする。
> それをロード、実行する（toplevel、関数呼び出しどちらも許容）。
> 返却値をRustのバリューにして検査する。
> 今回は、対応するバイトコードを出力するRubyコードがテストケースになると思います。
> IREPを直接記述するのは原則禁止です。
> （例外もある可能性はありますが、それらは古いテストで置き換え予定、またはしかたないエッジケースのはず）

> 上のコメントの通り、直接テストコードにRubyコードを記述してください。 include_bytes! は必要ないはずです。

> ドキュメントがないのはすいません（このPRでAIの指示を追加しても構いません）
（4点目は許可であって依頼ではない）

## udzura さんの追加の緩和（X/Twitter）

> 意味論が明確なものだけでも実装されていれば問題ないと考えています
https://x.com/udzura/status/2095519350929461663

→ 意味論が明確な命令はその範囲を**完全に**満たす。そうでないものは not_implemented
として、テスト上も明示的に skip 相当の位置づけにする。

## 対象

- リポジトリ `/home/user/mrubyedge`、ブランチ `feature/rite0400` = `445635d`
- `origin/master` = `c7dd9ae` (v1.1.12) の上に 6 コミット
- PR https://github.com/mrubyedge/mrubyedge/pull/88 (upstream は mrubyedge/mrubyedge、
  このセッションからは読めない)
- 比較用: `git diff c7dd9ae 445635d`

### コミット列
| | |
|---|---|
| `cc9fc36` | Read and run mruby 4.0 (RITE0400) bytecode |
| `a39e968` | Refuse a block when ENTER's n1 flag is set |
| `ab90d09` | Let an exception raised in an ensure clause reach its handler |
| `5183476` | Stop shipping 3.x chunks the crate can no longer load |
| `6f1ab5c` | Document mruby 4.0 support |
| `445635d` | Read the EXT1, EXT2 and EXT3 operand-width prefixes |

## 実アプリ（これが本当の負荷）

`/home/user/bash0c7-homepage` — https://bash0c7-homepage.bash0c7.workers.dev/
Cloudflare Workers 上で mruby/edge が Funicular を動かして SSR している。

- `workers/uzumibi/Cargo.toml` の `[patch.crates-io]` が今は
  `bash0C7/mrubyedge` の `feature/metaprogramming` を指している。
  **これはユーザーが以前に雑に作ったもので、滅ぼす対象。基準ではない。**
- `docs/mrubyedge/` に 14 本のパッチ。`0009-Read-mruby-4.0-bytecode.patch` が RITE0400。
  残り 13 本は別の関心事（メタプログラミング、String、引数、Data/URI/Hash など）
- アプリが踏む命令は PR がカバーすべき全命令より**少ない**。アプリが動くことは
  途中経過であって、PR の完成条件ではない

## 仕様の一次資料（手元にある）

- mruby 4.0 の `ops.h`: `/tmp/claude-0/-home-user-mrubyedge/c1f45c8a-684c-5de1-aaf5-7ae0a3a42b85/scratchpad/ops.h`（119 命令）
- mruby 3.3 の `opcode.h` / `ops.h`:
  `/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/mec-mrbc-sys-3.3.1/vendor/mruby/include/mruby/`
- 4.0 のコンパイラ: `~/.cargo/git/checkouts/mruby-compiler2-sys-*`

## 作業規律（レビューでもこれを基準にせよ）

- **測る前に範囲を決めない。** 「元から壊れている」「別レイヤー」は測ったあとの分類
- **範囲の判定は測定で行う。** 同じプローブを `origin/master` でも走らせ、
  両方で同じように壊れるものはこの PR の範囲外
- **テスト先行。** 正常系・イレギュラー系を fail させてから直す
- **コメントを足さない**
- **自分で規則を作って自分で従わない。** 根拠はレビュー原文と mruby の一次資料だけ
- バージョン番号を上げない
- Claude は PR も issue も作らない。PR 文面はユーザーの指示を仰ぐ

## 出力のしかた

- 断定する前に必ず手を動かして確かめる。`cargo test` / `cargo clippy -p mrubyedge
  --features "mruby-random,mruby-regexp"`（これが CI の実ゲート）/ 実際に Ruby を
  コンパイルして走らせる、まで。**推測で「壊れている」と書かない**
- 指摘ごとに: 事実（コマンドと出力）→ なぜ問題か → 根拠（レビュー原文 or 一次資料の該当箇所）
- **蛇足の指摘は減点。** 「あったほうがいい」ではなく「RITE0400 対応として欠けている」
  「レビュー原文に反している」だけを書く
- ディスクが逼迫しているので `target/` を作りすぎないこと。既存の
  `/home/user/mrubyedge/target` を使い、別ディレクトリでのフルビルドは避ける
