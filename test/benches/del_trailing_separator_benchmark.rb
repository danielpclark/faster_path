require 'benchmark_helper'

class DelTrailingSeparatorBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_del_trailing_separator
    benchmark :rust do
      FasterPath.del_trailing_separator('/hello/world')
      FasterPath.del_trailing_separator('/hello/world/')
    end
  end

  def bench_ruby_del_trailing_separator
    benchmark :ruby do
      Pathname.allocate.send(:del_trailing_separator, '/hello/world')
      Pathname.allocate.send(:del_trailing_separator, '/hello/world/')
    end
  end

end
