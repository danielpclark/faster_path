require 'test_helper'

class CleanpathConservativeTest < Minitest::Test
  def test_it_creates_cleanpath_conservative_correctly_part_1
    assert_equal FasterPath.cleanpath_conservative("/"),                      "/"
    assert_equal FasterPath.cleanpath_conservative(""),                       "."
    assert_equal FasterPath.cleanpath_conservative("."),                      "."
    assert_equal FasterPath.cleanpath_conservative(".."),                     ".."
    assert_equal FasterPath.cleanpath_conservative("a"),                      "a"
    assert_equal FasterPath.cleanpath_conservative("/."),                     "/"
    assert_equal FasterPath.cleanpath_conservative("/.."),                    "/"
    assert_equal FasterPath.cleanpath_conservative("/a"),                     "/a"
    assert_equal FasterPath.cleanpath_conservative("./"),                     "."
    assert_equal FasterPath.cleanpath_conservative("../"),                    ".."
    assert_equal FasterPath.cleanpath_conservative("a/"),                     "a/"
    assert_equal FasterPath.cleanpath_conservative("a//b"),                   "a/b"
    assert_equal FasterPath.cleanpath_conservative("a/."),                    "a/."
    assert_equal FasterPath.cleanpath_conservative("a/./"),                   "a/."
    assert_equal FasterPath.cleanpath_conservative("a/../"),                  "a/.."
    assert_equal FasterPath.cleanpath_conservative("/a/."),                   "/a/."
  end

  def test_it_creates_cleanpath_conservative_correctly_part_2
    assert_equal FasterPath.cleanpath_conservative("./.."),                   ".."
    assert_equal FasterPath.cleanpath_conservative("../."),                   ".."
    assert_equal FasterPath.cleanpath_conservative("./../"),                  ".."
    assert_equal FasterPath.cleanpath_conservative(".././"),                  ".."
    assert_equal FasterPath.cleanpath_conservative("/./.."),                  "/"
    assert_equal FasterPath.cleanpath_conservative("/../."),                  "/"
    assert_equal FasterPath.cleanpath_conservative("/./../"),                 "/"
    assert_equal FasterPath.cleanpath_conservative("/.././"),                 "/"
    assert_equal FasterPath.cleanpath_conservative("a/b/c"),                  "a/b/c"
    assert_equal FasterPath.cleanpath_conservative("./b/c"),                  "b/c"
    assert_equal FasterPath.cleanpath_conservative("a/./c"),                  "a/c"
    assert_equal FasterPath.cleanpath_conservative("a/b/."),                  "a/b/."
    assert_equal FasterPath.cleanpath_conservative("a/../."),                 "a/.."
    assert_equal FasterPath.cleanpath_conservative("/../.././../a"),          "/a"
    assert_equal FasterPath.cleanpath_conservative("a/b/../../../../c/../d"), "a/b/../../../../c/../d"
  end

  def test_windows_compat
    if DOSISH
      assert_equal FasterPath.cleanpath_conservative('c:/foo/bar'), 'c:\\foo\\bar'
    end

    if DOSISH_UNC
      assert_equal FasterPath.cleanpath_conservative('//'), '//'
    else
      assert_equal FasterPath.cleanpath_conservative('//'), '/'
    end
  end
end
