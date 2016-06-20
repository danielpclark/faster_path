require 'test_helper'
require 'faster_path/optional/refinements'

class RefinedPathname
  using FasterPath::RefinePathname
  def plus(a, b)
    send(:+, a, b).to_s
  end

  def +(pth, other)
    Pathname(pth) + Pathname(other)
  end
end

class CleanpathAggressiveTest < Minitest::Test
  def test_plus
    assert_kind_of(Pathname, RefinedPathname.new.+('a', 'b'))
  end

  def plus(path1, path2) # -> path
    (Pathname.new(path1) + Pathname.new(path2)).to_s
  end

  def test_clean_aggresive_defaults
    assert_equal RefinedPathname.new.plus('/', '/'), '/'
    assert_equal RefinedPathname.new.plus('a', 'b'), 'a/b'
    assert_equal RefinedPathname.new.plus('a', '.'), 'a'
    assert_equal RefinedPathname.new.plus('.', 'b'), 'b'
    assert_equal RefinedPathname.new.plus('.', '.'), '.'
    assert_equal RefinedPathname.new.plus('a', '/b'), '/b'
    assert_equal RefinedPathname.new.plus('/', '..'), '/'
    assert_equal RefinedPathname.new.plus('a', '..'), '.'
    assert_equal RefinedPathname.new.plus('a/b', '..'), 'a'
    assert_equal RefinedPathname.new.plus('..', '..'), '../..'
    assert_equal RefinedPathname.new.plus('/', '../c'), '/c'
    assert_equal RefinedPathname.new.plus('a', '../c'), 'c'
    assert_equal RefinedPathname.new.plus('a/b', '../c'), 'a/c'
    assert_equal RefinedPathname.new.plus('..', '../c'), '../../c'
    assert_equal RefinedPathname.new.plus('a//b/c', '../d//e'), 'a//b/d//e'
  end
end
