require 'test_helper'
require 'thermite/tasks'

class ThermiteTest < Minitest::Test
  def setup
    project_toplevel_dir = File.dirname(__dir__)
    @thermite = Thermite::Tasks.new(cargo_project_path: project_toplevel_dir,
                                    ruby_project_path: project_toplevel_dir)
  end

  def test_has_methods_needed_for_monkeypatch
    assert Thermite::Config.public_method_defined?(:ruby_version), 'Thermite ruby_version not defined!'
    assert Thermite::Tasks.public_method_defined?(:github_download_uri), 'Thermite github_download_uri not defined!'

    assert_match(/ruby\d+\.\d+\.\d+/, Thermite::Config.new.ruby_version)

    gh_uri = @thermite.github_download_uri('lol', '0.0.1')

    assert_match(/faster_path/, gh_uri)
    assert_match(/0\.0\.1/, gh_uri)
    assert_match(/#{FasterPath::VERSION}/, gh_uri)
    assert_match(/github/, gh_uri)
    assert_match(/tar\.gz/, gh_uri)
  end
end
