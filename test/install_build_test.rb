require 'test_helper.rb'
require 'rubygems/installer_test_case'
require 'rubygems/package'

class InstallBuildTest < Gem::InstallerTestCase
  def faster_path; "faster_path-#{FasterPath::VERSION}.gem" end
  def gem_cache; @gem_cache ||= File.join(ENV['GEM_HOME'], 'cache') end
  def gem_spec
    @gem_spec ||= eval IO.read( File.expand_path('../../faster_path.gemspec', __FILE__) )
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
        File.expand_path(File.join('..', '..', @gem_spec.send(path)), __FILE__)
      else
        @gem_spec.send(path).reduce([]){ |array, value|
          array << File.expand_path(File.join('..', '..', value), __FILE__)
          array
        }
      end
    end unless @gem_specd
    @gem_specd = true
  end
end
