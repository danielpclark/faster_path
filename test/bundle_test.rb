require 'test_helper'
require 'bundler/cli'
require 'fileutils' 


describe 'Bundler Test' do
  after do
    Dir.chdir(Pathname('__FILE__').parent.parent)
    system('bundle --system') # Revert from previous path used
  end

  it 'Compiles the extension' do
    skip 'WIP'
#    Bundler.with_clean_env do
    faster_path_gem = Pathname(__FILE__).parent.parent

      Dir.mktmpdir('build_test') do |dir|
        Dir.chdir(dir) do
          File.write(dir + '/Gemfile', <<-GEMFILE)
            source 'https://rubygems.org'
            gem 'faster_path', path: '#{faster_path_gem}'
          GEMFILE
          vendor = Pathname(dir) + 'vendor/bundle'
          FileUtils::mkdir_p vendor
          system(#{'GIT_DIR' => faster_path_gem.+('../.git').to_s},
                 {
                   'BUNDLE_GEMFILE' => dir + '/Gemfile',
                   'BUNDLE_PATH' => vendor.to_s,
                   'GEM_HOME' => vendor.to_s
                 },
                 Gem.bin_path('bundler', 'bundle'), 'install',
                 #'--gemfile', faster_path_gem.+('../faster_path.gemspec').to_s,
                 '--path', vendor.to_s,
                 '--verbose'
                )
          
          _(Dir.entries(vendor)).must_include 'target'
          _(Dir.entries(vendor + 'target')).must_include 'release'
          _(Dir.entries(vendor + 'target/release').keep_if {|i| i['faster_path']}).must_be :one?
        end
      end
#    end
  end
end
