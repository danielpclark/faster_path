class RefinedPathname
  using FasterPath::RefinePathname
  def has_trailing_separator?(path)
    Pathname.new(path).has_trailing_separator?
  end
end

class HasTrailingSeparatorRefinementTest < Minitest::Test
  def test_refines_pathname_has_trailing_separator?
    assert RefinedPathname.new.has_trailing_separator? "a/"
  end
end
