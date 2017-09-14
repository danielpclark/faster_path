require 'test_helper'

class ChildrenTest < Minitest::Test
  def test_pathnames_existing_behavior
    assert_kind_of Pathname, Pathname.new('.').children.first
  end

  def test_children_returns_string_objects
    assert_kind_of String, FasterPath.children('.').first
  end

  def test_children_compat_returns_pathname_objects
    assert_kind_of Pathname, FasterPathname::Public.allocate.send(:children_compat, '.').first
  end

  def test_it_returns_similar_results_to_pathname_children
    [".", "/", "../"].each do |pth|
      assert_equal Pathname.new(pth).children,
        FasterPathname::Public.allocate.send(:children_compat, pth)
    end
  end
end
