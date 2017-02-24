require "benchmark_helper"

class DirectoryBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end
  
  def bench_rust_directory?
    benchmark_graph __FILE__, :rust do
      FasterPath.directory?("/hello")
      FasterPath.directory?("goodbye")
    end
  end

  def bench_ruby_directory?
    benchmark_graph __FILE__, :ruby do
      Pathname.new("/hello").directory?
      Pathname.new("goodbye").directory?
    end
  end
end
