require "test_helper"
require "minitest/benchmark"
require "pathname"

class FasterPathBenchmark < Minitest::Benchmark

  def bench_ruby_dirname
    assert_performance_constant do |n|
      10000.times do
        File.dirname "/really/long/path/name/which/ruby/doesnt/like/bar.txt"
        File.dirname "/foo/"
        File.dirname "."  
      end
    end
  end

  def bench_rust_dirname
    assert_performance_constant do |n|
      10000.times do
        FasterPath.dirname "/really/long/path/name/which/ruby/doesnt/like/bar.txt"
        FasterPath.dirname "/foo/"
        FasterPath.dirname "."
      end
    end
  end
end
