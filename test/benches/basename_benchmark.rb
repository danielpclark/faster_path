require "benchmark_helper"

class BasenameBenchmark < BenchmarkHelper
  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end

  def bench_rust_basename
    benchmark :rust do
      FasterPath.basename("/hello/world")
      FasterPath.basename('/home/gumby/work/ruby.rb', '.rb')
      FasterPath.basename('/home/gumby/work/ruby.rb', '.*')
      FasterPath.basename('ruby.rbx', 'rbx')
      FasterPath.basename('ruby.rbx', 'x')
      FasterPath.basename('ruby.rbx', '*')
    end
  end

  def bench_ruby_basename
    benchmark :ruby do
      File.basename("/hello/world")
      File.basename('/home/gumby/work/ruby.rb', '.rb')
      File.basename('/home/gumby/work/ruby.rb', '.*')
      File.basename('ruby.rbx', 'rbx')
      File.basename('ruby.rbx', 'x')
      File.basename('ruby.rbx', '*')
    end
  end
end
