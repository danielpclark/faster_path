require 'test_helper'
require 'thermite/config'

class FasterPathTest < Minitest::Test
  def test_it_build_linked_library
    assert File.exist? begin
      toplevel_dir = File.dirname(File.dirname(__FILE__))
      config = Thermite::Config.new(cargo_project_path: toplevel_dir,
                                    ruby_project_path: toplevel_dir)
      config.ruby_extension_path
    end
  end
end
