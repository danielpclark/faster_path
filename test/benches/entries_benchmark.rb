require "test_helper"
require "minitest/benchmark"
require "pathname"

class FasterPathBenchmark < Minitest::Benchmark
  def bench_rust_entries
    assert_performance_constant do |n|
      10000.times do
        FasterPath.entries(".")
        FasterPath.entries("src")
      end
    end
  end

  def bench_ruby_entries
    assert_performance_constant do |n|
      10000.times do
        Pathname.new(".").entries
        Pathname.new("src").entries
      end
    end
  end
end
