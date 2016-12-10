require 'test_helper'
require 'bundler/cli'
require 'fileutils' 
require 'rubygems'
require 'rubygems/package'

# Gem::Installer.at
# - https://github.com/rubygems/rubygems/blob/master/lib/rubygems/installer_test_case.rb#L192-L196
# - https://github.com/rubygems/rubygems/blob/master/lib/rubygems/installer.rb#L101-L105
#
# Construct an installer object for an ephemeral gem (no .gem, only .gemspec)
# - https://github.com/rubygems/rubygems/blob/master/lib/rubygems/installer.rb#L133-L136
#
# Create .gem from .gemspec
# - https://github.com/rubygems/rubygems/blob/master/lib/rubygems/package.rb
#
# Gem.instance_variable_get :@default_dir
# # => "/home/danielpclark/.rvm/rubies/ruby-2.3.1/lib/ruby/gems/2.3.0"
# Gem.instance_variable_get :@ruby_api_version
# # => "2.3.0"

describe 'Bundler Test' do
  setup do
    @gemspec = eval IO.read('faster_path.gemspec')
  end

  after do
    Dir.chdir(Pathname('__FILE__').parent.parent)
    system('bundle --system') # Revert from previous path used
  end

  it 'Compiles the extension' do
    skip 'WIP'
    stars = "*" * 30
    puts
    puts stars
    puts "Cleaning Bundler env with block"
    Bundler.with_clean_env do
      #system('gem build faster_path.gemspec')
      faster_path_gem = Pathname(__FILE__).parent.parent
      puts stars
      puts "The faster_path_gem path is #{faster_path_gem}"

      Dir.mktmpdir('build_test') do |dir|
        _(Dir).must_be :exist?, dir
        puts stars
        puts "Made temporary directory #{dir}"
        Dir.chdir(dir) do
          puts "Changed into that directory"
          File.write(dir + '/Gemfile', <<-GEMFILE)
            source 'https://rubygems.org'
            gem 'faster_path', path: '#{faster_path_gem}'
          GEMFILE
          _(File).must_be :exist?, dir + '/Gemfile'
          puts stars
          puts "Made Gemfile with the following content"
          puts File.read(dir + '/Gemfile')
          vendor = Pathname(dir) + 'vendor/bundle'
          puts stars
          puts "Assigning a vendor path as #{vendor}"
          FileUtils::mkdir_p vendor
          puts "Making that directory"
          puts "Starting bundle..."
          # Bundler.clean_system()
          system(#{'GIT_DIR' => faster_path_gem.+('../.git').to_s},
                 {
                   'BUNDLE_GEMFILE' => dir + '/Gemfile',
                   'BUNDLE_PATH' => vendor.to_s,
                   'GEM_HOME' => vendor.to_s
                 },
                 Gem.bin_path('bundler', 'bundle'), 'install',
                 #'--gemfile', faster_path_gem.+('../faster_path.gemspec').to_s,
                 '--path', vendor.to_s,
                 '--standalone',
                 '--verbose'
                )
          puts stars
          puts "Advanced path env with bundle command instantiated to bundle Gemfile specs to temporary directory."
          
          puts stars
          puts "Testing directory results"
          _(Dir.entries(vendor)).must_include 'target'
          _(Dir.entries(vendor + 'target')).must_include 'release'
          _(Dir.entries(vendor + 'target/release').keep_if {|i| i['faster_path']}).must_be :one?
        end
      end
    end
  end
end
