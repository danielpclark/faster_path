require 'test_helper'
require 'minitest/benchmark'

class Pbench # < Minitest::Benchmark
  def initialize(_) # if for whatever reason we inherit from Minitest::Benchmark - they take 1 parameter
  end

  def self.io # :nodoc:
    @io = $stdout
  end

  def io # :nodoc:
    self.class.io
  end

  def self.bench_range
    # Looking for a consistent result
    # which seems to require more of the same
    [10_000] * 5
  end

  def performance(baseline, new_impl)
    range = self.class.bench_range

    times_a = []
    range.each do |x|
      GC.start
      t0 = Minitest.clock_time
      baseline.call(x)
      t = Minitest.clock_time - t0
      times_a << t
    end

    # This seems to stabalize the results a bit
    sleep 0.02; GC.start

    times_b = []
    range.each do |x|
      GC.start
      t0 = Minitest.clock_time
      new_impl.call(x)
      t = Minitest.clock_time - t0
      times_b << t
    end

    increase(average(times_a), average(times_b)).round(1)
  end

  # run(hash)
  # the key is the name of the method
  # value :old    will be a proc to execute original method behavior
  # value :new    will be a proc to execute newer method behavior
  def run(hsh)
    io.send :puts, "Pinch-bench (Pbench) by Daniel P. Clark"
    io.send :puts, "-"*80
    io.send :puts, os_lang_specs
    io.send :puts, "-"*80
    hsh.keys.each do |k|
      h = hsh[k]
      result = performance(h[:old], h[:new])
      io.send :puts, "Performance change for #{k} is %.1f%" % result
    end
  end

  def os_lang_specs
    "#{FasterPath.ruby_arch_bits}-bit #{`ruby -v`.chomp}\n" +
      "#{FasterPath.rust_arch_bits}-bit #{`rustc -V`.chomp}"
  end

  def increase(old_num, new_num)
    (old_num-new_num)*100/old_num
  end

  def average(t)
    t.map(&:to_f).inject(:+)./(t.count)
  end
end
