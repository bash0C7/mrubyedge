# 未実装 14 命令の到達性（実測・2026-09-12、feature/rite0400 = 4ce88e8）

到達 Ruby はすべて実際にコンパイルして、その命令が出たことをバイト列で確認。
「到達しない」は mruby 4.0 の `codegen40.c` に生成箇所が無いことで確認。

## 到達する — 9 命令

| 命令 | 到達 Ruby |
|---|---|
| `GETCV` / `SETCV` | `class K; def s; @@v = 1; end; def g; @@v; end; end` |
| `SETMCNST` | `class K; end` + `K::X = 5` |
| `INTERN` | `n = 1` + `:"a#{n}"` |
| `ARYPUSH` | `a = [1,2]` + `[*a, 3]`（`[0, *a, 3]` でも出る） |
| `HASHADD` | `h = {a: 1}` + `{**h, b: 2}` |
| `HASHCAT` | `{**h, **i}`（`{**h, b: 2}` でも HASHADD と同時に出る） |
| `ARGARY` | 引数なしの `super`（zsuper）。`super(x)` では出ない |
| `ARYSPLAT` | `def m(a); return *a; end` |

## 到達しない — 5 命令

| 命令 | 根拠 |
|---|---|
| `GETSV` | `codegen40.c` に `genop_*(s, OP_GETSV, ...)` が**無い**。952 行は peephole の case ラベルのみ。`$~` `$!` `$1` を試しても出ない |
| `SETSV` | `codegen40.c` に `OP_SETSV` の出現が 1 件も無い |
| `ASET` | `codegen40.c` に `OP_ASET` の出現が 1 件も無い（添字代入は `SETIDX`） |
| `DEBUG` | `codegen40.c` に `OP_DEBUG` の出現が 1 件も無い |
| `ERR` | 生成箇所は `raise_error()`（3316 行、呼び出し元 6 箇所＝unexpected break/next/redo/retry, unsupported pattern）だが、**Prism が codegen の手前で全部弾く**。試した 6 形すべて COMPILE-PANIC。**exhaustive ではない**（他の形が残っている可能性） |

## この結果の意味

udzura さんの「意味論が明確なものだけでも実装されていれば問題ない」に対し、
**到達性という、より客観的な線が引ける**:

- 到達する 9 命令 → 意味論も明確（class 変数・スコープ付き定数・動的シンボル・
  splat・zsuper）。**実装すべき**
- 到達しない 5 命令 → mruby 4.0 のコンパイラが出さない。not_implemented の
  正当性が「意味論が曖昧だから」より強い

なお実アプリ（bash0c7-homepage）が使うのは `ARGARY` `INTERN` `HASHCAT` の 3 で、
すべて到達する 9 の側に入る。

## 未解決

- `A-8` 空シンボル（0xFFFF）: このラウンドの全プローブで load 失敗はゼロ。
  到達 Ruby は依然不明
- `ERR`: 「到達しない」の証明が exhaustive でない
