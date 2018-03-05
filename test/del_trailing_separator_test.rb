require 'test_helper'

class DelTrailingSeparatorTest < Minitest::Test
  def test_del_trailing_separator
    assert_equal FasterPath.del_trailing_separator(""),      ""
    assert_equal FasterPath.del_trailing_separator("/"),     "/"
    assert_equal FasterPath.del_trailing_separator("/a"),    "/a"
    assert_equal FasterPath.del_trailing_separator("/a/"),   "/a"
    assert_equal FasterPath.del_trailing_separator("/a//"),  "/a"
    assert_equal FasterPath.del_trailing_separator("."),     "."
    assert_equal FasterPath.del_trailing_separator("./"),    "."
    assert_equal FasterPath.del_trailing_separator(".//"),   "."
  end

  if DOSISH_DRIVE_LETTER
    def test_del_trailing_separator_dos_drive_letter
      assert_equal FasterPath.del_trailing_separator("A:"),    "A:"
      assert_equal FasterPath.del_trailing_separator("A:/"),   "A:/"
      assert_equal FasterPath.del_trailing_separator("A://"),  "A:/"
      assert_equal FasterPath.del_trailing_separator("A:."),   "A:."
      assert_equal FasterPath.del_trailing_separator("A:./"),  "A:."
      assert_equal FasterPath.del_trailing_separator("A:.//"), "A:."
    end
  end

  def test_del_trailing_separator_dos_unc
    if DOSISH_UNC
      assert_equal FasterPath.del_trailing_separator("//"),        "//"
      assert_equal FasterPath.del_trailing_separator("//a"),       "//a"
      assert_equal FasterPath.del_trailing_separator("//a/"),      "//a"
      assert_equal FasterPath.del_trailing_separator("//a//"),     "//a"
      assert_equal FasterPath.del_trailing_separator("//a/b"),     "//a/b"
      assert_equal FasterPath.del_trailing_separator("//a/b/"),    "//a/b"
      assert_equal FasterPath.del_trailing_separator("//a/b//"),   "//a/b"
      assert_equal FasterPath.del_trailing_separator("//a/b/c"),   "//a/b/c"
      assert_equal FasterPath.del_trailing_separator("//a/b/c/"),  "//a/b/c"
      assert_equal FasterPath.del_trailing_separator("//a/b/c//"), "//a/b/c"
    else
      assert_equal FasterPath.del_trailing_separator("///"),       "/"
      assert_equal FasterPath.del_trailing_separator("///a/"),     "///a"
    end
  end

  if DOSISH
    def test_del_trailing_separator_dos
      assert_equal FasterPath.del_trailing_separator("a\\"), "a"
      assert_equal FasterPath.del_trailing_separator("\225\\\\".dup.force_encoding("cp932")), "\225\\".dup.force_encoding("cp932")
      assert_equal FasterPath.del_trailing_separator("\225\\\\".dup.force_encoding("cp437")), "\225".dup.force_encoding("cp437")
    end
  end
end
