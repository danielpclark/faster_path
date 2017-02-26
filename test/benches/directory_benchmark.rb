require "benchmark_helper"

class DirectoryBenchmark < BenchmarkHelper

  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_directory?
    benchmark_graph :rust do
      FasterPath.directory?("/hello")
      FasterPath.directory?("goodbye")
    end
  end

  def bench_ruby_directory?
    benchmark_graph :ruby do
      Pathname.new("/hello").directory?
      Pathname.new("goodbye").directory?
    end
  end
end
