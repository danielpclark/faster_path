require "benchmark_helper"

class RelativeBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end
  
  def bench_rust_relative?
    benchmark_graph __FILE__, :rust do
      FasterPath.relative?("/hello")
      FasterPath.relative?("goodbye")
    end
  end

  def bench_ruby_relative?
    benchmark_graph __FILE__, :ruby do
      Pathname.new("/hello").relative?
      Pathname.new("goodbye").relative?
    end
  end
end
