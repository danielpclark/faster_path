require 'test_helper'

class AddTrailingSeparatorTest < Minitest::Test
  def test_it_handles_nil
    refute FasterPath.add_trailing_separator nil
  end

  def test_add_trailing_separator
    assert_equal FasterPath.add_trailing_separator(''), ''
    assert_equal FasterPath.add_trailing_separator('/'), '/'
    assert_equal FasterPath.add_trailing_separator('//'), '//'
    assert_equal FasterPath.add_trailing_separator('hello'), 'hello/'
    assert_equal FasterPath.add_trailing_separator('hello/'), 'hello/'
    assert_equal FasterPath.add_trailing_separator('/hello/'), '/hello/'
    assert_equal FasterPath.add_trailing_separator('/hello//'), '/hello//'
    assert_equal FasterPath.add_trailing_separator('.'), './'
    assert_equal FasterPath.add_trailing_separator('./'), './'
    assert_equal FasterPath.add_trailing_separator('.//'), './/'
  end

  def test_add_trailing_separator_in_dosish_context
    if File.dirname('A:') == 'A:.'
      assert_equal FasterPath.add_trailing_separator('A:'), 'A:/'
      assert_equal FasterPath.add_trailing_separator('A:/'), 'A:/'
      assert_equal FasterPath.add_trailing_separator('A://'), 'A://'
      assert_equal FasterPath.add_trailing_separator('A:.'), 'A:./'
      assert_equal FasterPath.add_trailing_separator('A:./'), 'A:./'
      assert_equal FasterPath.add_trailing_separator('A:.//'), 'A:.//'
    end
  end

  def test_add_trailing_separator_against_pathname_implementation
    result_pair = lambda do |str|
      [
        Pathname.allocate.send(:add_trailing_separator, str),
        FasterPath.add_trailing_separator(str)
      ]
    end

    assert_equal(*result_pair.call(''))
    assert_equal(*result_pair.call('/'))
    assert_equal(*result_pair.call('//'))
    assert_equal(*result_pair.call('hello'))
    assert_equal(*result_pair.call('hello/'))
    assert_equal(*result_pair.call('/hello/'))
    assert_equal(*result_pair.call('/hello//'))
    assert_equal(*result_pair.call('.'))
    assert_equal(*result_pair.call('./'))
    assert_equal(*result_pair.call('.//'))
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
    assert_equal(*result_pair.call('http://www.example.com'))
    assert_equal(*result_pair.call('foor for thought'))
    assert_equal(*result_pair.call('2gb63b@%TY25GHawefb3/g3qb'))
  end
end
