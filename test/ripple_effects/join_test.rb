require 'test_helper'
require 'faster_path/optional/refinements'

class RefinedPathname
#  using FasterPath::RefinePathname
#  using FasterPath::RefineFile
  def join(*args)
    a, *rest = args
    Pathname(a).join(*rest)
  end
end

class JoinTest < Minitest::Test
  def test_join
    r = RefinedPathname.new.join("a", Pathname("b"), Pathname("c"))
    assert_equal(Pathname("a/b/c"), r)
    r = RefinedPathname.new.join("/a", Pathname("b"), Pathname("c"))
    assert_equal(Pathname("/a/b/c"), r)
    r = RefinedPathname.new.join("/a", Pathname("/b"), Pathname("c"))
    assert_equal(Pathname("/b/c"), r)
    r = RefinedPathname.new.join("/a", Pathname("/b"), Pathname("/c"))
    assert_equal(Pathname("/c"), r)
    r = RefinedPathname.new.join("/a", "/b", "/c")
    assert_equal(Pathname("/c"), r)
    r = RefinedPathname.new.join("/foo/var")
    assert_equal(Pathname("/foo/var"), r)
  end
end
