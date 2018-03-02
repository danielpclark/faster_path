require 'test_helper'
class RelativePathFromTest < Minitest::Test
  def test_relative_path_from
    assert_equal FasterPath.relative_path_from("a",           "b").to_s,         "../a"
    assert_equal FasterPath.relative_path_from("a",           "b/").to_s,        "../a"
    assert_equal FasterPath.relative_path_from("a/",          "b").to_s,         "../a"
    assert_equal FasterPath.relative_path_from("a/",          "b/").to_s,        "../a"
    assert_equal FasterPath.relative_path_from("/a",          "/b").to_s,        "../a"
    assert_equal FasterPath.relative_path_from("/a",          "/b/").to_s,       "../a"
    assert_equal FasterPath.relative_path_from("/a/",         "/b").to_s,        "../a"
    assert_equal FasterPath.relative_path_from("/a/",         "/b/").to_s,       "../a"
    assert_equal FasterPath.relative_path_from("a/b",         "a/c").to_s,       "../b"
    assert_equal FasterPath.relative_path_from("../a",        "../b").to_s,      "../a"
    #assert_equal FasterPath.relative_path_from("a",           ".").to_s,         "a"
    #assert_equal FasterPath.relative_path_from(".",           "a").to_s,         ".."
    assert_equal FasterPath.relative_path_from(".",           ".").to_s,         "."
    assert_equal FasterPath.relative_path_from("..",          "..").to_s,        "."
    #assert_equal FasterPath.relative_path_from("..",          ".").to_s,         ".."
    #assert_equal FasterPath.relative_path_from("/a/b/c/d",    "/a/b").to_s,      "c/d"
    #assert_equal FasterPath.relative_path_from("/a/b",        "/a/b/c/d").to_s,  "../.."
    #assert_equal FasterPath.relative_path_from("/e",          "/a/b/c/d").to_s,  "../../../../e"
    #assert_equal FasterPath.relative_path_from("a/b/c",       "a/d").to_s,       "../b/c"
    assert_equal FasterPath.relative_path_from("/../a",       "/b").to_s,        "../a"
    #assert_equal FasterPath.relative_path_from("../a",        "b").to_s,         "../../a"
    assert_equal FasterPath.relative_path_from("/a/../../b",  "/b").to_s,        "."
    #assert_equal FasterPath.relative_path_from("a/..",        "a").to_s,         ".."
    assert_equal FasterPath.relative_path_from("a/../b",      "b").to_s,         "."
    #assert_equal FasterPath.relative_path_from("a",           "b/..").to_s,      "a"
    #assert_equal FasterPath.relative_path_from("b/c",         "b/..").to_s,      "b/c"
    assert_raises(ArgumentError) { FasterPath.relative_path_from("/", ".")  }
    assert_raises(ArgumentError) { FasterPath.relative_path_from(".", "/")  }
    assert_raises(ArgumentError) { FasterPath.relative_path_from("a", "..") }
    #assert_raises(ArgumentError) { FasterPath.relative_path_from(".", "..") }
  end
end
