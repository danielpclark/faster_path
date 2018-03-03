require "benchmark_helper"

class RelativePathFromBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
    @one = "/a/b/c/d"
    @two = "/a/b"
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_relative_path_from
    benchmark :rust do
      FasterPath.relative_path_from "/a/b/c/d", "/a/b"
      FasterPath.relative_path_from "/a/b", "/a/b/c/d"
    end
  end

  def bench_ruby_relative_path_from
    one = Pathname.new(@one)
    two = Pathname.new(@two)
    benchmark :ruby do
      one.relative_path_from two
      two.relative_path_from one
    end
  end
end
