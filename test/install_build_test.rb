require 'test_helper.rb'
require 'rubygems/installer_test_case'
require 'rubygems/package'
require_relative 'support/bundler/helpers'
require_relative 'support/bundler/path'
include Spec::Helpers
include Spec::Path

class InstallBuildTest < Gem::InstallerTestCase
  ##
  # Gemfile name
  def faster_path; "faster_path-#{FasterPath::VERSION}.gem" end

  ## UNUSED
  # Current Ruby gem stash from Rubygems
  def gem_cache; @gem_cache ||= File.join(ENV['GEM_HOME'], 'cache') end

  ##
  # Load gemspec and change paths relative to current file
  def gem_spec
    @gem_spec ||= eval IO.read( testify_path "faster_path.gemspec" )
    reassociate_gemspec_paths
    @gem_spec
  end

  ##
  # Easily inspect gem file with Gem::Package object
  # https://github.com/rubygems/rubygems/blob/0749715e4bd9e7f0fb631a352ddc649574da91c1/lib/rubygems/package.rb#L35
  def the_gem; @the_gem ||= Gem::Package.new(gem_spec.cache_file) end

  def setup
    super
    Gem::Package.build gem_spec
    FileUtils.mv faster_path, gem_spec.cache_file
    @root = Pathname.new(@tempdir)
  end

  def test_cache_file
    assert File.exist? gem_spec.cache_file
  end

  def test_install_gem_in_rvm_gemset
    skip 'TODO: Need to install into empty RVM gemset and verify ext compilation'
  end
  
  def test_install_gem_in_bundler_vendor
    skip 'TODO: Need to bundle install to a vendor directory here and verify ext compilation'
    #Dir.chdir @tempdir do
    #  bundle :package, "cache-path" => "vendor"
    #  assert_equal Dir.new(bundled_app(faster_path).dirname).entries, ""
    #end
  end

  private
  def reassociate_gemspec_paths
    @gem_specd ||= false
    [:files, :bindir, :executables, :extensions, :require_paths].each do |path|
      @gem_spec.send "#{path}=", if @gem_spec.send(path).is_a? String
        testify_path @gem_spec.send(path)
      else
        @gem_spec.send(path).reduce([]){ |a,v| a << testify_path(v); a }
      end
    end unless @gem_specd
    @gem_specd = true
  end

  def testify_path file
    File.expand_path("../../#{file}", __FILE__)
  end
end
