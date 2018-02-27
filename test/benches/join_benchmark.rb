require "benchmark_helper"

class JoinBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_join
    benchmark :rust do
      FasterPath.join('a', 'b')
      FasterPath.join('.', 'b')
      FasterPath.join('a//b/c', '../d//e')
    end
  end

  def bench_ruby_join
    benchmark :ruby do
      Pathname.allocate.send(:join, 'a', 'b')
      Pathname.allocate.send(:join, '.', 'b')
      Pathname.allocate.send(:join, 'a//b/c', '../d//e')
    end
  end
end
