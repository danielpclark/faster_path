require 'test_helper'

class DirectoryTest < Minitest::Test
  def test_nil_for_directory?
    refute FasterPath.directory? nil
  end

  def test_of_As
    result_pair = lambda do |str|
      [
        Pathname.new(str).send(:directory?),
        FasterPath.directory?(str)
      ]
    end
    assert_equal(*result_pair.call('aa/a//a'))
    assert_equal(*result_pair.call('/aaa/a//a'))
    assert_equal(*result_pair.call('/aaa/a//a/a'))
    assert_equal(*result_pair.call('/aaa/a//a/a'))
    assert_equal(*result_pair.call('a//aaa/a//a/a'))
    assert_equal(*result_pair.call('a//aaa/a//a/aaa'))
    assert_equal(*result_pair.call('/aaa/a//a/aaa/a'))
    assert_equal(*result_pair.call('a//aaa/a//a/aaa/a'))
    assert_equal(*result_pair.call('a//aaa/a//a/aaa////'))
    assert_equal(*result_pair.call('a/a//aaa/a//a/aaa/a'))
    assert_equal(*result_pair.call('////a//aaa/a//a/aaa/a'))
    assert_equal(*result_pair.call('////a//aaa/a//a/aaa////'))
  end

  def test_of_Bs
    result_pair = lambda do |str|
      [
        Pathname.new(str).send(:directory?),
        FasterPath.directory?(str)
      ]
    end
    assert_equal(*result_pair.call('.'))
    assert_equal(*result_pair.call('.././'))
    assert_equal(*result_pair.call('.///..'))
    assert_equal(*result_pair.call('/././/'))
    assert_equal(*result_pair.call('//../././'))
    assert_equal(*result_pair.call('.///.../..'))
    assert_equal(*result_pair.call('/././/.//.'))
    assert_equal(*result_pair.call('/...//../././'))
    assert_equal(*result_pair.call('/..///.../..//'))
    assert_equal(*result_pair.call('/./././/.//...'))
    assert_equal(*result_pair.call('/...//.././././/.'))
    assert_equal(*result_pair.call('./../..///.../..//'))
    assert_equal(*result_pair.call('///././././/.//...'))
    assert_equal(*result_pair.call('./../..///.../..//././'))
    assert_equal(*result_pair.call('///././././/.//....///'))
  end

  def test_of_Cs
    result_pair = lambda do |str|
      [
        Pathname.new(str).send(:directory?),
        FasterPath.directory?(str)
      ]
    end
    assert_equal(*result_pair.call('http://www.example.com'))
    assert_equal(*result_pair.call('foor for thought'))
    assert_equal(*result_pair.call('2gb63b@%TY25GHawefb3/g3qb'))
  end
end
