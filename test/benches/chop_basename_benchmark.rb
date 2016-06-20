require 'test_helper'
require 'minitest/benchmark'

class FasterPathBenchmark < Minitest::Benchmark
  def bench_rust_chop_basename
    assert_performance_constant do |_n|
      100_000.times do
        FasterPath.chop_basename '/hello/world.txt'
        FasterPath.chop_basename 'world.txt'
        FasterPath.chop_basename ''
      end
    end
  end

  def bench_ruby_chop_basename
    assert_performance_constant do |_n|
      100_000.times do
        Pathname.new('').send :chop_basename, '/hello/world.txt'
        Pathname.new('').send :chop_basename, 'world.txt'
        Pathname.new('').send :chop_basename, ''
      end
    end
  end
end
