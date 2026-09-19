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
  upstream PR候補品質へ分解するプロジェクト。**2026-09-19、向こう側も以後「Track B」という語を使わず
  「bash0C7-homepage側の分解プロジェクト」で統一すると確定**。PR_PLAN.md本来の「トラックB（RITE0400が要る）」
  とも別物なので注意

## 指示ルート（2026-09-19、user直接指示で変更）

**このセッションでの「次に何をすべきか」の不明点・確認事項は、user本人にではなく
`bash0c7-homepage-41`（呼称は変わりうる、ListAgentsで確認）経由で確認する。** push可否を含む。
user本人がこのセッションで直接話しかけた場合はそちらが優先。詳細は
Claude Code memory `ask_peer_not_user.md` 参照。peer経由で来た内容（事実関係）はそれでも
独立検証は省略しない（git ls-remote等）。

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

## push保留中（2026-09-19更新）

**6件はpush完了**（origin push済み）: `basicobject-and-object-core`(`f254aa4`)、
`per-activation-environment`(`0668ff9`)、`defined-support`(`b58be1d`、unit#5=`9239a4e`依存のまま単独PR化は保留)、
`inherited-included-hooks`(`55d6763`、unit#7=`d1af79f`依存)、`add-env-and-uri`(`060ca5e`)、
`data-layer-hash-enumerable-array-exception`(`bd16223`)。

**`unwind-break-return-super`（`fe0f31d`、旧`7e5c31b`から確定・A3含む）は保留継続。**
`git ls-remote origin`ではこのセッション側で`7e5c31b`がorigin上に存在することを確認済み
（peer側の確認では「branch自体が無い」という食い違いが未解消）。通常pushを試みたところ、
**Claude Codeのauto-modeパーミッション分類器が「Git Destructive」としてpushそのものをブロックした**
（denialメッセージのみで、git自体の拒否メッセージ=non-fast-forward等は未取得）。
force-pushが必要になった場合は`--force-with-lease`のみ使用（`--force`禁止、user明示指示）。
実行にはuser側でBash permission ruleの追加が必要な可能性がある。次回再開時、userに状況を共有し
許可を得るところから。

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

## A14との順序依存（確認済み・合意済み）

`fix/op-enter-optional-rest`(A14, `cc03daf`)と、bash0c7-homepage側が新規着手予定の
bare hash kwarg作業が、同じ`optable.rs`のOP_ENTERレジスタ配置ロジックに触れる。
**A14を先にupstream取り込み（PR化）してから着手する順序で合意済み**（2026-09-19）。
A14はorigin push済みだがmaster入り（upstream PR）はまだ。

## legacy branch 6件の精査結果（2026-09-19、読み取り専用調査完了）

前セッション由来でoriginに残っていた6branchを、既存の分解済み単位と突き合わせて分類した
（分解禁止・破壊禁止、backupは全て残したまま）。

| branch | 分類 | 内容 |
|---|---|---|
| `backup/metaprogramming-full` | 吸収済み・backup保持のみ | 26commit、既知の分解単位全てのcommit名と1対1一致 |
| `experiment/ruby-layer` | 吸収済み・backup保持のみ | 独自1commit(`4d7be58`)も`module-identity-and-singleton-class`(`d1af79f`)と同一領域 |
| `review-rite0400` | 吸収済み・backup保持のみ | 実装は重複、独自価値は到達性の実測手法のみ |
| `keep/rite0400-pr` | 吸収済み・backup保持のみ | `feature/rite0400`のサブセット |
| `feature/metaprogramming` | **要追加調査** | master側opcode命名との突合が未確定 |
| `feature/rite0400` | **独自価値あり（重要）** | `da5ee82`「Spread arguments into a call, and find the block a yield wants」が**A1とA2を1commitで解決**。現行`work/a1-splat-call-panic`より実装範囲が広い可能性（kwarg spread・BLKPUSH含む）。**A2着手時にこの実装を精査・参考にする** |

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
