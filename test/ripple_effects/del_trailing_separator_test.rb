require 'test_helper'
require 'faster_path/optional/refinements'

class RefinedPathname
  using FasterPath::RefinePathname
  def del_trailing_separator(pth)
    Pathname.allocate.send(:del_trailing_separator, pth)
  end
end

class DelTrailingSeparatorTest < Minitest::Test
  def test_del_trailing_separator
    assert_equal RefinedPathname.new.del_trailing_separator('/'), '/'
    assert_equal RefinedPathname.new.del_trailing_separator('/a'), '/a'
    assert_equal RefinedPathname.new.del_trailing_separator('/a/'), '/a'
    assert_equal RefinedPathname.new.del_trailing_separator('/a//'), '/a'
    assert_equal RefinedPathname.new.del_trailing_separator('.'), '.'
    assert_equal RefinedPathname.new.del_trailing_separator('./'), '.'
    assert_equal RefinedPathname.new.del_trailing_separator('.//'), '.'
  end

  def test_del_trailing_separator_in_dosish_context
    if File.dirname('A:') == 'A:.' # DOSISH_DRIVE_LETTER
      assert_equal RefinedPathname.new.del_trailing_separator('A:'), 'A:'
      assert_equal RefinedPathname.new.del_trailing_separator('A:/'), 'A:/'
      assert_equal RefinedPathname.new.del_trailing_separator('A://'), 'A:/'
      assert_equal RefinedPathname.new.del_trailing_separator('A:.'), 'A:.'
      assert_equal RefinedPathname.new.del_trailing_separator('A:./'), 'A:.'
      assert_equal RefinedPathname.new.del_trailing_separator('A:.//'), 'A:.'
    end

    if File.dirname('//') == '//' # DOSISH_UNC
      assert_equal RefinedPathname.new.del_trailing_separator('//'), '//'
      assert_equal RefinedPathname.new.del_trailing_separator('//a'), '//a'
      assert_equal RefinedPathname.new.del_trailing_separator('//a/'), '//a'
      assert_equal RefinedPathname.new.del_trailing_separator('//a//'), '//a'
      assert_equal RefinedPathname.new.del_trailing_separator('//a/b'), '//a/b'
      assert_equal RefinedPathname.new.del_trailing_separator('//a/b/'), '//a/b'
      assert_equal RefinedPathname.new.del_trailing_separator('//a/b//'), '//a/b'
      assert_equal RefinedPathname.new.del_trailing_separator('//a/b/c'), '//a/b/c'
      assert_equal RefinedPathname.new.del_trailing_separator('//a/b/c/'), '//a/b/c'
      assert_equal RefinedPathname.new.del_trailing_separator('//a/b/c//'), '//a/b/c'
    else
      assert_equal RefinedPathname.new.del_trailing_separator('///'), '/'
      assert_equal RefinedPathname.new.del_trailing_separator('///a/'), '///a'
    end

    unless File::ALT_SEPARATOR.nil? # DOSISH
      assert_equal RefinedPathname.new.del_trailing_separator('a\\'), 'a'
      assert_equal RefinedPathname.new.del_trailing_separator("\225\\\\".dup.force_encoding('cp932')), "\225\\".dup.force_encoding('cp932')
      assert_equal RefinedPathname.new.del_trailing_separator("\225\\\\".dup.force_encoding('cp437')), "\225".dup.force_encoding('cp437')
    end
  end
end
