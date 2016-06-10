require "test_helper"
require "minitest/benchmark"

class FasterPathBenchmark < Minitest::Benchmark
  def bench_rust_blank?
    assert_performance_constant do |n|
      100.times do
        FasterPath.blank? "world.txt"
        FasterPath.blank? "  "
        FasterPath.blank? ""
      end
    end
  end
end
