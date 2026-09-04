# mruby/edge — Built-in Class & Method Coverage

A list of currently supported classes and methods, based on the implementations in `mrubyedge/src/yamrb/prelude/`.

> **Legend**
> - `.method` — class method (`self.method`)
> - `#method` — instance method
> - `(alias: x)` — also available under this name
> - `[feature: xxx]` — requires the corresponding Cargo feature flag

## Bytecode: mruby 3.x (RITE0300) and mruby 4.0 (RITE0400)

The loader picks the opcode table from the major version in the RITE header, so chunks from mruby 3.x and from mruby 4.0 (PicoRuby's mrbc) both run; any other version is refused by `rite::load`. See `docs/table.html` for the per-opcode status under either numbering.

The opcodes mruby 4.0 added: `GETIDX0`, `MATCHERR`, `SSEND0`, `SEND0`, `BLKCALL`, `RETSELF`, `RETNIL`, `RETTRUE`, `RETFALSE`, `ADDILV`, `SUBILV`, `TDEF`, `SDEF`; `ENTER` reads the 24-bit flags, including `&nil` (a method refusing a block). `LOADTRUE`/`LOADFALSE`/`LOADI8` are 3.x's `LOADT`/`LOADF`/`LOADI` renamed.

| Note | |
|---|---|
| `MATCHERR` | raises `NoMatchingPatternError`; the rest of pattern matching (`deconstruct`, `deconstruct_keys`) is not provided, so only value patterns in `case/in` run end to end |
| `defined?` | mruby 4.0 compiles it to `__defined_const?` and siblings (see Object) |
| tests | `tests/opcodes40.rs` (hand-built IREPs, one per opcode) and `tests/fixtures/mruby40.mrb` (a chunk compiled by PicoRuby 4.0.3) |

---

## Object (base of all classes)

`prelude/object.rs`

### Instance methods

| Method | Notes |
|---|---|
| `#initialize` | |
| `#==` | |
| `#!=` | |
| `#===` | |
| `#object_id` | alias: `__id__` |
| `#to_s` | |
| `#inspect` | |
| `#raise` | |
| `#nil?` | |
| `#lambda` | alias: `proc` |
| `#is_a?` | alias: `kind_of?` |
| `#class` | |
| `#<=>` | |
| `#method_missing` | |
| `#extend` | |
| `#loop` | |
| `#block_given?` | |
| `#respond_to?` | |
| `#public_send` | |
| `#!` | |
| `#equal?` | identity comparison, not value equality |
| `#send` | alias: `__send__` |
| `#instance_variable_get` | |
| `#instance_variable_set` | |
| `#instance_variable_defined?` | |
| `#instance_variables` | |
| `#freeze` | returns self and does nothing; no frozen state is modeled |
| `#frozen?` | always returns `false` |
| `#require` | only knows the linked-in features listed in `LINKED_FEATURES` (`uri`, `json`, `time`, `math`, `securerandom`); raises `LoadError` for anything else |
| `#require_relative` | shares `#require`'s implementation; a relative path never matches a linked feature, so it always raises `LoadError` |
| `#__defined_const?` | internal; backs `defined?(Foo)` in mruby 4.0 compiled bytecode |
| `#__defined_const_path?` | internal; backs `defined?(Foo::Bar)` |
| `#__defined_method?` | internal; backs `defined?(foo)` for a method call |
| `#__defined_ivar?` | internal; backs `defined?(@foo)` |
| `#__defined_gvar?` | internal; backs `defined?($foo)` |
| `#wasm?` | mruby/edge specific |
| `#puts` | `[feature: wasi]` only |
| `#p` | `[feature: wasi]` only |
| `#debug` | `[feature: wasi]` only |

### Predefined constants

| Constant | Value |
|---|---|
| `RUBY_VERSION` | VM VERSION string |
| `MRUBY_VERSION` | same as above |
| `MRUBY_EDGE_VERSION` | same as above |
| `RUBY_ENGINE` | VM ENGINE string |
| `ENV` | a plain Hash; carries the host's environment variables when built with `[feature: wasi]` and a host to ask, otherwise empty |

---

## BasicObject

`prelude/object.rs`

Deliberately carries almost nothing beyond what dispatch itself needs, so a class inheriting from it sees an empty namespace (useful for `method_missing` DSLs).

### Instance methods

| Method | Notes |
|---|---|
| `#initialize` | |
| `#==` | |
| `#!=` | |
| `#!` | |
| `#equal?` | identity |
| `#method_missing` | |
| `#__send__` | |
| `#__id__` | |

---

## Exception hierarchy

`prelude/exception.rs`

Defined classes (with inheritance):

```
Exception
├── InternalError
├── NoMemoryError
├── ScriptError
│   └── LoadError
├── SyntaxError
├── SignalException
│   └── Interrupt
├── SystemExit
├── SystemStackError
└── StandardError
    ├── RuntimeError
    ├── TypeError
    ├── ArgumentError
    ├── RangeError
    ├── IndexError
    │   ├── KeyError
    │   └── StopIteration
    ├── FrozenError
    ├── NoMatchingPatternError
    ├── ZeroDivisionError
    ├── NotImplementedError
    ├── SecurityError
    ├── SystemCallError
    ├── NoMethodError
    └── NameError
```

### Instance methods (Exception)

| Method | Notes |
|---|---|
| `#message` | |
| `#backtrace` | always returns `[]`; the VM keeps no call stack to report |

---

## Module

`prelude/module.rs`

| Method | Notes |
|---|---|
| `.new` | anonymous module; a block, if given, is `module_eval`'d against it |
| `#include` | calls `self.included(base)` on the included module if it defines that hook |
| `#ancestors` | |
| `#define_method` | takes a block or a `Proc` as the method body |
| `#module_eval` | alias: `class_eval` |
| `#instance_methods` | `include_super` argument defaults to `true` |
| `#method_defined?` | |
| `#instance_method` | returns the method body as a `Proc` (not a full `UnboundMethod`) |
| `#private_instance_methods` | always returns `[]`; no method visibility is modeled |
| `#const_defined?` | |
| `#const_get` | |
| `#const_set` | |
| `#private` | accepted as a no-op; no method visibility is modeled |
| `#protected` | accepted as a no-op; no method visibility is modeled |
| `#public` | accepted as a no-op; no method visibility is modeled |
| `#to_s` | defined in `prelude/class.rs`; alias: `#name` |
| `#name` | defined in `prelude/class.rs`; same as `#to_s`/`#inspect` |

---

## Class (subclass of Module)

`prelude/class.rs`

| Method | Notes |
|---|---|
| `#new` | creates a new instance |
| `#attr_reader` | |
| `#attr_writer` | |
| `#attr_accessor` | alias: `attr` |
| `#ancestors` | |
| `#inspect` | defined on the Module side |

---

## Data

`prelude/data.rs`

Ruby 3.2's immutable value object: `Point = Data.define(:x, :y)` builds a class whose instances carry exactly those members. `Data.define` does not accept a block to add methods (that would need a Ruby-level class body).

### Class methods (on `Data`)

| Method | Notes |
|---|---|
| `.define` | takes Symbol (or String) member names |

### Class methods (on a class returned by `Data.define`)

| Method | Notes |
|---|---|
| `.new` | alias: `.[]`; accepts positional or keyword arguments |
| `.[]` | alias of `.new` |
| `.members` | |

### Instance methods (on a `Data.define`d class)

| Method | Notes |
|---|---|
| `#<member>` | one reader per declared member, e.g. `#x`, `#y` |
| `#members` | |
| `#to_h` | |
| `#with` | returns a copy with the given keyword members replaced |
| `#==` | alias: `eql?` |
| `#eql?` | alias of `#==` |
| `#inspect` | alias: `#to_s`; formatted as `#<data x=1, y=2>` |
| `#to_s` | alias of `#inspect` |

---

## Integer

`prelude/integer.rs`

| Method | Notes |
|---|---|
| `#[]` | bit reference |
| `#-@` | unary minus |
| `#+` | mixed arithmetic with Float |
| `#-` | mixed arithmetic with Float |
| `#**` | mixed arithmetic with Float |
| `#%` | |
| `#&` | bitwise AND |
| `#\|` | bitwise OR |
| `#^` | bitwise XOR |
| `#~` | bitwise NOT |
| `#<<` | left shift |
| `#>>` | right shift |
| `#abs` | |
| `#to_i` | |
| `#to_f` | |
| `#chr` | |
| `#times` | takes a block |
| `#inspect` | alias: `to_s` |
| `#clamp` | |

---

## Float

`prelude/float.rs`

| Method | Notes |
|---|---|
| `#to_i` | |
| `#to_f` | |
| `#+` | mixed arithmetic with Integer |
| `#-` | mixed arithmetic with Integer |
| `#*` | mixed arithmetic with Integer |
| `#/` | mixed arithmetic with Integer |
| `#+@` | unary plus |
| `#-@` | unary minus |
| `#**` | mixed arithmetic with Integer |
| `#abs` | |
| `#nan?` | |
| `#infinite?` | |
| `#finite?` | |
| `#inspect` | alias: `to_s` |
| `#clamp` | |

---

## NilClass

`prelude/nilclass.rs`

| Method | Notes |
|---|---|
| `#to_s` | returns `""` |
| `#inspect` | returns `"nil"` |
| `#nil?` | returns `true` |

---

## TrueClass

`prelude/trueclass.rs`

| Method | Notes |
|---|---|
| `#to_s` | alias: `inspect` |
| `#&` | |
| `#\|` | |
| `#^` | |

---

## FalseClass

`prelude/falseclass.rs`

| Method | Notes |
|---|---|
| `#to_s` | alias: `inspect` |
| `#&` | |
| `#\|` | |
| `#^` | |

---

## Symbol

`prelude/symbol.rs`

| Method | Notes |
|---|---|
| `#to_s` | |
| `#to_sym` | returns self |
| `#inspect` | `:sym` format |
| `#to_proc` | converts symbol to a proc that calls the method |

---

## Proc

`prelude/proc.rs`

| Method | Notes |
|---|---|
| `.new` | class method |
| `#call` | |
| `#[]` | alias of `#call` |
| `#arity` | reads the required-argument count back from the `ENTER` instruction; `-1` for a Rust-implemented method (no `ENTER` to read) |

---

## String

`prelude/string.rs`

| Method | Notes |
|---|---|
| `.new` | class method |
| `#+` | string concatenation |
| `#*` | repetition |
| `#<<` | destructive append |
| `#[]` | alias: `slice`; also accepts a Range |
| `#b` | returns a binary (byte) string |
| `#clear` | |
| `#chomp` | |
| `#chomp!` | |
| `#dup` | |
| `#empty?` | |
| `#getbyte` | |
| `#setbyte` | |
| `#index` | |
| `#ord` | |
| `#slice` | also accepts a Range |
| `#slice!` | also accepts a Range |
| `#split` | |
| `#sub` | pattern is a String or `[feature: mruby-regexp]` Regexp; replacement is a String or a block |
| `#sub!` | destructive `#sub`; returns `nil` when the pattern never matched |
| `#gsub` | pattern is a String or `[feature: mruby-regexp]` Regexp; replacement is a String or a block |
| `#gsub!` | destructive `#gsub`; returns `nil` when the pattern never matched |
| `#lstrip` | |
| `#lstrip!` | |
| `#rstrip` | |
| `#rstrip!` | |
| `#strip` | |
| `#strip!` | |
| `#to_sym` | alias: `intern` |
| `#start_with?` | |
| `#end_with?` | |
| `#include?` | |
| `#bytes` | |
| `#chars` | |
| `#each_byte` | takes a block |
| `#each_char` | takes a block |
| `#upcase` | |
| `#upcase!` | |
| `#downcase` | |
| `#downcase!` | |
| `#capitalize` | first character upcased, rest downcased |
| `#to_i` | |
| `#to_f` | |
| `#unpack` | pack format: `Q q L l I i S s C c` |
| `#size` | alias: `bytesize`, `length` |
| `#inspect` | |
| `#to_s` | |
| `#=~` | added by `[feature: mruby-regexp]` |
| `#!~` | added by `[feature: mruby-regexp]` |

---

## Enumerable (module)

`prelude/enumerable.rs`  
Included in Array, Hash, and Range.

| Method | Notes |
|---|---|
| `#map` | |
| `#find` | |
| `#select` | |
| `#filter` | alias of `#select` |
| `#reject` | non-destructive; `#delete_if` is the destructive sibling |
| `#all?` | |
| `#any?` | |
| `#delete_if` | |
| `#each_with_index` | |
| `#sort` | |
| `#sort_by` | |
| `#max` | |
| `#min` | |
| `#minmax` | |
| `#compact` | |
| `#count` | |
| `#to_a` | |
| `#uniq` | |
| `#reduce` | |
| `#sum` | |

---

## Array

`prelude/array.rs`  
Includes Enumerable.

| Method | Notes |
|---|---|
| `.new` | class method |
| `#+` | returns a new array containing elements from both arrays |
| `#push` | alias: `<<` |
| `#[]` | alias: `at` |
| `#[]=` | |
| `#clear` | |
| `#delete_at` | |
| `#each` | |
| `#empty?` | |
| `#size` | alias: `length` |
| `#include?` | |
| `#&` | set intersection |
| `#-` | set difference |
| `#\|` | set union |
| `#first` | |
| `#last` | |
| `#pop` | |
| `#shift` | |
| `#unshift` | |
| `#dup` | |
| `#uniq!` | |
| `#map!` | |
| `#select!` | |
| `#reject!` | |
| `#sort!` | |
| `#sort_by!` | |
| `#pack` | format: `Q q L l I i S s C c` |
| `#inspect` | alias: `to_s` |
| `#join` | |
| `#flatten` | returns a new flattened array (recursive) |
| `#flatten!` | flattens self in place (recursive), returns self or nil |

---

## Hash

`prelude/hash.rs`  
Includes Enumerable.

| Method | Notes |
|---|---|
| `.new` | class method |
| `#[]` | |
| `#[]=` | |
| `#clear` | |
| `#dup` | |
| `#delete` | |
| `#empty?` | |
| `#has_key?` | |
| `#key?` | alias of `#has_key?` |
| `#member?` | alias of `#has_key?` |
| `#include?` | alias of `#has_key?` |
| `#fetch` | raises `KeyError` when the key is absent and no default/block was given |
| `#dig` | walks nested hashes, stopping at the first `nil` |
| `#has_value?` | |
| `#key` | reverse lookup: value → key |
| `#keys` | |
| `#each` | block receives key and value |
| `#each_pair` | alias of `#each` |
| `#each_key` | takes a block |
| `#each_value` | takes a block |
| `#size` | alias: `length`, `count` |
| `#merge` | |
| `#merge!` | |
| `#to_h` | |
| `#values` | |
| `#inspect` | alias: `to_s` |
| `#flatten` | returns an array of [key1, value1, key2, value2, ...] |

---

## Range

`prelude/range.rs`  
Includes Enumerable. Integer ranges only.

| Method | Notes |
|---|---|
| `#include?` | supports Integer and Float arguments |
| `#each` | Integer ranges only |

---

## SharedMemory (mruby/edge specific)

`prelude/shared_memory.rs`  
A class for zero-copy sharing with WASM linear memory.

| Method | Notes |
|---|---|
| `.new` | takes a size in bytes |
| `#to_s` | |
| `#offset_in_memory` | alias: `to_i` — returns the memory offset (address) |
| `#[]` | range / index access |
| `#[]=` | |
| `#replace` | |
| `#read_by_size` | |

---

## Random `[feature: mruby-random]`

`prelude/rand.rs`  
Uses the XorShift PRNG.

| Method | Notes |
|---|---|
| `.new` | seed is optional |
| `.rand` | class method |
| `.srand` | class method |
| `#rand` | instance method |
| `#seed` | returns the current seed |

Added to Kernel (Object):

| Method | Notes |
|---|---|
| `#rand` | uses the global default RNG |

---

## Regexp `[feature: mruby-regexp]`

`prelude/regexp.rs`  
Uses the Rust `regex` crate.

| Method | Notes |
|---|---|
| `.new` | alias: `.compile` |
| `#=~` | returns match position or `nil` |
| `#!~` | |
| `#match` | returns a MatchData object |
| `#inspect` | |

### MatchData `[feature: mruby-regexp]`

| Method | Notes |
|---|---|
| `#[]` | capture group reference |

---

## URI `[feature: mruby-uri]`

`prelude/uri.rs`  
`application/x-www-form-urlencoded` encoding only.

| Method | Notes |
|---|---|
| `.encode_www_form` | takes a Hash or an Array of `[key, value]` pairs |
| `.encode_www_form_component` | |
| `.decode_www_form_component` | |

---

## Notes

- Some arithmetic operators (`*`, `/`) for Integer are not defined as instance methods in this prelude; they are handled directly by the VM bytecode interpreter (`eval.rs`).
- Comparison operators (`<`, `<=`, `>`, `>=`) are similarly handled on the VM side.
- `String#=~` and `#!~` are only added when `[feature: mruby-regexp]` is enabled.
