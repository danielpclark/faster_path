require "test_helper"
require "minitest/benchmark"
require 'fileutils'
require 'stop_watch'
require 'gruff'

class BenchmarkHelper < Minitest::Benchmark
  def self.bench_range
    [20_000, 40_000, 60_000, 80_000, 100_000]
  end

  def graph_benchmarks
    if rust.time? && ruby.time?
      g = Gruff::Line.new
      g.title = graph_title
      g.labels = generate_benchmark_range_labels

      g.data(:ruby, graph_times(:ruby))
      g.data(:rust, graph_times(:rust))

      g.write( output_file )
    end
  end

  def benchmark lang
    assert_performance_constant do |n|
      send(lang).mark
      n.times do
        yield
      end
    end
    send(lang).mark
  end
  private :benchmark

  def test_name
    File.basename(@file, '.rb')
  end
  private :test_name

  def graph_title
    test_name.split('_').map(&:capitalize).join(' ')
  end
  private :graph_title

  def output_file
    path = File.join(File.expand_path('..', __dir__), 'doc', 'graph')

    FileUtils.mkdir_p path

    File.join path, "#{test_name}.png"
  end
  private :output_file

  def ranges_for_benchmarks
    instance_exec do
      self.class.bench_range if defined?(self.class.bench_range)
    end || BenchmarkHelper.bench_range
  end
  private :ranges_for_benchmarks

  def generate_benchmark_range_labels
    ranges_for_benchmarks.
      each_with_object({}).
      with_index do |(val, hash), idx|
        hash[ idx.succ ] = commafy val
      end.merge({0 => 0})
  end
  private :generate_benchmark_range_labels

  Languages = Struct.new(:ruby, :rust) do
    def initialize
      super(StopWatch::Timer.new, StopWatch::Timer.new)
    end
  end

  TIMERS = Hash.new.
    tap do |t|
      t.default_proc = \
        ->(hash, key){ hash[key] = Languages.new }
    end

  def timers
    TIMERS[@file]
  end
  private :timers

  def ruby
    timers.ruby
  end
  private :ruby

  def rust
    timers.rust
  end
  private :rust

  def graph_times lang
    send(lang).times.unshift(0)
  end
  private :graph_times

  def commafy num
    num.to_s.chars.reverse.
      each_with_object("").
      with_index do |(val, str), idx|
        str.prepend((idx%3).zero? ? val + ',' : val)
      end.chop
  end
end
