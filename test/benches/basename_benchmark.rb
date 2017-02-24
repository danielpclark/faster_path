require "benchmark_helper"

class BasenameBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end

  def bench_rust_basename
    assert_performance_constant do |n|
      TIMER[__FILE__].rust.mark
      n.times do
        FasterPath.basename("/hello/world")
        FasterPath.basename('/home/gumby/work/ruby.rb', '.rb')
        FasterPath.basename('/home/gumby/work/ruby.rb', '.*')
      end
    end
    TIMER[__FILE__].rust.mark
  end

  def bench_ruby_basename
    assert_performance_constant do |n|
      TIMER[__FILE__].ruby.mark
      n.times do
        File.basename("/hello/world")
        File.basename('/home/gumby/work/ruby.rb', '.rb')
        File.basename('/home/gumby/work/ruby.rb', '.*')
      end
    end
    TIMER[__FILE__].ruby.mark
  end
end
