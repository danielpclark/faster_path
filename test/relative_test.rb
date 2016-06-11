require 'test_helper'

class RelativeTest < Minitest::Test
  def it_knows_its_relativeness
    refute FasterPath.relative? '/'
    refute FasterPath.relative? '/a'
    refute FasterPath.relative? '/..'
    assert FasterPath.relative? 'a'
    assert FasterPath.relative? 'a/b'

    if File.dirname('//') == '//'
      refute FasterPath.relative? '//'
      refute FasterPath.relative? '//a'
      refute FasterPath.relative? '//a/'
      refute FasterPath.relative? '//a/b'
      refute FasterPath.relative? '//a/b/'
      refute FasterPath.relative? '//a/b/c'
    end
  end
  
  def it_knows_its_relativeness_in_dos_like_drive_letters
    refute FasterPath.relative? 'A:'
    refute FasterPath.relative? 'A:/'
    refute FasterPath.relative? 'A:/a'
  end if File.dirname("A:") == "A:." # DOSISH_DRIVE_LETTER
end
