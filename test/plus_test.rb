require 'test_helper'

class PlusTest < Minitest::Test
  def test_plus
    assert_equal('/', FasterPath.plus('/', '/'))
    assert_equal('a/b', FasterPath.plus('a', 'b'))
    assert_equal('a', FasterPath.plus('a', '.'))
    assert_equal('b', FasterPath.plus('.', 'b'))
    assert_equal('.', FasterPath.plus('.', '.'))
    assert_equal('/b', FasterPath.plus('a', '/b'))

    assert_equal('/', FasterPath.plus('/', '..'))
    assert_equal('.', FasterPath.plus('a', '..'))
    assert_equal('a', FasterPath.plus('a/b', '..'))
    assert_equal('../..', FasterPath.plus('..', '..'))
    assert_equal('/c', FasterPath.plus('/', '../c'))
    assert_equal('c', FasterPath.plus('a', '../c'))
    assert_equal('a/c', FasterPath.plus('a/b', '../c'))
    assert_equal('../../c', FasterPath.plus('..', '../c'))

    assert_equal('a//b/d//e', FasterPath.plus('a//b/c', '../d//e'))

    assert_equal('//foo/var/bar', FasterPath.plus('//foo/var', 'bar'))
  end
end
