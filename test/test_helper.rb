$LOAD_PATH.unshift File.expand_path('../../lib', __FILE__)
require 'faster_path'
require 'minitest/autorun'
require 'pathname'
require 'minitest/reporters'
require 'color_pound_spec_reporter'

Minitest::Reporters.use! [ColorPoundSpecReporter.new]
