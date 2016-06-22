require "test_helper"
require "minitest/benchmark"

class FasterPathBenchmark < Minitest::Benchmark
  def bench_rust_extname
    assert_performance_constant do |n|
      100000.times do
        FasterPath.extname('verylongfilename_verylongfilename.rb')
        FasterPath.extname('/very/long/path/name/very/long/path/name/very/long/path/name/file.rb')
        FasterPath.extname('/ext/mail.rb')
      end
    end
  end

  def bench_ruby_extname
    assert_performance_constant do |n|
      100000.times do
        File.extname('verylongfilename_verylongfilename.rb')
        File.extname('/very/long/path/name/very/long/path/name/very/long/path/name/file.rb')
        File.extname('/ext/main.rb')
      end
    end
  end
end
