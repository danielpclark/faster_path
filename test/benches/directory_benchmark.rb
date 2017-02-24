require "benchmark_helper"

class DirectoryBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end
  
  def bench_rust_directory?
    assert_performance_constant do |n|
      TIMER[__FILE__].rust.mark
      n.times do
        FasterPath.directory?("/hello")
        FasterPath.directory?("goodbye")
      end
    end
    TIMER[__FILE__].rust.mark
  end

  def bench_ruby_directory?
    assert_performance_constant do |n|
      TIMER[__FILE__].ruby.mark
      n.times do
        Pathname.new("/hello").directory?
        Pathname.new("goodbye").directory?
      end
    end
    TIMER[__FILE__].ruby.mark
  end
end
