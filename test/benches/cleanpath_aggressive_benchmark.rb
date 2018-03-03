require "benchmark_helper"

class CleanpathAggressiveBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
    @one = 'a/../.'
    @two = 'a/b/../../../../c/../d'
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_cleanpath_aggressive
    benchmark :rust do
      FasterPath.cleanpath_aggressive(@one)
      FasterPath.cleanpath_aggressive(@two)
    end
  end

  def bench_ruby_cleanpath_aggressive
    one = Pathname.new(@one)
    two = Pathname.new(@two)
    benchmark :ruby do
      one.send :cleanpath_aggressive
      two.send :cleanpath_aggressive
    end
  end
end
