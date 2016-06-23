require "test_helper"
require "minitest/benchmark"

class FasterPathBenchmark < Minitest::Benchmark
  CASES = %w(
    verylongfilename_verylongfilename_verylongfilename_verylongfilename.rb
    /very/long/path/name/very/long/path/name/very/long/path/name/file.rb
    /ext/mail.rb
    lots/of/trailing/slashes.rb/////////////////////
    .hiddenfile
    very_long_extension_verylongextensionverylongextensionverylongextensionverylongextension.rb
  ) + ['']

  def bench_rust_extname
    assert_performance_constant do |n|
      100000.times do
        CASES.each { |path| FasterPath.extname(path) }
      end
    end
  end

  def bench_ruby_extname
    assert_performance_constant do |n|
      100000.times do
        CASES.each { |path| File.extname(path) }
      end
    end
  end
end
