$LOAD_PATH.unshift File.expand_path('../../lib', __FILE__)
if ENV['TEST_MONKEYPATCHES'] &&
    ENV['WITH_REGRESSION'] &&
    !ENV['RUST_BACKTRACE'] &&
    ENV['TRAVIS_OS_NAME'] == 'linux' &&
    Gem::Dependency.new('', '~> 2.3', '< 2.4').match?('', RUBY_VERSION)
  require 'simplecov'
  require 'coveralls'

  SimpleCov.formatter = SimpleCov::Formatter::MultiFormatter[
    SimpleCov::Formatter::HTMLFormatter,
    Coveralls::SimpleCov::Formatter
  ]
  SimpleCov.start do
    add_filter '/test/'
    add_filter '/spec/'
  end
end
require 'faster_path'
require 'minitest/autorun'
require 'pathname'
require 'minitest/reporters'
require 'color_pound_spec_reporter'

Minitest::Reporters.use! [ColorPoundSpecReporter.new]

::Minitest::Assertions.module_eval do
  def assert_incompatible_encoding
    d = "\u{3042}\u{3044}".encode("utf-16le")
    assert_raises(Encoding::CompatibilityError) {yield d}
    m = Class.new {define_method(:to_path) {d}}
    assert_raises(Encoding::CompatibilityError) {yield m.new}
  end
end

::Minitest::Test.class_eval do
  DRIVE = Dir.pwd[%r{\A(?:[a-z]:|//[^/]+/[^/]+)}i]
  POSIX = /cygwin|mswin|bccwin|mingw|emx/ !~ RUBY_PLATFORM
  NTFS = !(/mingw|mswin|bccwin/ !~ RUBY_PLATFORM)
  DOSISH = !File::ALT_SEPARATOR.nil?
  DOSISH_DRIVE_LETTER = File.dirname("A:") == "A:."
  DOSISH_UNC = File.dirname("//") == "//"

  def regular_file
    return @file if defined? @file
    @file = make_tmp_filename("file")
    make_file("foo", @file)
    @file
  end

  def make_tmp_filename(prefix)
    "#{@dir}/#{prefix}.test"
  end

  def make_file(content, file)
    open(file, "w") {|fh| fh << content }
  end

  def utf8_file
    return @utf8file if defined? @utf8file
    @utf8file = make_tmp_filename("\u3066\u3059\u3068")
    make_file("foo", @utf8file)
    @utf8file
  end
end
