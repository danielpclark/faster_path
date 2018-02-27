require "benchmark_helper"

class CleanpathConservativeBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
    @one = 'a/../.'
    @two = 'a/b/../../../../c/../d'
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_cleanpath_conservative
    benchmark :rust do
      FasterPath.cleanpath_conservative(@one)
      FasterPath.cleanpath_conservative(@two)
    end
  end

  def bench_ruby_cleanpath_conservative
    one = Pathname.new(@one)
    two = Pathname.new(@two)
    benchmark :ruby do
      one.send :cleanpath_conservative
      two.send :cleanpath_conservative
    end
  end
end
