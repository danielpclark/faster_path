require "benchmark_helper"

class DirnameBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end
  
  def bench_ruby_dirname
    benchmark_graph __FILE__, :rust do
      File.dirname "/really/long/path/name/which/ruby/doesnt/like/bar.txt"
      File.dirname "/foo/"
      File.dirname "."  
    end
  end

  def bench_rust_dirname
    benchmark_graph __FILE__, :ruby do
      FasterPath.dirname "/really/long/path/name/which/ruby/doesnt/like/bar.txt"
      FasterPath.dirname "/foo/"
      FasterPath.dirname "."
    end
  end
end
