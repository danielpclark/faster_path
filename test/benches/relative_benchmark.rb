require "benchmark_helper"

class RelativeBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end
  
  def bench_rust_relative?
    assert_performance_constant do |n|
      TIMER[__FILE__].rust.mark
      n.times do
        FasterPath.relative?("/hello")
        FasterPath.relative?("goodbye")
      end
    end
  end

  def bench_ruby_relative?
    assert_performance_constant do |n|
      TIMER[__FILE__].ruby.mark
      n.times do
        Pathname.new("/hello").relative?
        Pathname.new("goodbye").relative?
      end
    end
  end
end
