require 'test_helper'

class CleanpathAggressiveTest < Minitest::Test
  def test_it_creates_cleanpath_aggressive_correctly_part_1
    assert_equal FasterPath.cleanpath_aggressive('/'                     ), '/'
    assert_equal FasterPath.cleanpath_aggressive(''                      ), '.'
    assert_equal FasterPath.cleanpath_aggressive('.'                     ), '.'
    assert_equal FasterPath.cleanpath_aggressive('..'                    ), '..'
    assert_equal FasterPath.cleanpath_aggressive('a'                     ), 'a'
    assert_equal FasterPath.cleanpath_aggressive('/.'                    ), '/'
    assert_equal FasterPath.cleanpath_aggressive('/..'                   ), '/'
    assert_equal FasterPath.cleanpath_aggressive('/a'                    ), '/a'
    assert_equal FasterPath.cleanpath_aggressive('./'                    ), '.'
    assert_equal FasterPath.cleanpath_aggressive('../'                   ), '..'
    assert_equal FasterPath.cleanpath_aggressive('a/'                    ), 'a'
    assert_equal FasterPath.cleanpath_aggressive('a//b'                  ), 'a/b'
    assert_equal FasterPath.cleanpath_aggressive('a/.'                   ), 'a'
    assert_equal FasterPath.cleanpath_aggressive('a/./'                  ), 'a'
    assert_equal FasterPath.cleanpath_aggressive('a/..'                  ), '.'
    assert_equal FasterPath.cleanpath_aggressive('a/../'                 ), '.'
  end

  def test_it_creates_cleanpath_aggressive_correctly_part_2
    assert_equal FasterPath.cleanpath_aggressive('/a/.'                  ), '/a'
    assert_equal FasterPath.cleanpath_aggressive('./..'                  ), '..'
    assert_equal FasterPath.cleanpath_aggressive('../.'                  ), '..'
    assert_equal FasterPath.cleanpath_aggressive('./../'                 ), '..'
    assert_equal FasterPath.cleanpath_aggressive('.././'                 ), '..'
    assert_equal FasterPath.cleanpath_aggressive('/./..'                 ), '/'
    assert_equal FasterPath.cleanpath_aggressive('/../.'                 ), '/'
    assert_equal FasterPath.cleanpath_aggressive('/./../'                ), '/'
    assert_equal FasterPath.cleanpath_aggressive('/.././'                ), '/'
    assert_equal FasterPath.cleanpath_aggressive('a/b/c'                 ), 'a/b/c'
    assert_equal FasterPath.cleanpath_aggressive('./b/c'                 ), 'b/c'
    assert_equal FasterPath.cleanpath_aggressive('a/./c'                 ), 'a/c'
    assert_equal FasterPath.cleanpath_aggressive('a/b/.'                 ), 'a/b'
    assert_equal FasterPath.cleanpath_aggressive('a/../.'                ), '.'
    assert_equal FasterPath.cleanpath_aggressive('/../.././../a'         ), '/a'
    assert_equal FasterPath.cleanpath_aggressive('a/b/../../../../c/../d'), '../../d'
  end
end
