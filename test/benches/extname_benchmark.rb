require "benchmark_helper"

class ExtnameBenchmark < BenchmarkHelper
  def teardown
    super __FILE__
  end
  
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
      TIMER[__FILE__].rust.mark
      n.times do
        CASES.each { |path| FasterPath.extname(path) }
      end
    end
  end

  def bench_ruby_extname
    assert_performance_constant do |n|
      TIMER[__FILE__].ruby.mark
      n.times do
        CASES.each { |path| File.extname(path) }
      end
    end
  end
end
