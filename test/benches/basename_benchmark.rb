require "test_helper"
require "minitest/benchmark"

class FasterPathBenchmark < Minitest::Benchmark
  def bench_rust_basename
    assert_performance_constant do |n|
      10000.times do
        FasterPath.basename("/hello/world")
        FasterPath.basename('/home/gumby/work/ruby.rb', '.rb')
        FasterPath.basename('/home/gumby/work/ruby.rb', '.*')
      end
    end
  end

  def bench_ruby_basename
    assert_performance_constant do |n|
      10000.times do
        File.basename("/hello/world")
        File.basename('/home/gumby/work/ruby.rb', '.rb')
        File.basename('/home/gumby/work/ruby.rb', '.*')
      end
    end
  end
end if FasterPath.respond_to? :basename
