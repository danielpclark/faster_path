require 'benchmark_helper'

class HasTrailingSeparatorBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end
  
  def bench_rust_has_trailing_separator
    assert_performance_constant do |n|
      TIMER[__FILE__].rust.mark
      n.times do
        FasterPath.has_trailing_separator? '////a//aaa/a//a/aaa////'
        FasterPath.has_trailing_separator? 'hello/'
      end
    end
    TIMER[__FILE__].rust.mark
  end

  def bench_ruby_has_trailing_separator
    assert_performance_constant do |n|
      TIMER[__FILE__].ruby.mark
      n.times do
        Pathname.allocate.send :has_trailing_separator?, '////a//aaa/a//a/aaa////'
        Pathname.allocate.send :has_trailing_separator?, 'hello/'
      end
    end
    TIMER[__FILE__].ruby.mark
  end
end
