require 'test_helper'
require "faster_path/optional/refinements"
# At the moment refinements don't allow introspection

class RefinedPathname
  using FasterPath::RefinePathname
  def relative?(v)
    Pathname.new(v).absolute?
  end
end 

class RelativeRefinementTest < Minitest::Test
  def test_refines_pathname_relative?
    assert RefinedPathname.new.relative?("/")
  end 
end
