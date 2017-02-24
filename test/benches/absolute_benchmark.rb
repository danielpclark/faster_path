require "benchmark_helper"

class AbsoluteBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end

  def bench_rust_absolute?
    benchmark_graph __FILE__, :rust do
      FasterPath.absolute?("/hello")
      FasterPath.absolute?("goodbye")
    end
  end

  def bench_ruby_absolute?
    benchmark_graph __FILE__, :ruby do
      Pathname.new("/hello").absolute?
      Pathname.new("goodbye").absolute?
    end
  end
end
