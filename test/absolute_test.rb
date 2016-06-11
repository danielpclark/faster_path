require 'test_helper'

class RefinedPathname
  using FasterPath::RefinePathname
  def a?(v)
    Pathname.new(v).absolute?
  end
end if ENV["TEST_REFINEMENTS"].to_s["true"]

class AbsoluteTest < Minitest::Test
  def test_it_determins_absolute_path
    assert FasterPath.absolute?("/hello")
    refute FasterPath.absolute?("goodbye")
  end

  def test_it_returns_similar_results_to_pathname_absolute?
    ["",".","/",".asdf","/asdf/asdf","/asdf/asdf.asdf","asdf/asdf.asd"].each do |pth|
      assert_equal Pathname.new(pth).absolute?,
                   FasterPath.absolute?(pth)
    end
  end

  def test_refines_pathname_absolute?
    assert RefinedPathname.new.a?("/")
  end if ENV["TEST_REFINEMENTS"].to_s["true"]

  def test_monkeypatches_pathname_absolute?
    FasterPath.sledgehammer_everything!
    assert Pathname.new("/").absolute?
  end if ENV["TEST_REFINEMENTS"].to_s["true"]
end
