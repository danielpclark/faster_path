require 'test_helper'
require 'minitest/benchmark'

class FasterPathBenchmark < Minitest::Benchmark
  def bench_rust_directory?
    assert_performance_constant do |_n|
      10_000.times do
        FasterPath.directory?('/hello')
        FasterPath.directory?('goodbye')
      end
    end
  end

  def bench_ruby_directory?
    assert_performance_constant do |_n|
      10_000.times do
        Pathname.new('/hello').directory?
        Pathname.new('goodbye').directory?
      end
    end
  end
end
