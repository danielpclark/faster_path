require "benchmark_helper"

class ChopBasenameBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_chop_basename
    benchmark :rust do
      FasterPath.chop_basename "/hello/world.txt"
      FasterPath.chop_basename "world.txt"
      FasterPath.chop_basename ""
    end
  end

  def bench_ruby_chop_basename
    benchmark :ruby do
      Pathname.new("").send :chop_basename, "/hello/world.txt"
      Pathname.new("").send :chop_basename, "world.txt"
      Pathname.new("").send :chop_basename, ""
    end
  end
end
