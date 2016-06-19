require 'test_helper'
require 'minitest/benchmark'

class FasterPathBenchmark < Minitest::Benchmark
  def bench_rust_has_trailing_separator
    assert_performance_constant do |n|
      100_000.times do
        FasterPath.has_trailing_separator? '////a//aaa/a//a/aaa////'
        FasterPath.has_trailing_separator? 'hello/'
      end
    end
  end

  def bench_ruby_has_trailing_separator
    assert_performance_constant do |n|
      100_000.times do
        Pathname.allocate.send :has_trailing_separator?, '////a//aaa/a//a/aaa////'
        Pathname.allocate.send :has_trailing_separator?, 'hello/'
      end
    end
  end
end
