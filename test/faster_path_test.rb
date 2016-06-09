require 'test_helper'

class RefinedPathname
  using FasterPath::RefinePathname
  def a?(v)
    Pathname.new(v).absolute?
  end
end if ENV["TEST_REFINEMENTS"]

class FasterPathTest < Minitest::Test
  def test_it_determins_absolute_path
    assert FasterPath.absolute?("/hello")
    refute FasterPath.absolute?("goodbye")
  end

  def test_refines_pathname_absolute?
    assert RefinedPathname.new.a?("/")
  end if ENV["TEST_REFINEMENTS"]

  def test_monkeypatches_pathname_absolute?
    FasterPath.monkeypatch_pathname
    assert Pathname.new("/").absolute?
  end if ENV["TEST_REFINEMENTS"]
end
