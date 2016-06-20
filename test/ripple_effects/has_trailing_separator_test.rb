require 'test_helper'
require 'faster_path/optional/refinements'

class RefinedPathname
  using FasterPath::RefinePathname
  def has_trailing_separator?(pth)
    Pathname.allocate.send(:has_trailing_separator?, pth)
  end
end

class HasTrailingSeparatorTest < Minitest::Test
  def test_has_trailing_separator?
    refute RefinedPathname.new.has_trailing_separator?('/')
    refute RefinedPathname.new.has_trailing_separator?('///')
    refute RefinedPathname.new.has_trailing_separator?('a')
    assert RefinedPathname.new.has_trailing_separator?('a/')
  end
end
