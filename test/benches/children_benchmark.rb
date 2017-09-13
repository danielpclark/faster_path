require "benchmark_helper"

class ChildrenBenchmark < BenchmarkHelper
  def self.bench_range
    [20, 40, 60, 80, 100]
  end

  def setup
    @file ||= __FILE__
    @one = "."
    @two = "../"
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_children
    benchmark :rust do
      FasterPath.children(@one)
      FasterPath.children(@two, false)
    end
  end

  def bench_ruby_children
    one = Pathname.new(@one)
    two = Pathname.new(@two)
    benchmark :ruby do
      one.children
      two.children(false)
    end
  end
end
