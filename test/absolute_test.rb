require 'test_helper'

class AbsoluteTest < Minitest::Test
  def test_it_determins_absolute_path
    assert FasterPath.absolute?("/hello")
    refute FasterPath.absolute?("goodbye")
  end

  def test_it_returns_similar_results_to_pathname_absolute?
    ["",".","/",".asdf","/asdf/asdf","/asdf/asdf.asdf","asdf/asdf.asd"].each do |pth|
      assert_equal Pathname.new(pth).absolute?,
                   FasterPath.absolute?(pth)
    end
  end
end
