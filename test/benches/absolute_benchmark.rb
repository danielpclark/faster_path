require "benchmark_helper"

class AbsoluteBenchmark < BenchmarkHelper

  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_absolute?
    benchmark_graph :rust do
      FasterPath.absolute?("/hello")
      FasterPath.absolute?("goodbye")
    end
  end

  def bench_ruby_absolute?
    benchmark_graph :ruby do
      Pathname.new("/hello").absolute?
      Pathname.new("goodbye").absolute?
    end
  end

end
