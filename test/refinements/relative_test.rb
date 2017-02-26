require 'test_helper'
require "faster_path/optional/refinements"
# At the moment refinements don't allow introspection

class RefinedPathname
  using FasterPath::RefinePathname
  def relative?(v)
    Pathname.new(v).relative?
  end
end

class RelativeRefinementTest < Minitest::Test
  def test_refines_pathname_relative?
    refute RefinedPathname.new.relative?("/")
  end

  def test_nil_behaves_the_same
    assert_raises(TypeError) { RefinedPathname.new.relative?(nil) }
    assert_raises(TypeError) { Pathname.new(nil).relative? }
  end
end
