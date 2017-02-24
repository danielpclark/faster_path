require "benchmark_helper"

class EntriesBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end

  def self.bench_range
    [2000, 4000, 6000, 8000, 10_000]
  end

  def bench_rust_entries
    assert_performance_constant do |n|
      TIMER[__FILE__].rust.mark
      n.times do
        FasterPath.entries(".")
        FasterPath.entries("src")
      end
    end
  end

  def bench_ruby_entries
    assert_performance_constant do |n|
      TIMER[__FILE__].ruby.mark
      n.times do
        Pathname.new(".").entries
        Pathname.new("src").entries
      end
    end
  end
end
