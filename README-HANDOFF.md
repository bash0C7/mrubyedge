# ローカルへの引き継ぎ（2026-09-19・`/compact` 準備時点）

mrubyedge の RITE0400 対応 PR 再挑戦、Track A（RITE0400 非依存の VM 修正、A1〜A14）を進めている。
このセッションはこれから `/compact` される見込み。再開・compact後の再開はこの文書から読む。

## まず読む順

1. この文書 — 現況・次にやること
2. `PR_PLAN.md` — トラック A/B の対応表（本体、番号は今もこれが基準）
3. Artifact「Track A 分解図」 https://claude.ai/code/artifact/2f6d24da-455a-4b8b-ab1f-18a86994b119
   （version 6。A1〜A14 それぞれの repro・実測サイズ・到達性・実装状況をグラフィカルにまとめてある。
   ただしA3の実装済み反映など、本文書の方が新しい情報を含む場合がある。食い違ったら本文書を正とする）
4. `REVIEW_BRIEF.md` / `PROBLEMS.md` / `REACHABILITY.md` / `CLASSIFICATION.md` — 前々セッションの実測・分類の一次資料
5. Claude Code のメモリ（working directoryで自動的に読み込まれる）:
   `mrubyedge-rite0400-track-a.md` / `push_to_fork_ok.md` / `dont_speculate_on_other_sessions.md`

## 用語（セッション間で食い違っていたので明記）

- **Track A**: このセッションが使う呼称。PR_PLAN.md定義のA1〜A14（RITE0400非依存のVM修正）
- **bash0c7-homepage側の分解プロジェクト**: このセッションが使う呼称。`bash0c7-homepage`（本番SSR worker）が
  使っている「動くだけ」の雑なfork差分（旧`feature/metaprogramming`、16 monkey-patch commit）を
  upstream PR候補品質へ分解するプロジェクト。**向こうのセッション内では「Track B」と呼ばれている場合がある**
  （このセッションが命名したわけではなく、由来未確認）。Track Aとは別物で、PR_PLAN.md本来の「トラックB
  （RITE0400が要る）」とも別物なので注意。「Track B」という語が出てきたらどちらの意味か文脈で判断する

## upstream の状態

`bash0C7/mrubyedge` の upstream remote から `v2.0.0`（`afff593`）がリリース済み。
内容は Track B1（RITE0400 本体、PR #89/#90）+ バージョン番号変更（PR #91）のみ。
Track A に関わる新規実装は upstream 側には無い。ローカル `master` は `v2.0.0` と一致（fast-forward）。
以後 upstream が進んだら `git fetch upstream --tags && git checkout master && git rebase upstream/master`。

## リポジトリ書き込みの体制

**`bash0C7/mrubyedge` への push・branch 作成・master 操作はこのセッション（Track A 担当）に集約**
（2026-09-14、ユーザー決定、理由は前提ズレ回避）。並行して `bash0c7-homepage-41`（session名は変わることがある。
ListAgentsで確認し、変わっていたら新しい名前へメッセージする）という別 Claude セッションが、
分解プロジェクトを進めている。手法（master から書き直す・1関心事=1PR・到達性を実測してから範囲決定）は共通。

運用: 向こうは自分の git worktree（`bash0C7/mrubyedge` と同じ `.git` を共有）でローカル commit まで進め、
origin への push だけこちらに依頼してくる（cross-session message経由、時に `mrubyedge-uN` という
orchestrator名義で届く）。依頼が来たら:
1. `git log --oneline v2.0.0..<branch>`（または依存先..<branch>）で commit 数・trailer 有無を確認
2. `git show --stat <branch>` で規模・ファイルを確認
3. `cargo test --manifest-path <worktree>/mrubyedge/Cargo.toml`（featureがあれば有効/無効両方）を
   **このセッションで独立実行**（相手の報告を鵜呑みにしない。過去2回fmt差分でpush保留になった実績あり）
4. `cargo fmt --manifest-path <worktree>/mrubyedge/Cargo.toml --check` を確認
5. 問題なければ `git push origin <branch>`。**base(v2.0.0)以外に依存するbranch（他unitの上に乗る）は
   単独PR化不可なので、依存関係のメモを残す**
6. 既にorigin pushしたbranchのSHAが変わった場合（amend/squash）は、pushしたかどうかを`git ls-remote origin <branch>`
   で必ず確認する。**まだpushしていなければ force-push不要（通常push）、既pushならforce-push（user確認必須）**

bash0C7-homepage 本体の作業はそちら側の担当のまま。PR文面・PRコメント・issueは**必ずユーザー本人が書く**。
「PRを出す」話自体、指示があるまでこちらから言い出さない。

**他セッションとの関係を推測して第三者に伝えない。** 似た名前のセッション（`bash0c7-homepage-41` と
`-44`等）を見ても、同じプロジェクトか無関係かは確認してから扱う（2026-09-19、user指摘で学習）。

## push保留中（2026-09-19時点、すべて実行可能な状態、user指示待ち）

| branch | SHA(tip) | 内容 | 備考 |
|---|---|---|---|
| `basicobject-and-object-core` | `f254aa4`（旧`5510a98..cbe07ad`から確定） | BasicObject階層・equal?/!実装 | 確定済み、即push可。未push（origin未登録） |
| `per-activation-environment` | `0668ff9`（旧2commitから1commitへsquash確定） | 環境がIREP idでファイルされ再帰で上書きされるバグ修正 | 確定済み、即push可。未push |
| `defined-support` | `b58be1d` | mruby4.0の`__defined_*?`対応 | **unit#5(`constant-lookup-through-namespace-and-class`=`9239a4e`)の上に依存。単独PR化はunit#5がupstreamに入るまで不可** |
| `inherited-included-hooks` | `55d6763` | Class#inherited/Module#included hook | **unit#7(`module-identity-and-singleton-class`=`d1af79f`、push済み)の上に依存** |
| `add-env-and-uri` | `060ca5e`（6commit） | ENV/URI追加 | Track A範囲外(stdlib)、現状維持でuser確定。未push |
| `data-layer-hash-enumerable-array-exception` | `bd16223`（4commit） | Hash/Enumerable/Array/Exceptionメソッド追加 | green light済み。未push |
| `unwind-break-return-super` | `fe0f31d`（旧`7e5c31b`から確定、A3含む） | RETURN_BLK/BREAK/SUPER修正 | **既に旧SHA(`7e5c31b`)をpush済みなので、新SHAへの更新はforce-pushが必要。user確認必須** |

このうち`7e5c31b`→`fe0f31d`（unwind-break-return-super）以外はorigin未登録（`git ls-remote origin <branch>`で空）
なので通常pushでよい。

## bare hash kwarg（新規、bash0c7-homepage側が引き取り予定）

分解プロジェクト側で13単位+単位11/12の統合検証をした際、`Funicular::Model.load_schema("attributes"=>{...})`
のような波括弧無しhash引数（keyword未宣言メソッドへのrocket hash）呼び出しが**v2.0.0時点で既に壊れている**
ことが判明（受け手がHashでなく文字列そのものを受け取る）。これは8fd972fの「keyword未宣言メソッドはpairを
1個のHashにまとめる」という、以前A1のスコープ外と切り分けた残り範囲に該当。**Track A側は未着手、
bash0c7-homepage側の分解プロジェクトが新規単位として引き取ることで合意済み（2026-09-19）。**
なお最初「本番SSRが全滅」と誤報したが、実際は検証用の使い捨て統合branchでの現象で本番影響は無かった
（訂正済み）。

## Track A の現況（A1〜A14）

詳細と repro・実測サイズは Artifact 参照。

| # | 内容 | 状態 | 担当・branch |
|---|---|---|---|
| A1 | 呼び出し側 splat 引数の PANIC（SEND の n=15=CALL_MAXARGS） | **実装済み・push済み** | このセッション、`work/a1-splat-call-panic` |
| A2 | yield のブロック探索 PANIC（BLKPUSH） | 未着手 | — |
| A3 | begin を出る return PANIC（RETURN_BLK） | **実装済み・push済み**（`7e5c31b`→`fe0f31d`更新待ち、上表参照） | bash0c7-homepage側、`unwind-break-return-super` |
| A4 | Module の singleton クラス PANIC | **実装済み・push済み**（A4単体より範囲が広い） | bash0c7-homepage側、`module-identity-and-singleton-class` |
| A5 | クラス変数 GETCV/SETCV | 未着手 | — |
| A6 | SETMCNST | 未着手 | — |
| A7 | 動的シンボル INTERN | 未着手 | — |
| A8 | 配列splat ARYPUSH/ARYSPLAT | 未着手 | — |
| A9 | hash展開 HASHADD/HASHCAT | **実装済み・push済み** | bash0c7-homepage側、`implement-hashadd-hashcat` |
| A10 | 引数無しsuper ARGARY+SUPER | 未着手 | — |
| A11 | 未実装命令をpanicでなくErrに | 未着手 | — |
| A12 | INT64 poolのbig-endian読み | 未着手 | — |
| A13 | コンテナの堅牢化（3コミット分割可） | 未着手 | — |
| A14 | optional+restパラメータのレジスタ配置 | 帰属確定・未実装（`fix/op-enter-optional-rest`=`cc03daf`） | Track A側で拾う |
| (新) | OP_BREAK unwind先バグ | `unwind-break-return-super`(`fe0f31d`)で修正済み。**Track A番号は不要**（bash0c7-homepage側の最終判断、unit#3自身の対象バグだった） | 番号化しない |
| (新) | `mrb_funcall`経由でsuperが呼べない | 同上、`fe0f31d`で修正済み。**Track A番号は不要** | 番号化しない |
| (新) | `per-activation-environment`の環境上書きバグ | Track A候補、push保留中（上表参照） | 番号未確定 |
| (新) | bare hash kwarg（keyword未宣言methodへのrocket hash） | bash0c7-homepage側が新規単位として引き取り予定 | 番号未確定、実装待ち |
| (新) | break out of yield の frame位置ずれ、2段block forwarding、ensure+returnがiteration停止、lambda returnがNil | すべてpre-existing・v2.0.0で同じに壊れている。Track A候補としてログ済み、numbering はuser判断 | 未着手 |

## Track Aの範囲外と判断したもの（bash0c7-homepage独自プロジェクトとして進行、push済み）

- `constant-lookup-through-namespace-and-class`（`9239a4e`）— 定数解決のnamespace/ancestor chain対応
- `string-sub-gsub-slice-range-and-more` — String#sub/gsub/slice(Range)等
- `op-eq-dispatch-honors-user-defined-equals` — centralization前のpush、そのままでよい扱い
- `add-data-class` — `Data.define`実装

## 環境の再現

### テストが使うコンパイラ

`mrubyedge/Cargo.toml` の dev-dependency は `mruby-compiler2-sys = "0.5.0"`（crates.io、v2.0.0で解決済み）。

### CI のゲート

```sh
cargo test --manifest-path mrubyedge/Cargo.toml
cargo fmt --manifest-path mrubyedge/Cargo.toml --check
cargo clippy -p mrubyedge --features "mruby-random,mruby-regexp"   # これがCIの実ゲート
MRUBYEDGE_INSN_LIMIT=10000 cargo test --features insn-limit --test insn_limit
```

Mac では `~/.cargo/bin` を PATH に通す必要がある（`export PATH="$HOME/.cargo/bin:$PATH"`）。
clippy component が無い場合は `rustup component add clippy`。v2.0.0時点、work/a1-splat-call-panic上で
364 passed / 0 failed（全ゲート green）。

## 作業規律

- **PRの単位は「取り込む価値の高いこと」（実Rubyで到達するか・実アプリが動的に使うか）と
  「コード(変更ファイル)のコンパクトさ」の2軸で決める**（2026-09-14、ユーザー明言）
- **「実アプリが使用」は静的到達性（バイトコードへの出現）であって動的な実行確認ではない。**
  混同した実例あり（ARGARY/INTERN、bash0c7-homepage側の実測で訂正）
- **代理指標を実測と取り違えない。範囲の線を測定より先に議論で引かない。「masterも壊れている」は
  範囲外の理由にならない。テスト先行、赤を確認してから直す。コメントを足さない。**
- **cherry-pickしない。masterから書き直す。**
- **PR文面・PRコメント・issueは必ずユーザー本人が書く。**「PRを出す」話自体、指示があるまで言い出さない
- push前に独立検証（cargo test・cargo fmt --check）する。相手の報告を鵜呑みにしない
- 一次資料（mruby本家のvendor `vm.c`等）で設計を確認してから実装する
- **他セッションの正体・関係性を推測で第三者に伝えない。確認してから動く**

## 未完（前セッションから持ち越し、Track Aとは別）

- `coverage/language.rb` の `#broken` 10件のbytecode/stdlib分類
- Ruby レイヤーの問題（`Module#define_method`等）— Track Aの範囲外、バイトコード層が片付いてから
