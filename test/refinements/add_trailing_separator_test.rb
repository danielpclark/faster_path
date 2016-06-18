require 'test_helper'
require 'faster_path/optional/refinements'

class RefinedPathname
  using FasterPath::RefinePathname
  def add_trailing_separator(pth)
    Pathname.allocate.send(:add_trailing_separator, pth)
  end
end

class AddTrailingSeparatorTest < Minitest::Test
  def test_refines_pathname_add_trailing_separator
    assert RefinedPathname.allocate.send(:add_trailing_separator, 'hello')
  end

  def test_refines_pathname_add_trailing_separator_in_dosish_context
    if File.dirname('A:') == 'A:.'
      assert RefinedPathname.allocate.send(:add_trailing_separator, 'A:')
    end
  end
end
