require 'test_helper.rb'
require 'rubygems/installer_test_case'
require 'rubygems/package'

class InstallBuildTest < Gem::InstallerTestCase
  def faster_path; "faster_path-#{FasterPath::VERSION}.gem" end
  def gem_cache; @gem_cache ||= File.join(ENV['GEM_HOME'], 'cache') end
  def gem_spec
    @gem_spec ||= eval IO.read( testify_path "faster_path.gemspec" )
    reassociate_gemspec_paths
    @gem_spec
  end

  def setup
    super
    Gem::Package.build gem_spec
    FileUtils.mv faster_path, gem_spec.cache_file
  end

  def test_cache_file
    assert File.exist? gem_spec.cache_file
  end

  def test_install_gem
    skip 'TODO'
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
