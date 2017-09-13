require 'test_helper'

class FasterPathnameTest < Minitest::Test
  def test_it_safely_takes_nil
    assert FasterPath.new(nil)
  end

  def test_it_creates_itself
    assert FasterPathname.new("/hello")
    assert_kind_of FasterPathname.new("/hello"), FasterPathname
  end

  def test_it_catches_null_byte
    assert_raises ArgumentError do
      FasterPathname.new("\0")
    end
  end
end
