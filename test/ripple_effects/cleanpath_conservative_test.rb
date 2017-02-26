require 'test_helper'
require 'faster_path/optional/refinements'

class RefinedPathname
  using FasterPath::RefinePathname
  using FasterPath::RefineFile
  def cleanpath_conservative(pth)
    Pathname.new(pth).send(:cleanpath_conservative).to_s
  end
end

class CleanpathConservativeTest < Minitest::Test
  def test_clean_conservative_defaults1
    assert_equal RefinedPathname.new.cleanpath_conservative('/'), '/'
    assert_equal RefinedPathname.new.cleanpath_conservative(''), '.'
    assert_equal RefinedPathname.new.cleanpath_conservative('.'), '.'
    assert_equal RefinedPathname.new.cleanpath_conservative('..'), '..'
    assert_equal RefinedPathname.new.cleanpath_conservative('a'), 'a'
    assert_equal RefinedPathname.new.cleanpath_conservative('/.'), '/'
    assert_equal RefinedPathname.new.cleanpath_conservative('/..'), '/'
    assert_equal RefinedPathname.new.cleanpath_conservative('/a'), '/a'
    assert_equal RefinedPathname.new.cleanpath_conservative('./'), '.'
    assert_equal RefinedPathname.new.cleanpath_conservative('../'), '..'
    assert_equal RefinedPathname.new.cleanpath_conservative('a/'), 'a/'
    assert_equal RefinedPathname.new.cleanpath_conservative('a//b'), 'a/b'
  end

  def test_clean_conservative_defaults2
    assert_equal RefinedPathname.new.cleanpath_conservative('a/.'), 'a/.'
    assert_equal RefinedPathname.new.cleanpath_conservative('a/./'), 'a/.'
    assert_equal RefinedPathname.new.cleanpath_conservative('a/../'), 'a/..'
    assert_equal RefinedPathname.new.cleanpath_conservative('/a/.'), '/a/.'
    assert_equal RefinedPathname.new.cleanpath_conservative('./..'), '..'
    assert_equal RefinedPathname.new.cleanpath_conservative('../.'), '..'
    assert_equal RefinedPathname.new.cleanpath_conservative('./../'), '..'
    assert_equal RefinedPathname.new.cleanpath_conservative('.././'), '..'
    assert_equal RefinedPathname.new.cleanpath_conservative('/./..'), '/'
    assert_equal RefinedPathname.new.cleanpath_conservative('/../.'), '/'
  end

  def test_clean_conservative_defaults3
    assert_equal RefinedPathname.new.cleanpath_conservative('/./../'), '/'
    assert_equal RefinedPathname.new.cleanpath_conservative('/.././'), '/'
    assert_equal RefinedPathname.new.cleanpath_conservative('a/b/c'), 'a/b/c'
    assert_equal RefinedPathname.new.cleanpath_conservative('./b/c'), 'b/c'
    assert_equal RefinedPathname.new.cleanpath_conservative('a/./c'), 'a/c'
    assert_equal RefinedPathname.new.cleanpath_conservative('a/b/.'), 'a/b/.'
    assert_equal RefinedPathname.new.cleanpath_conservative('a/../.'), 'a/..'
    assert_equal RefinedPathname.new.cleanpath_conservative('/../.././../a'), '/a'
    assert_equal RefinedPathname.new.cleanpath_conservative('a/b/../../../../c/../d'), 'a/b/../../../../c/../d'
  end

  def test_clean_conservative_dosish_stuff
    if !File::ALT_SEPARATOR.nil?
      assert_equal RefinedPathname.new.cleanpath_conservative('c:\\foo\\bar'), 'c:/foo/bar'
    end

    if File.dirname("//") == "//"
      assert_equal RefinedPathname.new.cleanpath_conservative('//'), '//'
    else
      assert_equal RefinedPathname.new.cleanpath_conservative('//'), '/'
    end
  end
end
