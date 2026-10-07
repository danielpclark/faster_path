require 'test_helper'
require 'thermite/tasks'

# What the Rakefiles rely on in thermite
class ThermiteTest < Minitest::Test
  def setup
    project_toplevel_dir = File.dirname(__dir__)
    @thermite = Thermite::Tasks.new(cargo_project_path: project_toplevel_dir,
                                    ruby_project_path: project_toplevel_dir,
                                    version: FasterPath::VERSION)
  end

  def test_release_tarballs_are_named_after_the_gem_version
    config = @thermite.config

    assert_equal FasterPath::VERSION, config.version
    refute_equal config.crate_version, config.version

    tarball = config.tarball_filename(config.version)

    assert_match(/\Afaster_path-#{Regexp.escape(FasterPath::VERSION)}-/, tarball)
    # The Ruby's major and minor version only: the extension works on every patch release
    assert_match(/-ruby\d\d-/, tarball)
    assert_match(/\.tar\.gz\z/, tarball)
  end

  def test_build_lib_can_clean_the_cargo_target
    assert_respond_to Thermite::Cargo.new(@thermite.config), :clean
  end
end
