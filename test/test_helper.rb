$LOAD_PATH.unshift File.expand_path('../../lib', __FILE__)
require 'faster_path'
require 'minitest/autorun'

if ENV['TEST_REFINEMENTS'].to_s["true"]
  require "faster_path/optional/refinements"
  require "faster_path/optional/monkeypatches"
end
