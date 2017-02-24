require 'benchmark_helper'

class AddTrailingSeparatorBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end

  def bench_rust_add_trailing_separator
    benchmark_graph __FILE__, :rust do
      FasterPath.add_trailing_separator('/hello/world')
      FasterPath.add_trailing_separator('/hello/world/')
    end
  end

  def bench_ruby_add_trailing_separator
    benchmark_graph __FILE__, :ruby do
      Pathname.allocate.send(:add_trailing_separator, '/hello/world')
      Pathname.allocate.send(:add_trailing_separator, '/hello/world/')
    end
  end
end

