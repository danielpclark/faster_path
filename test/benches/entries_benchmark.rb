require "benchmark_helper"

class EntriesBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end

  def self.bench_range
    [2000, 4000, 6000, 8000, 10_000]
  end

  def bench_rust_entries
    benchmark :rust do
      FasterPath.entries(".")
      FasterPath.entries("src")
    end
  end

  def bench_ruby_entries
    benchmark :ruby do
      Pathname.new(".").entries
      Pathname.new("src").entries
    end
  end
end
