require "benchmark_helper"

class PlusBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_plus
    benchmark :rust do
      FasterPath.plus('a', 'b')
      FasterPath.plus('.', 'b')
      FasterPath.plus('a//b/c', '../d//e')
    end
  end

  def bench_ruby_plus
    benchmark :ruby do
      Pathname.allocate.send(:plus, 'a', 'b')
      Pathname.allocate.send(:plus, '.', 'b')
      Pathname.allocate.send(:plus, 'a//b/c', '../d//e')
    end
  end
end
