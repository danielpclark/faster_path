require "benchmark_helper"

class DirnameBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end
  
  def bench_ruby_dirname
    assert_performance_constant do |n|
      TIMER[__FILE__].rust.mark
      n.times do
        File.dirname "/really/long/path/name/which/ruby/doesnt/like/bar.txt"
        File.dirname "/foo/"
        File.dirname "."  
      end
    end
  end

  def bench_rust_dirname
    assert_performance_constant do |n|
      TIMER[__FILE__].ruby.mark
      n.times do
        FasterPath.dirname "/really/long/path/name/which/ruby/doesnt/like/bar.txt"
        FasterPath.dirname "/foo/"
        FasterPath.dirname "."
      end
    end
  end
end
