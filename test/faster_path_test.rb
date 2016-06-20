require 'test_helper'
require 'ffi'

class FasterPathTest < Minitest::Test
  def test_it_build_linked_library
    assert File.exist? begin
      prefix = Gem.win_platform? ? '' : 'lib'
      "#{File.expand_path('../target/release/', File.dirname(__FILE__))}/#{prefix}faster_path.#{FFI::Platform::LIBSUFFIX}"
    end
  end
end
