require "benchmark_helper"

class RelativeBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_relative?
    benchmark :rust do
      FasterPath.relative?("/hello")
      FasterPath.relative?("goodbye")
    end
  end

  def bench_ruby_relative?
    benchmark :ruby do
      Pathname.new("/hello").relative?
      Pathname.new("goodbye").relative?
    end
  end
end
