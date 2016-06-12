require 'test_helper'
require 'faster_path/optional/refinements'

class RefinedPathname
  using FasterPath::RefinePathname
  using FasterPath::RefineFile
  def cleanpath_aggressive(pth)
    Pathname.new(pth).send(:cleanpath_aggressive).to_s
  end 
end

class CleanpathAggressiveTest < Minitest::Test
  def test_clean_aggressive_defaults
    assert_equal RefinedPathname.new.cleanpath_aggressive('/')                     , '/'
    assert_equal RefinedPathname.new.cleanpath_aggressive('')                      , '.'
    assert_equal RefinedPathname.new.cleanpath_aggressive('.')                     , '.'
    assert_equal RefinedPathname.new.cleanpath_aggressive('..')                    , '..'
    assert_equal RefinedPathname.new.cleanpath_aggressive('a')                     , 'a'
    assert_equal RefinedPathname.new.cleanpath_aggressive('/.')                    , '/'
    assert_equal RefinedPathname.new.cleanpath_aggressive('/..')                   , '/'
    assert_equal RefinedPathname.new.cleanpath_aggressive('/a')                    , '/a'
    assert_equal RefinedPathname.new.cleanpath_aggressive('./')                    , '.'
    assert_equal RefinedPathname.new.cleanpath_aggressive('../')                   , '..'
    assert_equal RefinedPathname.new.cleanpath_aggressive('a/')                    , 'a'
    assert_equal RefinedPathname.new.cleanpath_aggressive('a//b')                  , 'a/b'
    assert_equal RefinedPathname.new.cleanpath_aggressive('a/.')                   , 'a'
    assert_equal RefinedPathname.new.cleanpath_aggressive('a/./')                  , 'a'
    assert_equal RefinedPathname.new.cleanpath_aggressive('a/..')                  , '.'
    assert_equal RefinedPathname.new.cleanpath_aggressive('a/../')                 , '.'
    assert_equal RefinedPathname.new.cleanpath_aggressive('/a/.')                  , '/a'
    assert_equal RefinedPathname.new.cleanpath_aggressive('./..')                  , '..'
    assert_equal RefinedPathname.new.cleanpath_aggressive('../.')                  , '..'
    assert_equal RefinedPathname.new.cleanpath_aggressive('./../')                 , '..'
    assert_equal RefinedPathname.new.cleanpath_aggressive('.././')                 , '..'
    assert_equal RefinedPathname.new.cleanpath_aggressive('/./..')                 , '/'
    assert_equal RefinedPathname.new.cleanpath_aggressive('/../.')                 , '/'
    assert_equal RefinedPathname.new.cleanpath_aggressive('/./../')                , '/'
    assert_equal RefinedPathname.new.cleanpath_aggressive('/.././')                , '/'
    assert_equal RefinedPathname.new.cleanpath_aggressive('a/b/c')                 , 'a/b/c'
    assert_equal RefinedPathname.new.cleanpath_aggressive('./b/c')                 , 'b/c'
    assert_equal RefinedPathname.new.cleanpath_aggressive('a/./c')                 , 'a/c'
    assert_equal RefinedPathname.new.cleanpath_aggressive('a/b/.')                 , 'a/b'
    assert_equal RefinedPathname.new.cleanpath_aggressive('a/../.')                , '.'
    assert_equal RefinedPathname.new.cleanpath_aggressive('/../.././../a')         , '/a'
    assert_equal RefinedPathname.new.cleanpath_aggressive('a/b/../../../../c/../d'), '../../d' 
  end

  def test_clean_aggressive_dosish_stuff
    if File.dirname("//") == "//" # DOSISH_UNC
      assert_equal RefinedPathname.new.cleanpath_aggressive('//a/b/c/')  , '//a/b/c'
    else
      assert_equal RefinedPathname.new.cleanpath_aggressive('///')       , '/'
      assert_equal RefinedPathname.new.cleanpath_aggressive('///a')      , '/a'
      assert_equal RefinedPathname.new.cleanpath_aggressive('///..')     , '/'
      assert_equal RefinedPathname.new.cleanpath_aggressive('///.')      , '/'
      assert_equal RefinedPathname.new.cleanpath_aggressive('///a/../..'), '/'
    end

    if File::ALT_SEPARATOR != nil # DOSISH
      assert_equal RefinedPathname.new.cleanpath_aggressive('c:\\foo\\bar'), 'c:/foo/bar'
    end
  end
end
