require 'test_helper'

# TODO: see
# https://github.com/rubygems/rubygems/blob/master/test/rubygems/test_gem_commands_install_command.rb
# https://github.com/rubygems/rubygems/blob/master/test/rubygems/test_gem.rb
# for helpful ideas

describe 'Gem Test' do
  it 'Compiles the extension' do
    skip 'Doesn\'t work.  WIP'
    faster_path_gem = Pathname(__FILE__).expand_path + '..'

    Dir.mktmpdir('build_test') do |dir|
      Dir.chdir(dir) do
        #File.write('Gemfile', <<-GEMFILE)
        #  source 'https://rubygems.org'
        #  gem 'faster_path', path: '#{faster_path_gem}'
        #GEMFILE
        #FileUtils::mkdir_p 'vendor/bundle'
        system(Gem.bin_path('rubygems', 'gem'), 'install', '--local', faster_path_gem, '--install-dir', 'vendor/bundle')
        
        _(Dir.entries('vendor/bundle')).must_include 'target'
        _(Dir.entries('vendor/bundle/target')).must_include 'release'
        _(Dir.entries('vendor/bundle/target/release').keep_if {|i| i['faster_path']}).must_be :one?
      end
    end
  end
end
