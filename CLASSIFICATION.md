# 119 命令の確定分類（feature/rite0400 = 4ce88e8）

すべて mruby 4.0 のコンパイラで実際にコンパイルし、バイト列で確認。
「到達しない」は `codegen40.c` / prism に生成箇所が無いことで確認。

## 集計

| 分類 | 数 |
|---|---|
| **I. 到達する・実装済み・テスト済み** | 96 |
| **II. 到達する・実装済み・テストが踏んでいない** | 7 |
| **III. 到達する・未実装** | 9 |
| **IV. 到達しない（コンパイラが出さない）** | 7 |
| 計 | 119 |

## II. 到達する・実装済み・テストが踏んでいない — 7

| 命令 | 到達 Ruby |
|---|---|
| `SETCONST` | `FOO = 1` |
| `METHOD` | ブロック 256 個の後に `def late; 42; end`（シンボル番号 > 255） |
| `DEF` | 同上。境界は実測済み: 255 個なら `TDEF` に融合、256 個で `TCLASS`+`METHOD`+`DEF` |
| `TCLASS` | 同上 |
| `SCLASS` | `class << self ... end` / `o = Object.new; class << o ... end`（無条件） |
| `EXT3` | `def big` 内にローカル 250 個 + `@p0..@p319 = 0` + ivar の 10 段ネスト式 |
| `STOP` | 任意のトップレベル（`1` だけでも出る。全 irep0 の末尾） |

**`EXT3` は私が報告済みの C-1。** 自分で再現確認済み（`EXT3+GETIV: 6`）。

## III. 到達する・未実装 — 9

| 命令 | 到達 Ruby |
|---|---|
| `GETCV` / `SETCV` | `class K; def s; @@v = 1; end; def g; @@v; end; end` |
| `SETMCNST` | `K::X = 5` |
| `INTERN` | `:"a#{n}"` |
| `ARYPUSH` | `[*a, 3]` |
| `HASHADD` | `{**h, b: 2}` |
| `HASHCAT` | `{**h, **i}` |
| `ARGARY` | 引数なしの `super`（`super(x)` では出ない） |
| `ARYSPLAT` | `def m(a); return *a; end` |

実アプリが使うのは `ARGARY` `INTERN` `HASHCAT` の 3。すべてここに入る。

## IV. 到達しない — 7

| 命令 | 根拠 |
|---|---|
| `GETSV` | `codegen40.c` に生成箇所なし（952 行は peephole の case ラベルのみ）。`$~` `$!` `$1` でも出ない |
| `SETSV` | `OP_SETSV` の出現が 1 件も無い |
| `ASET` | 出現が 1 件も無い（添字代入は `SETIDX`） |
| `DEBUG` | 出現が 1 件も無い |
| `ERR` | 生成箇所 `raise_error()` はあるが prism が手前で弾く。6 形試行、全部 COMPILE-PANIC。**exhaustive ではない** |
| `CALL` | 生成箇所ゼロ。実体は VM 側の手書き irep（mruby `proc.c` の `call_iseq[] = { OP_CALL }`）。8 形試行、全部 SEND 系 |
| `SYMBOL` | 生成箇所は 1 つだが、発火条件（補間シンボルの parts が StringNode ただ 1 個）を prism が作らない。35 形試行、全部 `INTERN` か `LOADSYM` |

## 線引きへの含意

udzura さんの「意味論が明確なものだけでも実装されていれば問題ない」に対し、
**到達性のほうが客観的な線になる**:

- III の 9 → コンパイラが出す。実装すべき
- IV の 7 → コンパイラが出さない。not_implemented の根拠が「意味論が曖昧」より強い

## テストを書くときの罠（実測で判明）

`mrubyedge::rite::insn::OpCode` の宣言順はワイヤのバイト順と一致しない。
`ENUM_TABLE` がバイト → variant を張り直しているため、**`op as u8` はオペコードの
バイト値ではない**（バイト `0x63` = `METHOD` だが `op as u8` は 87）。
命令を検査するテストは生バイトか variant で比較すること。`as u8` は静かに誤る。
