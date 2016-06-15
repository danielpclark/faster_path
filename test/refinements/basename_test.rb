require 'test_helper'

class RefinedFile
  using FasterPath::RefineFile
  def basename(*args)
    File.basename(*args)
  end 
end if FasterPath.respond_to? :basename

class BasenameTest < Minitest::Test
  # Tests copied from https://searchcode.com/codesearch/view/12785140/
  def test_it_creates_basename_correctly
    # Tests for basename
    assert_equal(RefinedFile.new.basename('/home/gumby/work/ruby.rb'),        'ruby.rb', "Pickaxe A")
    assert_equal(RefinedFile.new.basename('/home/gumby/work/ruby.rb', '.rb'), 'ruby',    "Pickaxe B")
    assert_equal(RefinedFile.new.basename('/home/gumby/work/ruby.rb', '.*'),  'ruby',    "Pickaxe C")
    assert_equal(RefinedFile.new.basename('ruby.rb',  '.*'),  'ruby',      "GemStone A")
    assert_equal(RefinedFile.new.basename('/ruby.rb', '.*'),  'ruby',      "GemStone B")
    assert_equal(RefinedFile.new.basename('ruby.rbx', '.*'),  'ruby',      "GemStone C")
    assert_equal(RefinedFile.new.basename('ruby.rbx', '.rb'), 'ruby.rbx',  "GemStone D")
    assert_equal(RefinedFile.new.basename('ruby.rb', ''),      'ruby.rb',  "GemStone E")
    assert_equal(RefinedFile.new.basename('ruby.rbx', '.rb*'), 'ruby.rbx', "GemStone F")
    assert_equal(RefinedFile.new.basename('ruby.rbx'), 'ruby.rbx',         "GemStone G")
    
    # Try some extensions w/o a '.'
    assert_equal(RefinedFile.new.basename('ruby.rbx', 'rbx'), 'ruby.',     "GemStone H")
    assert_equal(RefinedFile.new.basename('ruby.rbx', 'x'),   'ruby.rb',   "GemStone I")
    assert_equal(RefinedFile.new.basename('ruby.rbx', '*'),   'ruby.rbx',  "GemStone J")
    
    # A couple of regressions:
    assert_equal(RefinedFile.new.basename('', ''),           '',     'File.basename regression 1')
    assert_equal(RefinedFile.new.basename('/'),              '/',    'File.basename regression 2')
    assert_equal(RefinedFile.new.basename('//'),             '/',    'File.basename regression 3')
    assert_equal(RefinedFile.new.basename('//dir///base//'), 'base', 'File.basename regression 4')
  end
end if FasterPath.respond_to? :basename
