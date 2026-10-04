ENV['BUNDLE_GEMFILE'] = File.expand_path('../Gemfile', File.dirname(__FILE__))
# Set up the bundle before loading faster_path, which needs rake (through
# thermite): otherwise the newest installed rake gets activated and conflicts
# with the one in Gemfile.lock when mspec is run outside `bundle exec`.
require 'bundler/setup'
$LOAD_PATH.unshift File.expand_path('../lib', File.dirname(__FILE__))
require 'faster_path'
require 'faster_path/optional/monkeypatches'
FasterPath.sledgehammer_everything! if ENV['TEST_MONKEYPATCHES'].to_s['true']
