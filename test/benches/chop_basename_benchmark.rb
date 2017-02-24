require "benchmark_helper"

class ChopBasenameBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end
  
  def bench_rust_chop_basename
    benchmark_graph __FILE__, :rust do
      FasterPath.chop_basename "/hello/world.txt"
      FasterPath.chop_basename "world.txt"
      FasterPath.chop_basename ""
    end
  end

  def bench_ruby_chop_basename
    benchmark_graph __FILE__, :ruby do
      Pathname.new("").send :chop_basename, "/hello/world.txt"
      Pathname.new("").send :chop_basename, "world.txt"
      Pathname.new("").send :chop_basename, ""
    end
  end
end
