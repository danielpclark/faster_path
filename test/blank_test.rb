require 'test_helper'

class BlankTest < Minitest::Test
  def test_it_is_blank?
    assert FasterPath.blank? "  "
    assert FasterPath.blank? ""
  end
end
