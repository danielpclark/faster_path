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
    benchmark_graph __FILE__, :rust do
      CASES.each { |path| FasterPath.extname(path) }
    end
  end

  def bench_ruby_extname
    benchmark_graph __FILE__, :ruby do
      CASES.each { |path| File.extname(path) }
    end
  end
end
