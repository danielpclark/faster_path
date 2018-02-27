require 'test_helper'

class JoinTest < Minitest::Test
  def test_join
    r = FasterPath.join(Pathname("a"), Pathname("b"), Pathname("c"))
    assert_equal(Pathname("a/b/c"), r)
    r = FasterPath.join(Pathname("/a"), Pathname("b"), Pathname("c"))
    assert_equal(Pathname("/a/b/c"), r)
    r = FasterPath.join(Pathname("/a"), Pathname("/b"), Pathname("c"))
    assert_equal(Pathname("/b/c"), r)
    r = FasterPath.join(Pathname("/a"), Pathname("/b"), Pathname("/c"))
    assert_equal(Pathname("/c"), r)
    r = FasterPath.join(Pathname("/a"), "/b", "/c")
    assert_equal(Pathname("/c"), r)
    r = FasterPath.join(Pathname("/foo/var"))
    assert_equal(Pathname("/foo/var"), r)
  end
end
