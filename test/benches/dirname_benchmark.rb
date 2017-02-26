require "benchmark_helper"

class DirnameBenchmark < BenchmarkHelper

  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_ruby_dirname
    benchmark_graph :rust do
      File.dirname "/really/long/path/name/which/ruby/doesnt/like/bar.txt"
      File.dirname "/foo/"
      File.dirname "."
    end
  end

  def bench_rust_dirname
    benchmark_graph :ruby do
      FasterPath.dirname "/really/long/path/name/which/ruby/doesnt/like/bar.txt"
      FasterPath.dirname "/foo/"
      FasterPath.dirname "."
    end
  end
end
