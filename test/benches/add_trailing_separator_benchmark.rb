require 'benchmark_helper'

class AddTrailingSeparatorBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_add_trailing_separator
    benchmark :rust do
      FasterPath.add_trailing_separator('/hello/world')
      FasterPath.add_trailing_separator('/hello/world/')
    end
  end

  def bench_ruby_add_trailing_separator
    benchmark :ruby do
      Pathname.allocate.send(:add_trailing_separator, '/hello/world')
      Pathname.allocate.send(:add_trailing_separator, '/hello/world/')
    end
  end

end
