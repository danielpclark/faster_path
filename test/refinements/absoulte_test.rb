require 'test_helper'
require "faster_path/optional/refinements"
# At the moment refinements don't allow introspection

class RefinedPathname
  using FasterPath::RefinePathname
  def absolute?(v)
    Pathname.new(v).absolute?
  end
end 

class AbsoluteRefinementTest < Minitest::Test
  def test_refines_pathname_absolute?
    assert RefinedPathname.new.absolute?("/")
  end 
end
