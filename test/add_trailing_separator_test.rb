require 'test_helper'

class AddTrailingSeparatorTest < Minitest::Test
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
end
