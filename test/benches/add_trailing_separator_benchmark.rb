require 'benchmark_helper'

class AddTrailingSeparatorBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end

  def bench_rust_add_trailing_separator
    assert_performance_constant do |n|
      TIMER[__FILE__].rust.mark
      n.times do
        FasterPath.add_trailing_separator('/hello/world')
        FasterPath.add_trailing_separator('/hello/world/')
      end
    end
  end

  def bench_ruby_add_trailing_separator
    assert_performance_constant do |n|
      TIMER[__FILE__].ruby.mark
      n.times do
        Pathname.allocate.send(:add_trailing_separator, '/hello/world')
        Pathname.allocate.send(:add_trailing_separator, '/hello/world/')
      end
    end
  end
end

