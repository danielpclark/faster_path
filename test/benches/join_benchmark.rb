require "benchmark_helper"

class JoinBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
    @pathname = Pathname.new(".").freeze
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_join
    benchmark :rust do
      FasterPath.join('.', 'b')
      FasterPath.join('.', '../d//e')
    end
  end

  def bench_ruby_join
    benchmark :ruby do
      @pathname.send(:join, 'b')
      @pathname.send(:join, '../d//e')
    end
  end
end
