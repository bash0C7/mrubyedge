# ローカルへの引き継ぎ（2026-09-16・セッション中断時点）

mrubyedge の RITE0400 対応 PR 再挑戦、Track A（RITE0400 非依存の VM 修正、13→14項目）を進めている。
このセッションは Remote Control 経由で中断。再開時はこの文書から読む。

## まず読む順

1. この文書 — 現況・次にやること
2. `PR_PLAN.md` — トラック A/B の対応表（本体、番号は今もこれが基準）
3. Artifact「Track A 分解図」 https://claude.ai/code/artifact/2f6d24da-455a-4b8b-ab1f-18a86994b119
   （version 5。A1〜A14 それぞれの repro・実測サイズ・到達性・実装状況をグラフィカルにまとめてある）
4. `REVIEW_BRIEF.md` / `PROBLEMS.md` / `REACHABILITY.md` / `CLASSIFICATION.md` — 前々セッションの実測・分類の一次資料
5. Claude Code のメモリ（`mrubyedge-rite0400-track-a.md` / `push_to_fork_ok.md`、このリポジトリのworking directoryで自動的に読み込まれる）

## upstream の状態（もう古い可能性は低いが必ず fetch で確認）

`bash0C7/mrubyedge` の upstream remote から `v2.0.0`（`afff593`）がリリース済み。
内容は Track B1（RITE0400 本体、PR #89/#90、author はユーザー本人が別経路で実装）+ バージョン番号変更（PR #91）のみ。
Track A に関わる新規実装は upstream 側には無い。

ローカル `master` は `v2.0.0` と一致（fast-forward、書き換えなし）。以後 upstream が進んだら同様に
`git fetch upstream --tags && git checkout master && git rebase upstream/master` でよい。

## リポジトリ書き込みの体制（重要・変わっていないはず）

**`bash0C7/mrubyedge` への push・branch 作成・master 操作はこのセッション（Track A 担当）に集約**（2026-09-14、
ユーザー決定、理由は前提ズレ回避）。並行して `bash0c7-homepage-e8` という別 Claude セッションが、
`bash0c7-homepage`（本番 SSR worker）が使っている「動くだけ」の雑な fork 差分（旧 `feature/metaprogramming`）を
upstream PR 候補品質へ分解するプロジェクトを進めている。手法（master から書き直す・1関心事=1PR・到達性を
実測してから範囲決定）はこちらと共通。

運用: 向こうは自分の git worktree（`bash0C7/mrubyedge` と同じ `.git` を共有）でローカル commit まで進め、
origin への push だけこちらに依頼してくる（cross-session message経由）。依頼が来たら:
1. `git log --oneline v2.0.0..<branch>` で commit 数・trailer 有無を確認
2. `git show --stat <branch>` で規模・ファイルを確認
3. `cargo test --manifest-path <worktree>/mrubyedge/Cargo.toml`（featureがあれば有効/無効両方）を**このセッションで独立実行**
4. `cargo fmt --manifest-path <worktree>/mrubyedge/Cargo.toml --check` を確認（**これまで2回、fmt差分でpush保留になっている。周知はしたが徹底されていない**）
5. 問題なければ `git push origin <branch>`

bash0C7-homepage 本体（bash0c7-homepage repo）の作業はそちら側の担当のまま。PR文面・PRコメント・issueは
**必ずユーザー本人が書く**（Claude は書かない、これはどちらのセッションも同じ）。「PRを出す」話自体を
こちらから言い出さない（ユーザーから指示があるまで）。

## Track A の現況（A1〜A14）

詳細と repro・実測サイズは Artifact 参照。ここでは状態だけ：

| # | 内容 | 状態 | 担当・branch |
|---|---|---|---|
| A1 | 呼び出し側 splat 引数の PANIC（SEND の n=15=CALL_MAXARGS） | **実装済み・push済み** | このセッション、`work/a1-splat-call-panic` |
| A2 | yield のブロック探索 PANIC（BLKPUSH） | 未着手 | — |
| A3 | begin を出る return PANIC（RETURN_BLK） | 未着手 | — |
| A4 | Module の singleton クラス PANIC | **実装済み・push済み**（A4単体より範囲が広い：module intern化・`Module#to_s`/`#name`も同居） | bash0c7-homepage側、`module-identity-and-singleton-class` |
| A5 | クラス変数 GETCV/SETCV | 未着手 | — |
| A6 | SETMCNST | 未着手 | — |
| A7 | 動的シンボル INTERN | 未着手 | — |
| A8 | 配列splat ARYPUSH/ARYSPLAT | 未着手 | — |
| A9 | hash展開 HASHADD/HASHCAT | **実装済み・push済み** | bash0c7-homepage側、`implement-hashadd-hashcat` |
| A10 | 引数無しsuper ARGARY+SUPER | 未着手 | — |
| A11 | 未実装命令をpanicでなくErrに | 未着手 | — |
| A12 | INT64 poolのbig-endian読み | 未着手 | — |
| A13 | コンテナの堅牢化（3コミット分割可） | 未着手 | — |
| A14 | optional+restパラメータのレジスタ配置 | 帰属確定・未実装（`fix/op-enter-optional-rest`=`cc03daf`が既にorigin上に存在、これをそのまま採用するかは要検討） | Track A側で拾う（2026-09-14合意） |

## 新規に見つかった、未確認の重複可能性（要確認・最優先でここから見る）

セッション中断の直前、`git branch -vv` で **bash0c7-homepage側が作った、まだ push 依頼が来ていない
local branch/worktree** が複数見つかった（push依頼が来ていないので中身を検証していない）:

- `unwind-break-return-super`（worktree: `mrubyedge-pr-unwind-break-return-super`）—
  **"Unwind OP_RETURN_BLK, OP_BREAK and OP_SUPER the way vm.c does"。A2/A3/A10と重複する可能性が高い。最優先で中身を確認すること。**
- `per-activation-environment`（`mrubyedge-pr-per-activation-environment`）— "Give each activation its own environment"。A2のBLKPUSH/env問題と関係あるかもしれない
- `add-env-and-uri`（`mrubyedge-pr-env-and-uri`）
- `basicobject-and-object-core`（`mrubyedge-pr-basicobject-and-object-core`）
- `data-layer-hash-enumerable-array-exception`（`mrubyedge-pr-hash-enumerable-array-exception`）
- `defined-support`（`mrubyedge-pr-defined-support`）— mruby 4.0 の `defined?` 対応、Track Bと関係あるかも

これらはTrack Aの範囲か、bash0c7-homepage独自の範囲か、A2/A3/A10との重複があるかを、push依頼が来た時点で
（あるいはこちらから先に`git log`/`git show --stat`で）確認する。**push依頼が来る前に先読みして構わない**
（読むだけなら centralization の対象外）。

## push待ち・保留中

- **`add-data-class`（`Data.define`実装、bash0c7-homepage側）: fmt差分2箇所を指摘し修正依頼中、再送待ち。** worktree: `mrubyedge-pr-data-class`。Track Aの範囲外（stdlib/prelude）。

## Track Aの範囲外と判断したもの（bash0c7-homepage独自プロジェクトとして進行）

- `constant-lookup-through-namespace-and-class`（push済み）— 定数解決のnamespace/ancestor chain対応
- `string-sub-gsub-slice-range-and-more`（push済み）— String#sub/gsub/slice(Range)等
- `op-eq-dispatch-honors-user-defined-equals`（push済み、centralization前のpush、そのままでよい扱い）

## 環境の再現

### テストが使うコンパイラ

`mrubyedge/Cargo.toml` の dev-dependency は `mruby-compiler2-sys = "0.5.0"`（crates.io、v2.0.0で解決済み。
以前あった「個人forkのgit branch依存」問題はもう無い）。

### CI のゲート（これと同じコマンドで確認する）

```sh
cargo test --manifest-path mrubyedge/Cargo.toml
cargo fmt --manifest-path mrubyedge/Cargo.toml --check
cargo clippy -p mrubyedge --features "mruby-random,mruby-regexp"   # これがCIの実ゲート
MRUBYEDGE_INSN_LIMIT=10000 cargo test --features insn-limit --test insn_limit
```

Mac では `~/.cargo/bin` を PATH に通す必要がある（`export PATH="$HOME/.cargo/bin:$PATH"`）。
clippy component が無い場合は `rustup component add clippy`。

v2.0.0時点、work/a1-splat-call-panic上で364 passed / 0 failed（全ゲート green）。

## 作業規律（このセッションで効いたもの・前セッションからの継続）

- **PRの単位は「取り込む価値の高いこと」（実Rubyで到達するか・実アプリが動的に使うか）と
  「コード(変更ファイル)のコンパクトさ」の2軸で決める**（2026-09-14、ユーザー明言）。PR_PLAN.mdの行分割は
  前セッションの実測に基づく良い出発点だが、機械的に採用するのではなくこの2軸で都度判断する
- **「実アプリが使用」は静的到達性（バイトコードへの出現）であって動的な実行確認ではない。**
  この2つを混同した実例が本セッション内にもあった（ARGARY/INTERN、bash0c7-homepage側の実測で訂正）
- **代理指標を実測と取り違えない。範囲の線を測定より先に議論で引かない。「masterも壊れている」は範囲外の
  理由にならない。テスト先行、赤を確認してから直す。コメントを足さない。**（前セッションから継続）
- **cherry-pickしない。masterから書き直す。** 旧`feature/rite0400`等の参照ブランチは答え合わせにのみ使う
- **PR文面・PRコメント・issueは必ずユーザー本人が書く。** 「PRを出す」話自体、指示があるまでこちらから言い出さない
- push前に独立検証（cargo test・cargo fmt --check）する。相手の報告を鵜呑みにしない
- 一次資料（mruby本家のvendor `vm.c`等、`~/.cargo/registry/.../mruby-compiler2-sys-0.5.0/` や
  `mec-mrbc-sys-3.3.1/vendor/mruby/mrbc-src/`）で設計を確認してから実装する。A1はこれで
  「展開はSENDでなくENTER側の責務」という本家の設計と一致させた

## 未完（前セッションから持ち越し、Track Aとは別）

- `coverage/language.rb` の `#broken` 10件のbytecode/stdlib分類
- Ruby レイヤーの問題（`Module#define_method`等）— Track Aの範囲外、バイトコード層が片付いてから
