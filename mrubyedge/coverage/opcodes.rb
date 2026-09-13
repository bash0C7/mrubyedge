# Every opcode mruby 4.0 emits, as Ruby that runs.
#
# Each section below is a standalone program. The line above it names the
# section, the value it evaluates to, and the opcodes compiling it emits.
# tests/opcode_coverage.rs reads this file, compiles each section on its own, checks those
# opcodes are in the chunk, runs it, and checks the value.
#
# The whole file is also valid Ruby: the markers are comments.
#
# Seven opcodes are in the table and no Ruby reaches them. mruby's code
# generator has no site that emits them, or the parser rejects the shape
# that would. The test checks they stay absent.
#absent CALL GETSV SETSV ASET SYMBOL DEBUG ERR

#section move_and_load | int 1 | LOADI_1 MOVE RETURN STOP
a_move = 1
b_move = a_move
b_move

#section small_integers | int 27 | ADD LOADI_0 LOADI_1 LOADI_2 LOADI_3 LOADI_4 LOADI_5 LOADI_6 LOADI_7 LOADI__1 MOVE RETURN STOP
z0 = 0
z1 = 1
z2 = 2
z3 = 3
z4 = 4
z5 = 5
z6 = 6
z7 = 7
zm1 = -1
z0 + z1 + z2 + z3 + z4 + z5 + z6 + z7 + zm1

#section wide_integers | int 70293 | ADD LOADI16 LOADI32 LOADINEG MOVE RETURN STOP
w16 = 300
w32 = 70000
wneg = -7
w16 + w32 + wneg

#section float_from_pool | int 1 | LOADL RETURN STOP
1.5

#section singletons | str "abc" | JMP JMPNOT LOADF LOADNIL LOADSYM LOADT MOVE RETURN SEND0 STOP
n_nil = nil
t_true = true
f_false = false
s_sym = :abc
n_nil.nil? && t_true ? s_sym.to_s : f_false.to_s

#section global_variable | int 21 | GETGV LOADI RETURN SETGV STOP
$cov_global = 21
$cov_global

#section instance_variable | int 5 | CLASS ENTER EXEC GETCONST GETIV LOADI_5 LOADNIL MOVE RETURN SEND0 SETIV STOP TDEF
class CovIvar
  def put
    @v = 5
  end

  def get
    @v
  end
end

cov_ivar = CovIvar.new
cov_ivar.put
cov_ivar.get

#section class_variable | int 6 | CLASS ENTER EXEC GETCONST GETCV LOADI_6 LOADNIL MOVE RETURN SEND0 SETCV STOP TDEF
class CovCvar
  def put
    @@v = 6
  end

  def get
    @@v
  end
end

cov_cvar = CovCvar.new
cov_cvar.put
cov_cvar.get

#section constant | int 7 | GETCONST LOADI_7 RETURN SETCONST STOP
COV_CONST = 7
COV_CONST

#section scoped_constant | int 8 | CLASS GETCONST GETMCNST LOADI LOADNIL RETURN SETMCNST STOP
class CovScope
end

CovScope::LIMIT = 8
CovScope::LIMIT

#section toplevel_scope | int 9 | GETMCNST LOADI OCLASS RETURN SETMCNST STOP
::COV_TOPLEVEL = 9
::COV_TOPLEVEL

#section upvalue | int 10 | ARRAY BLOCK ENTER GETUPVAR LOADI LOADI_0 LOADI_1 RETURN SENDB SETUPVAR STOP
up_read = 10
up_write = 0
[1].each { up_write = up_read }
up_write

#section branch_if_else | int 11 | JMP JMPNOT LOADI LOADI_1 MOVE RETURN STOP
cov_branch = 1
if cov_branch then 11 else 22 end

#section branch_or | int 12 | JMPIF LOADI LOADNIL MOVE RETURN STOP
cov_or = nil
cov_or || 12

#section branch_on_nil | int 13 | JMPNIL LOADI LOADNIL MOVE RETURN SEND0 STOP
cov_nil = nil
cov_nil&.size
13

#section while_loop | int 2 | ADDILV JMP JMPNOT LOADI_0 LOADI_2 LT MOVE NOP RETURN STOP
cov_i = 0
while cov_i < 2
  cov_i += 1
end
cov_i

#section break_from_block | int 1 | ARRAY BLOCK BREAK ENTER LOADI_0 LOADI_1 LOADI_2 LOADNIL RETURN SENDB SETUPVAR STOP
cov_break = 0
[1, 2].each { |x| cov_break = x; break }
cov_break

#section send_forms | int 5 | ADD ARRAY ENTER LOADI_1 LOADI_2 RETURN SEND0 SSEND SSEND0 STOP TDEF
def cov_one(a)
  a
end

def cov_zero
  1
end

cov_one(2) + cov_zero + [1, 2].size

#section yield_to_block | int 6 | BLKCALL BLKPUSH BLOCK ENTER LOADI_2 LOADI_3 MOVE MUL RETURN SSENDB STOP TDEF
def cov_yield
  yield 3
end

cov_yield { |x| x * 2 }

#section block_object | int 14 | ENTER LAMBDA LOADI_2 LOADI_7 MOVE MUL RETURN SEND STOP
cov_lambda = ->(a) { a * 2 }
cov_lambda.call(7)

#section super_call | int 16 | ARGARY CLASS ENTER EXEC GETCONST LOADI LOADI_2 LOADNIL MOVE MUL RETURN SEND SEND0 STOP SUPER TDEF
class CovBase
  def twice(n)
    n * 2
  end
end

class CovSub < CovBase
  def twice(n)
    super
  end
end

CovSub.new.twice(8)

#section keyword_arguments | int 3 | ADD ENTER KARG KEYEND LOADI_1 LOADI_2 LOADSYM MOVE RETURN SSEND STOP TDEF
def cov_kw(a:, b:)
  a + b
end

cov_kw(a: 1, b: 2)

#section returns | int 4 | CLASS ENTER EXEC GETCONST JMP JMPNOT LOADI_0 LOADI_4 LOADNIL MOVE RETNIL RETTRUE RETURN SEND0 STOP TDEF
class CovRet
  def plain
    return 4
  end

  def none
    nil
  end

  def yes
    true
  end
end

cov_ret = CovRet.new
cov_ret.yes && cov_ret.none.nil? ? cov_ret.plain : 0

#section return_out_of_begin | int 5 | ENTER EXCEPT GETGV GETMCNST JMP JMPNOT LOADI_5 OCLASS RAISEIF RESCUE RETURN RETURN_BLK SETGV SSEND0 STOP TDEF
def cov_ret_blk
  begin
    return 5
  ensure
    6
  end
end

cov_ret_blk

#section arithmetic | int 99 | ADDI ADDILV DIV LOADI_1 LOADI_2 LOADI_4 MOVE MUL RETURN STOP SUBI
ar = 1
ar += 2
br = ar - 1
cr = br + 200
dr = cr - 3
dr * 2 / 4

#section inline_arithmetic | int 2 | ADD ADDI ENTER LOADI_1 MOVE RETURN SSEND STOP SUBI TDEF
def cov_inc(x)
  x + 1
end

def cov_dec(x)
  x - 1
end

cov_inc(1) + cov_dec(1)

#section comparisons | int 1 | EQ GE GT JMP JMPNOT LE LOADI_0 LOADI_1 LOADI_2 LT RETURN STOP
(1 == 1) && (1 < 2) && (1 <= 1) && (2 > 1) && (2 >= 2) ? 1 : 0

#section array_literal | int 3 | ARRAY2 LOADI_1 LOADI_2 LOADI_3 MOVE RETURN SEND0 STOP
ae = 1
arr = [ae, 2, 3]
arr.size

#section array_index | int 7 | ADD ARRAY2 GETIDX GETIDX0 LOADI_0 LOADI_1 LOADI_2 LOADI_5 MOVE RETURN SETIDX STOP
ix = [1, 2]
ix[0] = 5
ix[0] + ix[1]

#section array_destructure | int 6 | ADD AREF ARRAY2 LOADI_1 LOADI_2 LOADI_3 MOVE RETURN STOP
src = [1, 2, 3]
first_e, second_e, third_e = src
first_e + second_e + third_e

#section array_rest | int 3 | ADD APOST AREF ARRAY2 LOADI_1 LOADI_2 LOADI_3 MOVE RETURN SEND0 STOP
rest_src = [1, 2, 3]
head_e, *tail_e = rest_src
head_e + tail_e.size

#section array_splat | int 5 | ADD ARRAY ARRAY2 ARYCAT ARYPUSH ARYSPLAT ENTER LOADI_1 LOADI_2 LOADI_3 LOADI_4 MOVE RETURN SEND0 SSEND STOP TDEF
spread = [2, 3]
joined = [1, *spread, 4]

def cov_spread(x)
  return *x
end

joined.size + cov_spread([1]).size

#section string_interpolation | str "x1y" | LOADI_1 MOVE RETURN STOP STRCAT STRING
si = 1
"x#{si}y"

#section dynamic_symbol | str "a1" | INTERN LOADI_1 MOVE RETURN SEND0 STOP STRCAT STRING
dn = 1
:"a#{dn}".to_s

#section hash_literal | int 3 | HASH HASHADD HASHCAT LOADI_1 LOADI_2 LOADI_3 LOADSYM MOVE RETURN SEND0 STOP
base_h = { a: 1 }
extra_h = { b: 2 }
merged_h = { **base_h, **extra_h, c: 3 }
merged_h.size

#section ranges | int 5 | ADD LOADI_1 LOADI_3 RANGE_EXC RANGE_INC RETURN SEND0 STOP
(1..3).to_a.size + (1...3).to_a.size

#section class_and_module | int 14 | CLASS LOADI LOADNIL MODULE RETURN STOP
class CovClass
end

module CovModule
end

14

#section method_definitions | int 3 | ADD CLASS ENTER EXEC GETCONST LOADI_1 LOADI_2 LOADNIL LOADSELF RETURN SDEF SEND0 STOP TDEF
class CovDef
  def inst
    1
  end

  def self.klass
    2
  end
end

CovDef.new.inst + CovDef.klass

#section singleton_class_body | int 15 | CLASS ENTER EXEC GETCONST LOADI LOADNIL LOADSELF RETURN SCLASS SEND0 STOP TDEF
class CovSingleton
  class << self
    def build
      15
    end
  end
end

CovSingleton.build

#section alias_and_undef | int 16 | ALIAS CLASS ENTER EXEC GETCONST LOADI LOADI_0 LOADNIL RETNIL RETURN SEND0 STOP TDEF UNDEF
class CovAlias
  def original
    16
  end
  alias renamed original

  def dropped
    0
  end
  undef dropped
end

CovAlias.new.renamed

#section exceptions | str "boom" | EXCEPT GETCONST GETGV JMP JMPIF MOVE RAISEIF RESCUE RETURN SEND0 SETGV SSEND STOP STRING
begin
  raise "boom"
rescue => e
  e.message
end

#section pattern_match_value | int 20 | JMP JMPNOT LOADF LOADI LOADI_1 LOADI_2 MATCHERR MOVE RETURN SEND STOP
pm = 2
case pm
in 1 then 10
in 2 then 20
end

#section definition_past_symbol_limit | int 42 | ARRAY BLOCK DEF ENTER EXT2 LOADI LOADI_1 METHOD RETURN SENDB SSEND0 STOP TCLASS
[1].each { |q0| q0 }
[1].each { |q1| q1 }
[1].each { |q2| q2 }
[1].each { |q3| q3 }
[1].each { |q4| q4 }
[1].each { |q5| q5 }
[1].each { |q6| q6 }
[1].each { |q7| q7 }
[1].each { |q8| q8 }
[1].each { |q9| q9 }
[1].each { |q10| q10 }
[1].each { |q11| q11 }
[1].each { |q12| q12 }
[1].each { |q13| q13 }
[1].each { |q14| q14 }
[1].each { |q15| q15 }
[1].each { |q16| q16 }
[1].each { |q17| q17 }
[1].each { |q18| q18 }
[1].each { |q19| q19 }
[1].each { |q20| q20 }
[1].each { |q21| q21 }
[1].each { |q22| q22 }
[1].each { |q23| q23 }
[1].each { |q24| q24 }
[1].each { |q25| q25 }
[1].each { |q26| q26 }
[1].each { |q27| q27 }
[1].each { |q28| q28 }
[1].each { |q29| q29 }
[1].each { |q30| q30 }
[1].each { |q31| q31 }
[1].each { |q32| q32 }
[1].each { |q33| q33 }
[1].each { |q34| q34 }
[1].each { |q35| q35 }
[1].each { |q36| q36 }
[1].each { |q37| q37 }
[1].each { |q38| q38 }
[1].each { |q39| q39 }
[1].each { |q40| q40 }
[1].each { |q41| q41 }
[1].each { |q42| q42 }
[1].each { |q43| q43 }
[1].each { |q44| q44 }
[1].each { |q45| q45 }
[1].each { |q46| q46 }
[1].each { |q47| q47 }
[1].each { |q48| q48 }
[1].each { |q49| q49 }
[1].each { |q50| q50 }
[1].each { |q51| q51 }
[1].each { |q52| q52 }
[1].each { |q53| q53 }
[1].each { |q54| q54 }
[1].each { |q55| q55 }
[1].each { |q56| q56 }
[1].each { |q57| q57 }
[1].each { |q58| q58 }
[1].each { |q59| q59 }
[1].each { |q60| q60 }
[1].each { |q61| q61 }
[1].each { |q62| q62 }
[1].each { |q63| q63 }
[1].each { |q64| q64 }
[1].each { |q65| q65 }
[1].each { |q66| q66 }
[1].each { |q67| q67 }
[1].each { |q68| q68 }
[1].each { |q69| q69 }
[1].each { |q70| q70 }
[1].each { |q71| q71 }
[1].each { |q72| q72 }
[1].each { |q73| q73 }
[1].each { |q74| q74 }
[1].each { |q75| q75 }
[1].each { |q76| q76 }
[1].each { |q77| q77 }
[1].each { |q78| q78 }
[1].each { |q79| q79 }
[1].each { |q80| q80 }
[1].each { |q81| q81 }
[1].each { |q82| q82 }
[1].each { |q83| q83 }
[1].each { |q84| q84 }
[1].each { |q85| q85 }
[1].each { |q86| q86 }
[1].each { |q87| q87 }
[1].each { |q88| q88 }
[1].each { |q89| q89 }
[1].each { |q90| q90 }
[1].each { |q91| q91 }
[1].each { |q92| q92 }
[1].each { |q93| q93 }
[1].each { |q94| q94 }
[1].each { |q95| q95 }
[1].each { |q96| q96 }
[1].each { |q97| q97 }
[1].each { |q98| q98 }
[1].each { |q99| q99 }
[1].each { |q100| q100 }
[1].each { |q101| q101 }
[1].each { |q102| q102 }
[1].each { |q103| q103 }
[1].each { |q104| q104 }
[1].each { |q105| q105 }
[1].each { |q106| q106 }
[1].each { |q107| q107 }
[1].each { |q108| q108 }
[1].each { |q109| q109 }
[1].each { |q110| q110 }
[1].each { |q111| q111 }
[1].each { |q112| q112 }
[1].each { |q113| q113 }
[1].each { |q114| q114 }
[1].each { |q115| q115 }
[1].each { |q116| q116 }
[1].each { |q117| q117 }
[1].each { |q118| q118 }
[1].each { |q119| q119 }
[1].each { |q120| q120 }
[1].each { |q121| q121 }
[1].each { |q122| q122 }
[1].each { |q123| q123 }
[1].each { |q124| q124 }
[1].each { |q125| q125 }
[1].each { |q126| q126 }
[1].each { |q127| q127 }
[1].each { |q128| q128 }
[1].each { |q129| q129 }
[1].each { |q130| q130 }
[1].each { |q131| q131 }
[1].each { |q132| q132 }
[1].each { |q133| q133 }
[1].each { |q134| q134 }
[1].each { |q135| q135 }
[1].each { |q136| q136 }
[1].each { |q137| q137 }
[1].each { |q138| q138 }
[1].each { |q139| q139 }
[1].each { |q140| q140 }
[1].each { |q141| q141 }
[1].each { |q142| q142 }
[1].each { |q143| q143 }
[1].each { |q144| q144 }
[1].each { |q145| q145 }
[1].each { |q146| q146 }
[1].each { |q147| q147 }
[1].each { |q148| q148 }
[1].each { |q149| q149 }
[1].each { |q150| q150 }
[1].each { |q151| q151 }
[1].each { |q152| q152 }
[1].each { |q153| q153 }
[1].each { |q154| q154 }
[1].each { |q155| q155 }
[1].each { |q156| q156 }
[1].each { |q157| q157 }
[1].each { |q158| q158 }
[1].each { |q159| q159 }
[1].each { |q160| q160 }
[1].each { |q161| q161 }
[1].each { |q162| q162 }
[1].each { |q163| q163 }
[1].each { |q164| q164 }
[1].each { |q165| q165 }
[1].each { |q166| q166 }
[1].each { |q167| q167 }
[1].each { |q168| q168 }
[1].each { |q169| q169 }
[1].each { |q170| q170 }
[1].each { |q171| q171 }
[1].each { |q172| q172 }
[1].each { |q173| q173 }
[1].each { |q174| q174 }
[1].each { |q175| q175 }
[1].each { |q176| q176 }
[1].each { |q177| q177 }
[1].each { |q178| q178 }
[1].each { |q179| q179 }
[1].each { |q180| q180 }
[1].each { |q181| q181 }
[1].each { |q182| q182 }
[1].each { |q183| q183 }
[1].each { |q184| q184 }
[1].each { |q185| q185 }
[1].each { |q186| q186 }
[1].each { |q187| q187 }
[1].each { |q188| q188 }
[1].each { |q189| q189 }
[1].each { |q190| q190 }
[1].each { |q191| q191 }
[1].each { |q192| q192 }
[1].each { |q193| q193 }
[1].each { |q194| q194 }
[1].each { |q195| q195 }
[1].each { |q196| q196 }
[1].each { |q197| q197 }
[1].each { |q198| q198 }
[1].each { |q199| q199 }
[1].each { |q200| q200 }
[1].each { |q201| q201 }
[1].each { |q202| q202 }
[1].each { |q203| q203 }
[1].each { |q204| q204 }
[1].each { |q205| q205 }
[1].each { |q206| q206 }
[1].each { |q207| q207 }
[1].each { |q208| q208 }
[1].each { |q209| q209 }
[1].each { |q210| q210 }
[1].each { |q211| q211 }
[1].each { |q212| q212 }
[1].each { |q213| q213 }
[1].each { |q214| q214 }
[1].each { |q215| q215 }
[1].each { |q216| q216 }
[1].each { |q217| q217 }
[1].each { |q218| q218 }
[1].each { |q219| q219 }
[1].each { |q220| q220 }
[1].each { |q221| q221 }
[1].each { |q222| q222 }
[1].each { |q223| q223 }
[1].each { |q224| q224 }
[1].each { |q225| q225 }
[1].each { |q226| q226 }
[1].each { |q227| q227 }
[1].each { |q228| q228 }
[1].each { |q229| q229 }
[1].each { |q230| q230 }
[1].each { |q231| q231 }
[1].each { |q232| q232 }
[1].each { |q233| q233 }
[1].each { |q234| q234 }
[1].each { |q235| q235 }
[1].each { |q236| q236 }
[1].each { |q237| q237 }
[1].each { |q238| q238 }
[1].each { |q239| q239 }
[1].each { |q240| q240 }
[1].each { |q241| q241 }
[1].each { |q242| q242 }
[1].each { |q243| q243 }
[1].each { |q244| q244 }
[1].each { |q245| q245 }
[1].each { |q246| q246 }
[1].each { |q247| q247 }
[1].each { |q248| q248 }
[1].each { |q249| q249 }
[1].each { |q250| q250 }
[1].each { |q251| q251 }
[1].each { |q252| q252 }
[1].each { |q253| q253 }
[1].each { |q254| q254 }
[1].each { |q255| q255 }
def cov_late
  42
end

cov_late

#section wide_first_operand | int 249 | ARRAY2 EXT1 GETIDX LOADI LOADI_0 LOADI_1 LOADI_2 LOADI_3 LOADI_4 LOADI_5 LOADI_6 LOADI_7 MOVE RETURN STOP
e1_0 = 0
e1_1 = 1
e1_2 = 2
e1_3 = 3
e1_4 = 4
e1_5 = 5
e1_6 = 6
e1_7 = 7
e1_8 = 8
e1_9 = 9
e1_10 = 10
e1_11 = 11
e1_12 = 12
e1_13 = 13
e1_14 = 14
e1_15 = 15
e1_16 = 16
e1_17 = 17
e1_18 = 18
e1_19 = 19
e1_20 = 20
e1_21 = 21
e1_22 = 22
e1_23 = 23
e1_24 = 24
e1_25 = 25
e1_26 = 26
e1_27 = 27
e1_28 = 28
e1_29 = 29
e1_30 = 30
e1_31 = 31
e1_32 = 32
e1_33 = 33
e1_34 = 34
e1_35 = 35
e1_36 = 36
e1_37 = 37
e1_38 = 38
e1_39 = 39
e1_40 = 40
e1_41 = 41
e1_42 = 42
e1_43 = 43
e1_44 = 44
e1_45 = 45
e1_46 = 46
e1_47 = 47
e1_48 = 48
e1_49 = 49
e1_50 = 50
e1_51 = 51
e1_52 = 52
e1_53 = 53
e1_54 = 54
e1_55 = 55
e1_56 = 56
e1_57 = 57
e1_58 = 58
e1_59 = 59
e1_60 = 60
e1_61 = 61
e1_62 = 62
e1_63 = 63
e1_64 = 64
e1_65 = 65
e1_66 = 66
e1_67 = 67
e1_68 = 68
e1_69 = 69
e1_70 = 70
e1_71 = 71
e1_72 = 72
e1_73 = 73
e1_74 = 74
e1_75 = 75
e1_76 = 76
e1_77 = 77
e1_78 = 78
e1_79 = 79
e1_80 = 80
e1_81 = 81
e1_82 = 82
e1_83 = 83
e1_84 = 84
e1_85 = 85
e1_86 = 86
e1_87 = 87
e1_88 = 88
e1_89 = 89
e1_90 = 90
e1_91 = 91
e1_92 = 92
e1_93 = 93
e1_94 = 94
e1_95 = 95
e1_96 = 96
e1_97 = 97
e1_98 = 98
e1_99 = 99
e1_100 = 100
e1_101 = 101
e1_102 = 102
e1_103 = 103
e1_104 = 104
e1_105 = 105
e1_106 = 106
e1_107 = 107
e1_108 = 108
e1_109 = 109
e1_110 = 110
e1_111 = 111
e1_112 = 112
e1_113 = 113
e1_114 = 114
e1_115 = 115
e1_116 = 116
e1_117 = 117
e1_118 = 118
e1_119 = 119
e1_120 = 120
e1_121 = 121
e1_122 = 122
e1_123 = 123
e1_124 = 124
e1_125 = 125
e1_126 = 126
e1_127 = 127
e1_128 = 128
e1_129 = 129
e1_130 = 130
e1_131 = 131
e1_132 = 132
e1_133 = 133
e1_134 = 134
e1_135 = 135
e1_136 = 136
e1_137 = 137
e1_138 = 138
e1_139 = 139
e1_140 = 140
e1_141 = 141
e1_142 = 142
e1_143 = 143
e1_144 = 144
e1_145 = 145
e1_146 = 146
e1_147 = 147
e1_148 = 148
e1_149 = 149
e1_150 = 150
e1_151 = 151
e1_152 = 152
e1_153 = 153
e1_154 = 154
e1_155 = 155
e1_156 = 156
e1_157 = 157
e1_158 = 158
e1_159 = 159
e1_160 = 160
e1_161 = 161
e1_162 = 162
e1_163 = 163
e1_164 = 164
e1_165 = 165
e1_166 = 166
e1_167 = 167
e1_168 = 168
e1_169 = 169
e1_170 = 170
e1_171 = 171
e1_172 = 172
e1_173 = 173
e1_174 = 174
e1_175 = 175
e1_176 = 176
e1_177 = 177
e1_178 = 178
e1_179 = 179
e1_180 = 180
e1_181 = 181
e1_182 = 182
e1_183 = 183
e1_184 = 184
e1_185 = 185
e1_186 = 186
e1_187 = 187
e1_188 = 188
e1_189 = 189
e1_190 = 190
e1_191 = 191
e1_192 = 192
e1_193 = 193
e1_194 = 194
e1_195 = 195
e1_196 = 196
e1_197 = 197
e1_198 = 198
e1_199 = 199
e1_200 = 200
e1_201 = 201
e1_202 = 202
e1_203 = 203
e1_204 = 204
e1_205 = 205
e1_206 = 206
e1_207 = 207
e1_208 = 208
e1_209 = 209
e1_210 = 210
e1_211 = 211
e1_212 = 212
e1_213 = 213
e1_214 = 214
e1_215 = 215
e1_216 = 216
e1_217 = 217
e1_218 = 218
e1_219 = 219
e1_220 = 220
e1_221 = 221
e1_222 = 222
e1_223 = 223
e1_224 = 224
e1_225 = 225
e1_226 = 226
e1_227 = 227
e1_228 = 228
e1_229 = 229
e1_230 = 230
e1_231 = 231
e1_232 = 232
e1_233 = 233
e1_234 = 234
e1_235 = 235
e1_236 = 236
e1_237 = 237
e1_238 = 238
e1_239 = 239
e1_240 = 240
e1_241 = 241
e1_242 = 242
e1_243 = 243
e1_244 = 244
e1_245 = 245
e1_246 = 246
e1_247 = 247
e1_248 = 248
e1_249 = 249
wide1 = [e1_240, e1_241, e1_242, e1_243, e1_244, e1_245, e1_246, e1_247, e1_248, e1_249]
wide1[9]

#section wide_second_operand | int 299 | EXT2 GETIV LOADI LOADI16 LOADI_0 LOADI_1 LOADI_2 LOADI_3 LOADI_4 LOADI_5 LOADI_6 LOADI_7 RETURN SETIV STOP
@e2_0 = 0
@e2_1 = 1
@e2_2 = 2
@e2_3 = 3
@e2_4 = 4
@e2_5 = 5
@e2_6 = 6
@e2_7 = 7
@e2_8 = 8
@e2_9 = 9
@e2_10 = 10
@e2_11 = 11
@e2_12 = 12
@e2_13 = 13
@e2_14 = 14
@e2_15 = 15
@e2_16 = 16
@e2_17 = 17
@e2_18 = 18
@e2_19 = 19
@e2_20 = 20
@e2_21 = 21
@e2_22 = 22
@e2_23 = 23
@e2_24 = 24
@e2_25 = 25
@e2_26 = 26
@e2_27 = 27
@e2_28 = 28
@e2_29 = 29
@e2_30 = 30
@e2_31 = 31
@e2_32 = 32
@e2_33 = 33
@e2_34 = 34
@e2_35 = 35
@e2_36 = 36
@e2_37 = 37
@e2_38 = 38
@e2_39 = 39
@e2_40 = 40
@e2_41 = 41
@e2_42 = 42
@e2_43 = 43
@e2_44 = 44
@e2_45 = 45
@e2_46 = 46
@e2_47 = 47
@e2_48 = 48
@e2_49 = 49
@e2_50 = 50
@e2_51 = 51
@e2_52 = 52
@e2_53 = 53
@e2_54 = 54
@e2_55 = 55
@e2_56 = 56
@e2_57 = 57
@e2_58 = 58
@e2_59 = 59
@e2_60 = 60
@e2_61 = 61
@e2_62 = 62
@e2_63 = 63
@e2_64 = 64
@e2_65 = 65
@e2_66 = 66
@e2_67 = 67
@e2_68 = 68
@e2_69 = 69
@e2_70 = 70
@e2_71 = 71
@e2_72 = 72
@e2_73 = 73
@e2_74 = 74
@e2_75 = 75
@e2_76 = 76
@e2_77 = 77
@e2_78 = 78
@e2_79 = 79
@e2_80 = 80
@e2_81 = 81
@e2_82 = 82
@e2_83 = 83
@e2_84 = 84
@e2_85 = 85
@e2_86 = 86
@e2_87 = 87
@e2_88 = 88
@e2_89 = 89
@e2_90 = 90
@e2_91 = 91
@e2_92 = 92
@e2_93 = 93
@e2_94 = 94
@e2_95 = 95
@e2_96 = 96
@e2_97 = 97
@e2_98 = 98
@e2_99 = 99
@e2_100 = 100
@e2_101 = 101
@e2_102 = 102
@e2_103 = 103
@e2_104 = 104
@e2_105 = 105
@e2_106 = 106
@e2_107 = 107
@e2_108 = 108
@e2_109 = 109
@e2_110 = 110
@e2_111 = 111
@e2_112 = 112
@e2_113 = 113
@e2_114 = 114
@e2_115 = 115
@e2_116 = 116
@e2_117 = 117
@e2_118 = 118
@e2_119 = 119
@e2_120 = 120
@e2_121 = 121
@e2_122 = 122
@e2_123 = 123
@e2_124 = 124
@e2_125 = 125
@e2_126 = 126
@e2_127 = 127
@e2_128 = 128
@e2_129 = 129
@e2_130 = 130
@e2_131 = 131
@e2_132 = 132
@e2_133 = 133
@e2_134 = 134
@e2_135 = 135
@e2_136 = 136
@e2_137 = 137
@e2_138 = 138
@e2_139 = 139
@e2_140 = 140
@e2_141 = 141
@e2_142 = 142
@e2_143 = 143
@e2_144 = 144
@e2_145 = 145
@e2_146 = 146
@e2_147 = 147
@e2_148 = 148
@e2_149 = 149
@e2_150 = 150
@e2_151 = 151
@e2_152 = 152
@e2_153 = 153
@e2_154 = 154
@e2_155 = 155
@e2_156 = 156
@e2_157 = 157
@e2_158 = 158
@e2_159 = 159
@e2_160 = 160
@e2_161 = 161
@e2_162 = 162
@e2_163 = 163
@e2_164 = 164
@e2_165 = 165
@e2_166 = 166
@e2_167 = 167
@e2_168 = 168
@e2_169 = 169
@e2_170 = 170
@e2_171 = 171
@e2_172 = 172
@e2_173 = 173
@e2_174 = 174
@e2_175 = 175
@e2_176 = 176
@e2_177 = 177
@e2_178 = 178
@e2_179 = 179
@e2_180 = 180
@e2_181 = 181
@e2_182 = 182
@e2_183 = 183
@e2_184 = 184
@e2_185 = 185
@e2_186 = 186
@e2_187 = 187
@e2_188 = 188
@e2_189 = 189
@e2_190 = 190
@e2_191 = 191
@e2_192 = 192
@e2_193 = 193
@e2_194 = 194
@e2_195 = 195
@e2_196 = 196
@e2_197 = 197
@e2_198 = 198
@e2_199 = 199
@e2_200 = 200
@e2_201 = 201
@e2_202 = 202
@e2_203 = 203
@e2_204 = 204
@e2_205 = 205
@e2_206 = 206
@e2_207 = 207
@e2_208 = 208
@e2_209 = 209
@e2_210 = 210
@e2_211 = 211
@e2_212 = 212
@e2_213 = 213
@e2_214 = 214
@e2_215 = 215
@e2_216 = 216
@e2_217 = 217
@e2_218 = 218
@e2_219 = 219
@e2_220 = 220
@e2_221 = 221
@e2_222 = 222
@e2_223 = 223
@e2_224 = 224
@e2_225 = 225
@e2_226 = 226
@e2_227 = 227
@e2_228 = 228
@e2_229 = 229
@e2_230 = 230
@e2_231 = 231
@e2_232 = 232
@e2_233 = 233
@e2_234 = 234
@e2_235 = 235
@e2_236 = 236
@e2_237 = 237
@e2_238 = 238
@e2_239 = 239
@e2_240 = 240
@e2_241 = 241
@e2_242 = 242
@e2_243 = 243
@e2_244 = 244
@e2_245 = 245
@e2_246 = 246
@e2_247 = 247
@e2_248 = 248
@e2_249 = 249
@e2_250 = 250
@e2_251 = 251
@e2_252 = 252
@e2_253 = 253
@e2_254 = 254
@e2_255 = 255
@e2_256 = 256
@e2_257 = 257
@e2_258 = 258
@e2_259 = 259
@e2_260 = 260
@e2_261 = 261
@e2_262 = 262
@e2_263 = 263
@e2_264 = 264
@e2_265 = 265
@e2_266 = 266
@e2_267 = 267
@e2_268 = 268
@e2_269 = 269
@e2_270 = 270
@e2_271 = 271
@e2_272 = 272
@e2_273 = 273
@e2_274 = 274
@e2_275 = 275
@e2_276 = 276
@e2_277 = 277
@e2_278 = 278
@e2_279 = 279
@e2_280 = 280
@e2_281 = 281
@e2_282 = 282
@e2_283 = 283
@e2_284 = 284
@e2_285 = 285
@e2_286 = 286
@e2_287 = 287
@e2_288 = 288
@e2_289 = 289
@e2_290 = 290
@e2_291 = 291
@e2_292 = 292
@e2_293 = 293
@e2_294 = 294
@e2_295 = 295
@e2_296 = 296
@e2_297 = 297
@e2_298 = 298
@e2_299 = 299
@e2_299

#section wide_both_operands | int 55 | ADD ENTER EXT1 EXT2 EXT3 GETIV LOADI LOADI_0 LOADI_1 LOADI_2 LOADI_3 LOADI_4 LOADI_5 LOADI_6 LOADI_7 RETURN SETIV SSEND0 STOP TDEF
def cov_wide_both
  l0 = 0
  l1 = 1
  l2 = 2
  l3 = 3
  l4 = 4
  l5 = 5
  l6 = 6
  l7 = 7
  l8 = 8
  l9 = 9
  l10 = 10
  l11 = 11
  l12 = 12
  l13 = 13
  l14 = 14
  l15 = 15
  l16 = 16
  l17 = 17
  l18 = 18
  l19 = 19
  l20 = 20
  l21 = 21
  l22 = 22
  l23 = 23
  l24 = 24
  l25 = 25
  l26 = 26
  l27 = 27
  l28 = 28
  l29 = 29
  l30 = 30
  l31 = 31
  l32 = 32
  l33 = 33
  l34 = 34
  l35 = 35
  l36 = 36
  l37 = 37
  l38 = 38
  l39 = 39
  l40 = 40
  l41 = 41
  l42 = 42
  l43 = 43
  l44 = 44
  l45 = 45
  l46 = 46
  l47 = 47
  l48 = 48
  l49 = 49
  l50 = 50
  l51 = 51
  l52 = 52
  l53 = 53
  l54 = 54
  l55 = 55
  l56 = 56
  l57 = 57
  l58 = 58
  l59 = 59
  l60 = 60
  l61 = 61
  l62 = 62
  l63 = 63
  l64 = 64
  l65 = 65
  l66 = 66
  l67 = 67
  l68 = 68
  l69 = 69
  l70 = 70
  l71 = 71
  l72 = 72
  l73 = 73
  l74 = 74
  l75 = 75
  l76 = 76
  l77 = 77
  l78 = 78
  l79 = 79
  l80 = 80
  l81 = 81
  l82 = 82
  l83 = 83
  l84 = 84
  l85 = 85
  l86 = 86
  l87 = 87
  l88 = 88
  l89 = 89
  l90 = 90
  l91 = 91
  l92 = 92
  l93 = 93
  l94 = 94
  l95 = 95
  l96 = 96
  l97 = 97
  l98 = 98
  l99 = 99
  l100 = 100
  l101 = 101
  l102 = 102
  l103 = 103
  l104 = 104
  l105 = 105
  l106 = 106
  l107 = 107
  l108 = 108
  l109 = 109
  l110 = 110
  l111 = 111
  l112 = 112
  l113 = 113
  l114 = 114
  l115 = 115
  l116 = 116
  l117 = 117
  l118 = 118
  l119 = 119
  l120 = 120
  l121 = 121
  l122 = 122
  l123 = 123
  l124 = 124
  l125 = 125
  l126 = 126
  l127 = 127
  l128 = 128
  l129 = 129
  l130 = 130
  l131 = 131
  l132 = 132
  l133 = 133
  l134 = 134
  l135 = 135
  l136 = 136
  l137 = 137
  l138 = 138
  l139 = 139
  l140 = 140
  l141 = 141
  l142 = 142
  l143 = 143
  l144 = 144
  l145 = 145
  l146 = 146
  l147 = 147
  l148 = 148
  l149 = 149
  l150 = 150
  l151 = 151
  l152 = 152
  l153 = 153
  l154 = 154
  l155 = 155
  l156 = 156
  l157 = 157
  l158 = 158
  l159 = 159
  l160 = 160
  l161 = 161
  l162 = 162
  l163 = 163
  l164 = 164
  l165 = 165
  l166 = 166
  l167 = 167
  l168 = 168
  l169 = 169
  l170 = 170
  l171 = 171
  l172 = 172
  l173 = 173
  l174 = 174
  l175 = 175
  l176 = 176
  l177 = 177
  l178 = 178
  l179 = 179
  l180 = 180
  l181 = 181
  l182 = 182
  l183 = 183
  l184 = 184
  l185 = 185
  l186 = 186
  l187 = 187
  l188 = 188
  l189 = 189
  l190 = 190
  l191 = 191
  l192 = 192
  l193 = 193
  l194 = 194
  l195 = 195
  l196 = 196
  l197 = 197
  l198 = 198
  l199 = 199
  l200 = 200
  l201 = 201
  l202 = 202
  l203 = 203
  l204 = 204
  l205 = 205
  l206 = 206
  l207 = 207
  l208 = 208
  l209 = 209
  l210 = 210
  l211 = 211
  l212 = 212
  l213 = 213
  l214 = 214
  l215 = 215
  l216 = 216
  l217 = 217
  l218 = 218
  l219 = 219
  l220 = 220
  l221 = 221
  l222 = 222
  l223 = 223
  l224 = 224
  l225 = 225
  l226 = 226
  l227 = 227
  l228 = 228
  l229 = 229
  l230 = 230
  l231 = 231
  l232 = 232
  l233 = 233
  l234 = 234
  l235 = 235
  l236 = 236
  l237 = 237
  l238 = 238
  l239 = 239
  l240 = 240
  l241 = 241
  l242 = 242
  l243 = 243
  l244 = 244
  l245 = 245
  l246 = 246
  l247 = 247
  l248 = 248
  l249 = 249
  @p0 = 0
  @p1 = 0
  @p2 = 0
  @p3 = 0
  @p4 = 0
  @p5 = 0
  @p6 = 0
  @p7 = 0
  @p8 = 0
  @p9 = 0
  @p10 = 0
  @p11 = 0
  @p12 = 0
  @p13 = 0
  @p14 = 0
  @p15 = 0
  @p16 = 0
  @p17 = 0
  @p18 = 0
  @p19 = 0
  @p20 = 0
  @p21 = 0
  @p22 = 0
  @p23 = 0
  @p24 = 0
  @p25 = 0
  @p26 = 0
  @p27 = 0
  @p28 = 0
  @p29 = 0
  @p30 = 0
  @p31 = 0
  @p32 = 0
  @p33 = 0
  @p34 = 0
  @p35 = 0
  @p36 = 0
  @p37 = 0
  @p38 = 0
  @p39 = 0
  @p40 = 0
  @p41 = 0
  @p42 = 0
  @p43 = 0
  @p44 = 0
  @p45 = 0
  @p46 = 0
  @p47 = 0
  @p48 = 0
  @p49 = 0
  @p50 = 0
  @p51 = 0
  @p52 = 0
  @p53 = 0
  @p54 = 0
  @p55 = 0
  @p56 = 0
  @p57 = 0
  @p58 = 0
  @p59 = 0
  @p60 = 0
  @p61 = 0
  @p62 = 0
  @p63 = 0
  @p64 = 0
  @p65 = 0
  @p66 = 0
  @p67 = 0
  @p68 = 0
  @p69 = 0
  @p70 = 0
  @p71 = 0
  @p72 = 0
  @p73 = 0
  @p74 = 0
  @p75 = 0
  @p76 = 0
  @p77 = 0
  @p78 = 0
  @p79 = 0
  @p80 = 0
  @p81 = 0
  @p82 = 0
  @p83 = 0
  @p84 = 0
  @p85 = 0
  @p86 = 0
  @p87 = 0
  @p88 = 0
  @p89 = 0
  @p90 = 0
  @p91 = 0
  @p92 = 0
  @p93 = 0
  @p94 = 0
  @p95 = 0
  @p96 = 0
  @p97 = 0
  @p98 = 0
  @p99 = 0
  @p100 = 0
  @p101 = 0
  @p102 = 0
  @p103 = 0
  @p104 = 0
  @p105 = 0
  @p106 = 0
  @p107 = 0
  @p108 = 0
  @p109 = 0
  @p110 = 0
  @p111 = 0
  @p112 = 0
  @p113 = 0
  @p114 = 0
  @p115 = 0
  @p116 = 0
  @p117 = 0
  @p118 = 0
  @p119 = 0
  @p120 = 0
  @p121 = 0
  @p122 = 0
  @p123 = 0
  @p124 = 0
  @p125 = 0
  @p126 = 0
  @p127 = 0
  @p128 = 0
  @p129 = 0
  @p130 = 0
  @p131 = 0
  @p132 = 0
  @p133 = 0
  @p134 = 0
  @p135 = 0
  @p136 = 0
  @p137 = 0
  @p138 = 0
  @p139 = 0
  @p140 = 0
  @p141 = 0
  @p142 = 0
  @p143 = 0
  @p144 = 0
  @p145 = 0
  @p146 = 0
  @p147 = 0
  @p148 = 0
  @p149 = 0
  @p150 = 0
  @p151 = 0
  @p152 = 0
  @p153 = 0
  @p154 = 0
  @p155 = 0
  @p156 = 0
  @p157 = 0
  @p158 = 0
  @p159 = 0
  @p160 = 0
  @p161 = 0
  @p162 = 0
  @p163 = 0
  @p164 = 0
  @p165 = 0
  @p166 = 0
  @p167 = 0
  @p168 = 0
  @p169 = 0
  @p170 = 0
  @p171 = 0
  @p172 = 0
  @p173 = 0
  @p174 = 0
  @p175 = 0
  @p176 = 0
  @p177 = 0
  @p178 = 0
  @p179 = 0
  @p180 = 0
  @p181 = 0
  @p182 = 0
  @p183 = 0
  @p184 = 0
  @p185 = 0
  @p186 = 0
  @p187 = 0
  @p188 = 0
  @p189 = 0
  @p190 = 0
  @p191 = 0
  @p192 = 0
  @p193 = 0
  @p194 = 0
  @p195 = 0
  @p196 = 0
  @p197 = 0
  @p198 = 0
  @p199 = 0
  @p200 = 0
  @p201 = 0
  @p202 = 0
  @p203 = 0
  @p204 = 0
  @p205 = 0
  @p206 = 0
  @p207 = 0
  @p208 = 0
  @p209 = 0
  @p210 = 0
  @p211 = 0
  @p212 = 0
  @p213 = 0
  @p214 = 0
  @p215 = 0
  @p216 = 0
  @p217 = 0
  @p218 = 0
  @p219 = 0
  @p220 = 0
  @p221 = 0
  @p222 = 0
  @p223 = 0
  @p224 = 0
  @p225 = 0
  @p226 = 0
  @p227 = 0
  @p228 = 0
  @p229 = 0
  @p230 = 0
  @p231 = 0
  @p232 = 0
  @p233 = 0
  @p234 = 0
  @p235 = 0
  @p236 = 0
  @p237 = 0
  @p238 = 0
  @p239 = 0
  @p240 = 0
  @p241 = 0
  @p242 = 0
  @p243 = 0
  @p244 = 0
  @p245 = 0
  @p246 = 0
  @p247 = 0
  @p248 = 0
  @p249 = 0
  @p250 = 0
  @p251 = 0
  @p252 = 0
  @p253 = 0
  @p254 = 0
  @p255 = 0
  @q0 = 1
  @q1 = 2
  @q2 = 3
  @q3 = 4
  @q4 = 5
  @q5 = 6
  @q6 = 7
  @q7 = 8
  @q8 = 9
  @q9 = 10
  @q0 + (@q1 + (@q2 + (@q3 + (@q4 + (@q5 + (@q6 + (@q7 + (@q8 + @q9))))))))
end

cov_wide_both

#section subtraction | int 3 | LOADI_2 LOADI_5 MOVE RETURN STOP SUB
sa = 5
sb = 2
sa - sb

#section compound_assignment | int 6 | ADDILV ENTER LOADI_5 RETURN SSEND0 STOP SUBILV TDEF
def cov_compound
  ci = 5
  ci -= 1
  ci += 2
  ci
end

cov_compound

#section implicit_returns | int 17 | CLASS ENTER EXEC GETCONST JMP JMPNIL JMPNOT LOADI LOADI_0 LOADNIL MOVE RETFALSE RETSELF RETURN SEND0 STOP TDEF
class CovImplicit
  def itself_
    self
  end

  def no
    false
  end
end

cov_impl = CovImplicit.new
cov_impl.no ? 0 : cov_impl.itself_.nil? ? 0 : 17

#section unwinding_jump | int 1 | ADDILV ENTER EXCEPT GETGV GETMCNST JMP JMPNOT JMPUW LOADI_0 LOADI_3 LT MOVE NOP OCLASS RAISEIF RESCUE RETURN SETGV SSEND0 STOP TDEF
def cov_unwind
  ui = 0
  while ui < 3
    begin
      break
    ensure
      ui += 1
    end
  end
  ui
end

cov_unwind

#section optional_keyword | int 7 | ADD ENTER JMP JMPIF KARG KEYEND KEY_P LOADI_1 LOADI_2 LOADI_5 LOADSYM MOVE RETURN SSEND STOP TDEF
def cov_optkw(a: 1, b: 2)
  a + b
end

cov_optkw(a: 5)

