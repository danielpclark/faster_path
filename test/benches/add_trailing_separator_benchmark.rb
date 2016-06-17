require 'test_helper'
require 'minitest/benchmark'

class FasterPathBenchmark < Minitest::Benchmark
  def bench_rust_add_trailing_separator
    assert_performance_constant do |n|
      100_000.times do
        FasterPath.add_trailing_separator('/hello/world')
        FasterPath.add_trailing_separator('/hello/world/')
      end
    end
  end

  def bench_ruby_add_trailing_separator
    assert_performance_constant do |n|
      100_000.times do
        Pathname.allocate.send(:add_trailing_separator, '/hello/world')
        Pathname.allocate.send(:add_trailing_separator, '/hello/world/')
      end
    end
  end
end

