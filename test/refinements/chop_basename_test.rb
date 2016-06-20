require 'test_helper'
require 'faster_path/optional/refinements'
# At the moment refinements don't allow introspection

class RefinedPathname
  using FasterPath::RefinePathname
  def chop_basename(v)
    Pathname.new(v).send :chop_basename, v
  end
end

class ChopBasenameRefinementTest < Minitest::Test
  def test_refines_pathname_chop_basename
    assert RefinedPathname.new.chop_basename('/hello/world')
  end
end
