# The language mruby 4.0 speaks, and the methods it answers, as Ruby that runs.
#
# Sibling of coverage/opcodes.rb. That file proves every opcode is reached;
# this one proves the behaviour behind them is right. Each section is a
# standalone program and the line above it names the value it must produce.
# tests/language_coverage.rs reads this file, compiles each section, runs it, and checks.
#
# A section marked broken states the value Ruby gives and what this VM gives
# instead. The test holds them to the wrong answer on purpose: when one is
# fixed the test fails and says to move the section back to #section.
#
# The whole file is also valid Ruby: the markers are comments.

#section if_elsif_else | int 20
lx = 2
if lx == 1 then 10 elsif lx == 2 then 20 else 30 end

#section unless_clause | int 20
lu = 1
unless lu == 2 then 20 else 30 end

#section case_when_value | int 20
cw = 2
case cw
when 1 then 10
when 2 then 20
else 30
end

#broken case_when_class | int 2 | evaluates to 1: a class in a when clause is compared with itself, not the subject
cc = 2
case cc
when String then 1
when Integer then 2
else 3
end

#section case_when_range | int 2
cr = 5
case cr
when 1..3 then 1
when 4..9 then 2
else 3
end

#section case_in_value | int 20
ci = 2
case ci
in 1 then 10
in 2 then 20
end

#section ternary | int 7
1 == 1 ? 7 : 8

#section while_loop | int 3
wi = 0
while wi < 3
  wi += 1
end
wi

#section until_loop | int 3
ui = 0
until ui >= 3
  ui += 1
end
ui

#section for_loop | int 6
fs = 0
for fe in [1, 2, 3]
  fs += fe
end
fs

#section loop_with_break | int 3
li = 0
loop do
  li += 1
  break if li > 2
end
li

#section next_in_block | int 4
ns = 0
[1, 2, 3].each do |x|
  next if x == 2
  ns += x
end
ns

#section begin_rescue_else | int 3
begin
  1
rescue
  2
else
  3
end

#section begin_ensure_value | int 1
def cov_ensure
  begin
    1
  ensure
    99
  end
end

cov_ensure

#section retry_once | int 2
$cov_retry = 0
begin
  $cov_retry += 1
  raise "again" if $cov_retry < 2
  $cov_retry
rescue
  retry
end

#section rescue_message | str "boom"
begin
  raise "boom"
rescue => e
  e.message
end

#section return_in_ensure | int 1
def cov_ret_ensure
  begin
    return 1
  ensure
    2
  end
end

cov_ret_ensure

#section return_in_rescue | int 1
def cov_ret_rescue
  begin
    return 1
  rescue
    2
  end
end

cov_ret_rescue

#section return_in_while | int 1
def cov_ret_while
  while true
    return 1
  end
end

cov_ret_while

#section optional_argument | int 3
def cov_opt(a, b = 2)
  a + b
end

cov_opt(1)

#section rest_argument | int 3
def cov_rest(*a)
  a.size
end

cov_rest(1, 2, 3)

#section keyword_argument | int 3
def cov_kw(a:, b: 2)
  a + b
end

cov_kw(a: 1)

#section double_splat_parameter | int 5
def cov_dsplat(**o)
  o[:a]
end

cov_dsplat(a: 5)

#section block_parameter | int 7
def cov_blockp(&b)
  b.call
end

cov_blockp { 7 }

#section yield_value | int 6
def cov_yield_v
  yield 3
end

cov_yield_v { |x| x * 2 }

#section block_given | int 1
def cov_bg
  block_given? ? 1 : 0
end

cov_bg { }

#section symbol_to_proc | int 3
[1, 2, 3].map(&:to_s).size

#section lambda_call | int 6
cov_l = ->(a) { a * 2 }
cov_l.call(3)

#section lambda_dot_call | int 6
cov_l2 = ->(a) { a * 2 }
cov_l2.(3)

#section proc_new | int 2
cov_p = Proc.new { |a| a + 1 }
cov_p.call(1)

#section method_chain | int 1
[3, 1, 2].sort.first

#section safe_navigation | int 1
cov_sn = nil
cov_sn&.size.nil? ? 1 : 0

#section class_with_ivar | int 4
class CovL1
  def initialize(v)
    @v = v
  end

  def v
    @v
  end
end

CovL1.new(4).v

#section inheritance | int 1
class CovL2
  def m
    1
  end
end

class CovL3 < CovL2
end

CovL3.new.m

#section super_with_args | int 2
class CovL4
  def m(x)
    x
  end
end

class CovL5 < CovL4
  def m(x)
    super + 1
  end
end

CovL5.new.m(1)

#section module_singleton | int 3
module CovL6
  def self.m
    3
  end
end

CovL6.m

#section module_include | int 5
module CovL7
  def m
    5
  end
end

class CovL8
  include CovL7
end

CovL8.new.m

#section module_extend | int 6
module CovL9
  def m
    6
  end
end

class CovL10
  extend CovL9
end

CovL10.m

#section attr_accessor | int 8
class CovL11
  attr_accessor :v
end

cov_l11 = CovL11.new
cov_l11.v = 8
cov_l11.v

#section constant_in_class | int 11
class CovL12
  V = 11

  def g
    V
  end
end

CovL12.new.g

#section singleton_on_object | int 13
cov_obj = Object.new

def cov_obj.m
  13
end

cov_obj.m

#section method_missing | int 15
class CovL13
  def method_missing(n, *a)
    15
  end
end

CovL13.new.whatever

#section respond_to | int 1
class CovL14
  def m
  end
end

CovL14.new.respond_to?(:m) ? 1 : 0

#section is_a_check | int 1
1.is_a?(Integer) ? 1 : 0

#section nested_class | int 17
class CovL15
  class Inner
    def m
      17
    end
  end
end

CovL15::Inner.new.m

#section alias_method_keyword | int 18
class CovL16
  def a
    18
  end
  alias b a
end

CovL16.new.b

#section to_s_override | str "k"
class CovL17
  def to_s
    "k"
  end
end

"#{CovL17.new}"

#broken equality_override | int 1 | evaluates to 0: == compares identity and does not reach a user-defined ==
class CovL18
  def ==(o)
    true
  end
end

CovL18.new == 1 ? 1 : 0

#section sort_with_block | int 1
[3, 1, 2].sort { |x, y| x <=> y }.first

#section string_interpolation | str "x1y"
sv = 1
"x#{sv}y"

#section heredoc | int 3
hd = <<~TXT
  hi
TXT
hd.size

#section symbol_to_s | str "abc"
:abc.to_s

#section dynamic_symbol | str "a1"
dv = 1
:"a#{dv}".to_s

#section array_size | int 3
[1, 2, 3].size

#section array_splat_literal | int 3
asl = [2, 3]
[1, *asl].size

#section hash_size | int 2
{ a: 1, b: 2 }.size

#section hash_spread | int 2
hs = { a: 1 }
{ **hs, b: 2 }.size

#section range_inclusive | int 3
(1..3).to_a.size

#section range_exclusive | int 2
(1...3).to_a.size

#section nested_index | int 3
[[1, 2], [3, 4]][1][0]

#section multiple_assignment | int 3
ma, mb = 1, 2
ma + mb

#section swap | int 2
sa2, sb2 = 1, 2
sa2, sb2 = sb2, sa2
sa2

#section masgn_splat | int 2
msa, *msb = [1, 2, 3]
msb.size

#section op_assign | int 3
oa = 1
oa += 2
oa

#section or_assign | int 5
ora = nil
ora ||= 5
ora

#section and_assign | int 5
aaa = 1
aaa &&= 5
aaa

#section index_assign | int 9
ia = [1, 2]
ia[0] = 9
ia[0]

#section hash_index_assign | int 3
hia = {}
hia[:k] = 3
hia[:k]

#section big_integer | int 4294967296123
4294967296123

#section float_arithmetic | int 3
(1.5 + 1.5).to_i

#section boolean_and | int 0
true && false ? 1 : 0

#section global_variable | int 21
$cov_lang_g = 21
$cov_lang_g

#section string_length | int 5
"hello".length

#section string_upcase | str "AB"
"ab".upcase

#section string_split | int 3
"a,b,c".split(",").size

#section string_include | int 1
"hello".include?("ell") ? 1 : 0

#section string_to_i | int 42
"42".to_i

#section string_times | int 4
("ab" * 2).size

#section string_index | str "e"
"hello"[1]

#section array_map | int 3
[1, 2, 3].map { |x| x * 2 }.size

#section array_select | int 2
[1, 2, 3].select { |x| x > 1 }.size

#broken array_inject | int 6 | NoMethodError: Array#inject
[1, 2, 3].inject(0) { |s, x| s + x }

#section array_push_pop | int 1
ap = [1]
ap.push(2)
ap.pop
ap.size

#section array_join | str "1-2"
[1, 2].join("-")

#section array_include | int 1
[1, 2].include?(2) ? 1 : 0

#section array_first_last | int 4
[1, 2, 3].first + [1, 2, 3].last

#broken array_reverse | int 3 | NoMethodError: Array#reverse
[1, 2, 3].reverse.first

#section array_flatten | int 2
[[1], [2]].flatten.size

#section array_uniq | int 2
[1, 1, 2].uniq.size

#section hash_keys_values | int 3
{ a: 1, b: 2 }.keys.size + { a: 1 }.values.size

#section hash_each | int 3
hsum = 0
{ a: 1, b: 2 }.each { |k, v| hsum += v }
hsum

#section hash_merge | int 2
{ a: 1 }.merge({ b: 2 }).size

#section integer_times | int 3
tsum = 0
3.times { |i| tsum += i }
tsum

#section integer_to_s | str "42"
42.to_s

#section integer_abs | int 3
(-3).abs

#section enumerable_each_with_index | int 1
ewi = 0
[10, 20].each_with_index { |v, i| ewi += i }
ewi

#broken comparable_between | int 1 | NoMethodError: Integer#between?
5.between?(1, 10) ? 1 : 0

#section kernel_puts_returns_nil | int 1
puts("").nil? ? 1 : 0

#broken post_argument | int 4 | Internal error: a parameter after a rest parameter is left unassigned
def cov_post(a, *b, c)
  a + c
end

cov_post(1, 2, 3, 4)

#broken masgn_with_post | int 5 | Internal error: APOST with three operands is not implemented
cov_masgn_src = [1, 2, 3, 4]
cov_head, *cov_mid, cov_tail = cov_masgn_src
cov_mid.size + cov_tail

#broken nested_constant_path | int 1 | NameError: a constant two namespaces deep is not found
module CovOuterNs
  module CovInnerNs
    V = 1
  end
end

CovOuterNs::CovInnerNs::V

#broken logical_not_on_false | int 1 | NoMethodError: FalseClass has no !
!false ? 1 : 0

#broken next_inside_ensure | int 2 | evaluates to 0: next inside an ensure skips the ensure body
cov_next_count = 0
[1, 2].each { |i| begin; next; ensure; cov_next_count += 1; end }
cov_next_count
