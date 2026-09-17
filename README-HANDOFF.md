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
| A3 | begin を出る return PANIC（RETURN_BLK） | **実装済み・push済み**（repro直接確認済み。ただしOP_BREAK unwind先バグ・`mrb_funcall`経由superバグの2件が同じcommitに同居、新規発見として別報告） | bash0c7-homepage側、`unwind-break-return-super` |
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

## 未検証branchの分類状況（2026-09-18更新）

前回中断時に見つけた未push branchを`git log`/`git show --stat`、一部は実際にrepro再テストして分類した:

- **`unwind-break-return-super` → 確認済み。A3(RETURN_BLK)を含む。push済み。** ただし同じ1commitに
  OP_BREAKのunwind先バグ（`[1,2,3].each{break}`が呼び出し元methodへ抜ける）と`mrb_funcall`経由
  （`Class#new`→`initialize`）でsuperが呼べないバグが同居。**この2つはTrack Aのどの項目にも無い新規発見。**
  Track A追加候補としてbash0c7-homepage-41へ報告済み、user判断待ち（番号は未確定）
- **`per-activation-environment` → 未push。** "Give each activation its own environment"。
  環境がIREP idでファイルされ再帰呼び出しで上書きされるバグ。RITE0400非依存のVM正当性バグで
  **Track A候補**（A2のBLKPUSH問題とは別原因と見ている、要検証）
- **`defined-support` → 未push。** mruby4.0の`__defined_*?`呼び出し対応（3.3は別方式でinline）。
  **RITE0400依存なのでTrack B候補、Track Aではない。**
- **`add-env-and-uri` / `basicobject-and-object-core` / `data-layer-hash-enumerable-array-exception`
  → 未push。** stdlib/prelude層のメソッド追加（ENV/URI、BasicObject階層、Hash等）。**Track A範囲外**、
  string-ext・add-data-classと同じくbash0c7-homepage独自プロジェクトの範囲

bash0c7-homepage側は全16 unitのうち残り7 unit(#3/#4/#8/#9/#10/#15/#16)のorchestratorを順次
再ディスパッチ中（2026-09-18時点）。push依頼はこれからも来る。**push依頼が来る前に`git branch -vv`で
先読みして構わない**（読むだけならcentralizationの対象外）。

## push待ち・保留中

（現在無し）

## Track Aの範囲外と判断したもの（bash0c7-homepage独自プロジェクトとして進行）

- `constant-lookup-through-namespace-and-class`（push済み）— 定数解決のnamespace/ancestor chain対応
- `string-sub-gsub-slice-range-and-more`（push済み）— String#sub/gsub/slice(Range)等
- `op-eq-dispatch-honors-user-defined-equals`（push済み、centralization前のpush、そのままでよい扱い）
- `add-data-class`（push済み、`Data.define`実装、fmt差分は修正依頼→再送で解消済み）

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
