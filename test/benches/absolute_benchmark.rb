require "benchmark_helper"

class AbsoluteBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
    @one = "/hello"
    @two = "goodbye"
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_absolute?
    benchmark :rust do
      FasterPath.absolute?(@one)
      FasterPath.absolute?(@two)
    end
  end

  def bench_ruby_absolute?
    one = Pathname.new(@one)
    two = Pathname.new(@two)
    benchmark :ruby do
      one.absolute?
      two.absolute?
    end
  end
end
