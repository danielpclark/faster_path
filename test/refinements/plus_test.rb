require 'test_helper'

class RefinedPathname
  using FasterPath::RefinePathname
  def plus(*args)
    Pathname.plus(*args)
  end
end

class BasenameTest < Minitest::Test
  # Tests copied from https://searchcode.com/codesearch/view/12785140/
  def test_it_creates_basename_correctly
    assert_equal('/', RefinedPathname.allocate.send(:plus, '/', '/'))
    assert_equal('a/b', RefinedPathname.allocate.send(:plus, 'a', 'b'))
    assert_equal('a', RefinedPathname.allocate.send(:plus, 'a', '.'))
    assert_equal('b', RefinedPathname.allocate.send(:plus, '.', 'b'))
    assert_equal('.', RefinedPathname.allocate.send(:plus, '.', '.'))
    assert_equal('/b', RefinedPathname.allocate.send(:plus, 'a', '/b'))

    assert_equal('/', RefinedPathname.allocate.send(:plus, '/', '..'))
    assert_equal('.', RefinedPathname.allocate.send(:plus, 'a', '..'))
    assert_equal('a', RefinedPathname.allocate.send(:plus, 'a/b', '..'))
    assert_equal('../..', RefinedPathname.allocate.send(:plus, '..', '..'))
    assert_equal('/c', RefinedPathname.allocate.send(:plus, '/', '../c'))
    assert_equal('c', RefinedPathname.allocate.send(:plus, 'a', '../c'))
    assert_equal('a/c', RefinedPathname.allocate.send(:plus, 'a/b', '../c'))
    assert_equal('../../c', RefinedPathname.allocate.send(:plus, '..', '../c'))

    assert_equal('a//b/d//e', RefinedPathname.allocate.send(:plus, 'a//b/c', '../d//e'))

    assert_equal('//foo/var/bar', RefinedPathname.allocate.send(:plus, '//foo/var', 'bar'))
  end
end
