require 'test_helper'
require 'rubygems'
##
# adds writer method for Gem::Installer
# and includes rubygems/test_case
require 'rubygems/installer_test_case' 


##
# TODO: Look into using RbConfig to detect and validate built extension libraries across platforms.


class GemInstallTest < Gem::TestCase
  def setup
    @root = File.absolute_path(Pathname(__FILE__).parent.parent)
    super

    #set_fetcher

    gemspec = 'faster_path.gemspec'
    @spec = eval IO.read( File.join(@root, gemspec) )
    @gem = @spec.cache_file
    @spec.files = @spec.files.map {|f| File.join @root, f }
    @spec.extensions = @spec.extensions.map {|f| File.join @root, f }
    @installer = Gem::Installer.at(@gem, install_dir: @gemhome, user_install: false)

    Gem::Installer.path_warning = false

    common_installer_setup

    @config = Gem.configuration
  end

  def teardown
    common_installer_teardown

    super

    Gem.configuration = @config
    
    Dir.chdir(@root)
    system('bundle --system') # Revert from previous path used
  end

  def test_extract_files
    #require 'logger'
    #FileUtils.mkdir_p File.join(@root, 'log')
    #@logger = Logger.new(File.join @root, 'log', 'gem_build.log')

    set_fetcher
    require 'rubygems/dependency_installer'
    inst = Gem::DependencyInstaller.new
    request_set = inst.resolve_dependencies @spec.name, @spec.version
    request_set.install({})
    ##inst.errors
    
    #@installer.install
    #@logger.info `tree #{@gemhome}`
    #@logger.close
    ##@installer.extract_files

    #assert_path_exists File.join @spec.gem_dir, 'bin/executable'
    #
  end

  private
  def set_fetcher
    @fetcher = Gem::RemoteFetcher.new(Gem.configuration[:http_proxy])
    Gem::RemoteFetcher.fetcher = @fetcher

    @gem_repo = 'https://rubygems.org'
    @uri = URI.parse @gem_repo
    Gem.sources.replace [@gem_repo]

    sources = Gem::SourceList.from(Gem.sources)
    #Gem.searcher = nil
    Gem::SpecFetcher.fetcher = Gem::SpecFetcher.new sources
  end
end

##
# These directories need to be built
# puts "Testing directory results"
# _(Dir.entries(tempdir)).must_include 'target'
# _(Dir.entries(tempdir + 'target')).must_include 'release'
# _(Dir.entries(tempdir + 'target/release').keep_if {|i| i['faster_path']}).must_be :one?

