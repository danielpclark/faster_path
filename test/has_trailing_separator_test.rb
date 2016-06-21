require 'test_helper'

class HasTrailingSeparatorTest < Minitest::Test
  def test_has_trailing_separator_against_pathname_implementation
    result_pair = -> (str) {
      [
        Pathname.allocate.send(:has_trailing_separator?, str),
        FasterPath.has_trailing_separator?(str)
      ]
    }

    assert_equal(*result_pair.(''))
    assert_equal(*result_pair.('/'))
    assert_equal(*result_pair.('//'))
    assert_equal(*result_pair.('///'))
    assert_equal(*result_pair.('hello'))
    assert_equal(*result_pair.('hello/'))
    assert_equal(*result_pair.('/hello/'))
    assert_equal(*result_pair.('/hello//'))
    assert_equal(*result_pair.('.'))
    assert_equal(*result_pair.('./'))
    assert_equal(*result_pair.('.//'))
    assert_equal(*result_pair.("aa/a//a"))
    assert_equal(*result_pair.("/aaa/a//a"))
    assert_equal(*result_pair.("/aaa/a//a/a"))
    assert_equal(*result_pair.("/aaa/a//a/a"))
    assert_equal(*result_pair.("a//aaa/a//a/a"))
    assert_equal(*result_pair.("a//aaa/a//a/aaa"))
    assert_equal(*result_pair.("/aaa/a//a/aaa/a"))
    assert_equal(*result_pair.("a//aaa/a//a/aaa/a"))
    assert_equal(*result_pair.("a//aaa/a//a/aaa////"))
    assert_equal(*result_pair.("a/a//aaa/a//a/aaa/a"))
    assert_equal(*result_pair.("////a//aaa/a//a/aaa/a"))
    assert_equal(*result_pair.("////a//aaa/a//a/aaa////"))
    assert_equal(*result_pair.("."))
    assert_equal(*result_pair.(".././"))
    assert_equal(*result_pair.(".///.."))
    assert_equal(*result_pair.("/././/"))
    assert_equal(*result_pair.("//../././"))
    assert_equal(*result_pair.(".///.../.."))
    assert_equal(*result_pair.("/././/.//."))
    assert_equal(*result_pair.("/...//../././"))
    assert_equal(*result_pair.("/..///.../..//"))
    assert_equal(*result_pair.("/./././/.//..."))
    assert_equal(*result_pair.("/...//.././././/."))
    assert_equal(*result_pair.("./../..///.../..//"))
    assert_equal(*result_pair.("///././././/.//..."))
    assert_equal(*result_pair.("./../..///.../..//././"))
    assert_equal(*result_pair.("///././././/.//....///"))
    assert_equal(*result_pair.("http://www.example.com"))
    assert_equal(*result_pair.("foor for thought"))
    assert_equal(*result_pair.("2gb63b@%TY25GHawefb3/g3qb"))
  end

  def test_has_trailing_separator_with_nil
    refute FasterPath.has_trailing_separator?(nil)
  end
end
