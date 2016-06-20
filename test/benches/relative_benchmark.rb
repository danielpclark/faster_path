require 'test_helper'
require 'minitest/benchmark'

class FasterPathBenchmark < Minitest::Benchmark
  def bench_rust_relative?
    assert_performance_constant do |_n|
      10_000.times do
        FasterPath.relative?('/hello')
        FasterPath.relative?('goodbye')
      end
    end
  end

  def bench_ruby_relative?
    assert_performance_constant do |_n|
      10_000.times do
        Pathname.new('/hello').relative?
        Pathname.new('goodbye').relative?
      end
    end
  end
end
