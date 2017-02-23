ENV['BUNDLE_GEMFILE'] = File.expand_path('../Gemfile', File.dirname(__FILE__))
$LOAD_PATH.unshift File.expand_path('../lib', File.dirname(__FILE__))
require 'faster_path'
require 'faster_path/optional/monkeypatches'
FasterPath.sledgehammer_everything! if ENV['TEST_MONKEYPATCHES'].to_s['true']
