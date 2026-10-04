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
    amplitude = ENV['LONG_RUN'].to_s[/\d+/].to_i
    amplitude = 1 if amplitude < 1
    amplitude = 30 if amplitude > 30
    [10_000 * amplitude] * 5
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
  # value :min    (optional) the least improvement, in percent, the method
  #               must keep over the original; see `regressions`
  #
  # Returns a hash of the method names to their measured improvement.
  def run(hsh)
    io.send :puts, "Pinch-bench (Pbench) by Daniel P. Clark"
    io.send :puts, "-"*80
    io.send :puts, os_lang_specs
    io.send :puts, "-"*80
    io.flush
    hsh.each_with_object({}) do |(k, h), results|
      result = performance(h[:old], h[:new])
      results[k] = result
      io.send :puts, "Performance change for #{k} is %.1f%%" % result
      io.flush
    end
  end

  # The benchmarks whose result fell below their :min floor, as messages.
  #
  # The floors are regression tests for performance: a method that has been
  # changed, or a dependency that has been upgraded, must not lose the
  # improvement over Ruby that this library exists for.
  def regressions(results, hsh)
    hsh.map do |k, h|
      next unless h[:min] && results[k] < h[:min]
      "%s is %.1f%% faster than Ruby, below its floor of %d%%" % [k, results[k], h[:min]]
    end.compact
  end

  def os_lang_specs
    os_stats = "#{FasterPath.ruby_arch_bits}-bit #{`ruby -v`.chomp}\n"
    rust, detail = `rustc -Vv`.split("\n", 2)
    os_stats += "#{FasterPath.rust_arch_bits}-bit #{rust}\n"
    os_stats + "architecture: #{detail.scan(/(?<=host: ).+(?=\n)/).first}"
  end

  def increase(old_num, new_num)
    (old_num-new_num)*100/old_num
  end

  def average(t)
    t.map(&:to_f).inject(:+)./(t.count)
  end
end
