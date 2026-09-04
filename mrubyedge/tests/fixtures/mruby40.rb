# Reaches every opcode mruby 4.0 (RITE0400) added, except MATCHERR.
# mrubyedge/tests/fixtures/mruby40.mrb is this file compiled by PicoRuby
# 4.0.3's mrbc; see tests/rite0400.rs.
class Counter
  def initialize(start)   # TDEF
    @n = start
  end

  def bump(by)
    @n = @n + by
    self                  # RETSELF
  end

  def value
    @n
  end

  def empty
  end                     # RETNIL

  def yes
    true                  # RETTRUE
  end

  def no
    false                 # RETFALSE
  end

  def self.build(start)   # SDEF
    new(start)
  end
end

def locals
  a = 1
  b = 2
  c = 3
  a += 4                  # ADDILV
  b -= 1                  # SUBILV
  [a, b, c].join("-")
end

def first_of(list)
  list[0]                 # GETIDX0
end

def through_block
  yield 3                 # BLKPUSH + BLKCALL
end

def bare                  # called through SSEND0
  7
end

counter = Counter.build(10)
counter.bump(5)
counter.bump(-3)

parts = [
  counter.value,          # SEND0
  locals,
  first_of([1, 2, 3]),
  through_block { |x| x * 2 },
  bare,
  counter.empty.nil? ? 1 : 0,
  counter.yes ? 1 : 0,
  counter.no ? 0 : 1
]
parts.join(",")
