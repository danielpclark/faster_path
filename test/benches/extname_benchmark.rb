require "benchmark_helper"

class ExtnameBenchmark < BenchmarkHelper

  def setup
    @file ||= __FILE__
  end

  def teardown
    super
    graph_benchmarks
  end
  
  def cases
    %w(
      verylongfilename_verylongfilename_verylongfilename_verylongfilename.rb
      /very/long/path/name/very/long/path/name/very/long/path/name/file.rb
      /ext/mail.rb
      lots/of/trailing/slashes.rb/////////////////////
      .hiddenfile
      very_long_extension_verylongextensionverylongextensionverylongextensionverylongextension.rb
    ) + ['']
  end

  def bench_rust_extname
    benchmark_graph :rust do
      cases.each { |path| FasterPath.extname(path) }
    end
  end

  def bench_ruby_extname
    benchmark_graph :ruby do
      cases.each { |path| File.extname(path) }
    end
  end
end
