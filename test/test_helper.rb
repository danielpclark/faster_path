$LOAD_PATH.unshift File.expand_path('../../lib', __FILE__)
if ENV['TEST_MONKEYPATCHES'] &&
    ENV['WITH_REGRESSION'] &&
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
