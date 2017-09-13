require 'test_helper'

class ChildrenTest < Minitest::Test
  def test_it_returns_similar_results_to_pathname_children?
    [".", "/", "../"].each do |pth|
      assert_equal Pathname.new(pth).children.map(&:to_s),
                   FasterPath.children(pth)
    end
  end
end
