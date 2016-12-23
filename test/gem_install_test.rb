require 'test_helper'
#require 'fileutils' 
require 'rubygems'
#require 'rubygems/package'
require 'rubygems/installer_test_case' # adds writer method for Gem::Installer & includes rubygems/test_case

class GemInstallTest < Gem::TestCase
  def setup
    super

    gemspec = 'faster_path.gemspec'
    @spec = eval IO.read( File.join(File.expand_path(Pathname(__FILE__).parent.parent), gemspec) )
    @gem = @spec.cache_file
    @installer = Gem::Installer.at(@gem, install_dir: @gemhome, user_install: false)

    Gem::Installer.path_warning = false

    common_installer_setup

    @config = Gem.configuration
  end

  def teardown
    common_installer_teardown

    super

    Gem.configuration = @config
  end

  def test_compiles_the_extension
    skip 'WIP'

    #    puts "Testing directory results"
    #    _(Dir.entries(tempdir)).must_include 'target'
    #    _(Dir.entries(tempdir + 'target')).must_include 'release'
    #    _(Dir.entries(tempdir + 'target/release').keep_if {|i| i['faster_path']}).must_be :one?
    #  end
    #end
  end
end
