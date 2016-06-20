require 'test_helper'
require 'minitest/benchmark'

class FasterPathBenchmark < Minitest::Benchmark
  def bench_rust_absolute?
    assert_performance_constant do |_n|
      10_000.times do
        FasterPath.absolute?('/hello')
        FasterPath.absolute?('goodbye')
      end
    end
  end

  def bench_ruby_absolute?
    assert_performance_constant do |_n|
      10_000.times do
        Pathname.new('/hello').absolute?
        Pathname.new('goodbye').absolute?
      end
    end
  end
end
