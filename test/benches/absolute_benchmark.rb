require "benchmark_helper"

class AbsoluteBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end

  def bench_rust_absolute?
    assert_performance_constant do |n|
      TIMER[__FILE__].rust.mark
      n.times do
        FasterPath.absolute?("/hello")
        FasterPath.absolute?("goodbye")
      end
    end
    TIMER[__FILE__].rust.mark
  end

  def bench_ruby_absolute?
    assert_performance_constant do |n|
      TIMER[__FILE__].ruby.mark
      n.times do
        Pathname.new("/hello").absolute?
        Pathname.new("goodbye").absolute?
      end
    end
    TIMER[__FILE__].ruby.mark
  end
end
