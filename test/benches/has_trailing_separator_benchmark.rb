require 'benchmark_helper'

class HasTrailingSeparatorBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_has_trailing_separator
    benchmark :rust do
      FasterPath.has_trailing_separator? '////a//aaa/a//a/aaa////'
      FasterPath.has_trailing_separator? 'hello/'
    end
  end

  def bench_ruby_has_trailing_separator
    benchmark :ruby do
      Pathname.allocate.send :has_trailing_separator?, '////a//aaa/a//a/aaa////'
      Pathname.allocate.send :has_trailing_separator?, 'hello/'
    end
  end
end
