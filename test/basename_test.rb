require 'test_helper'

class BasenameTest < Minitest::Test
  def setup
    @dir = Dir.mktmpdir("rubytest-file")
    File.chown(-1, Process.gid, @dir)
  end

  def teardown
    GC.start
    FileUtils.remove_entry_secure @dir
  end

  # Tests copied from https://searchcode.com/codesearch/view/12785140/
  def test_it_creates_basename_correctly
    assert_equal FasterPath.basename('/home/gumby/work/ruby.rb'),        'ruby.rb'
    assert_equal FasterPath.basename('/home/gumby/work/ruby.rb', '.rb'), 'ruby'
    assert_equal FasterPath.basename('/home/gumby/work/ruby.rb', '.*'),  'ruby'
    assert_equal FasterPath.basename('ruby.rb',  '.*'),  'ruby'
    assert_equal FasterPath.basename('/ruby.rb', '.*'),  'ruby'
    assert_equal FasterPath.basename('ruby.rbx', '.*'),  'ruby'
    assert_equal FasterPath.basename('ruby.rbx', '.rb'), 'ruby.rbx'
    assert_equal FasterPath.basename('ruby.rb', ''),      'ruby.rb'
    assert_equal FasterPath.basename('ruby.rbx', '.rb*'), 'ruby.rbx'
    assert_equal FasterPath.basename('ruby.rbx'), 'ruby.rbx'

    # Try some extensions w/o a '.'
    assert_equal FasterPath.basename('ruby.rbx', 'rbx'), 'ruby.'
    assert_equal FasterPath.basename('ruby.rbx', 'x'),   'ruby.rb'
    assert_equal FasterPath.basename('ruby.rbx', '*'),   'ruby.rbx'

    # A couple of regressions:
    assert_equal FasterPath.basename('', ''),           ''
    assert_equal FasterPath.basename('/'),              '/'
    assert_equal FasterPath.basename('//'),             '/'
    assert_equal FasterPath.basename('//dir///base//'), 'base'
    assert_equal FasterPath.basename('.x', '.x'), '.x'

    # returns the basename for unix suffix
    assert_equal FasterPath.basename("bar.c", ".c"), "bar"
    assert_equal FasterPath.basename("bar.txt", ".txt"), "bar"
    assert_equal FasterPath.basename("/bar.txt", ".txt"), "bar"
    assert_equal FasterPath.basename("/foo/bar.txt", ".txt"), "bar"
    assert_equal FasterPath.basename("bar.txt", ".exe"), "bar.txt"
    assert_equal FasterPath.basename("bar.txt.exe", ".exe"), "bar.txt"
    assert_equal FasterPath.basename("bar.txt.exe", ".txt"), "bar.txt.exe"
    assert_equal FasterPath.basename("bar.txt", ".*"), "bar"
    assert_equal FasterPath.basename("bar.txt.exe", ".*"), "bar.txt"
    assert_equal FasterPath.basename("bar.txt.exe", ".txt.exe"), "bar"
  end

  def test_it_does_the_same_as_file_basename
    assert_equal FasterPath.basename('/home/gumby/work/ruby.rb'),        'ruby.rb'
    assert_equal FasterPath.basename('/home/gumby/work/ruby.rb', '.rb'), 'ruby'
    assert_equal FasterPath.basename('/home/gumby/work/ruby.rb', '.*'),  'ruby'
    assert_equal FasterPath.basename('ruby.rb',  '.*'),  'ruby'
    assert_equal FasterPath.basename('/ruby.rb', '.*'),  'ruby'
    assert_equal FasterPath.basename('ruby.rbx', '.*'),  'ruby'
    assert_equal FasterPath.basename('ruby.rbx', '.rb'), 'ruby.rbx'
    assert_equal FasterPath.basename('ruby.rb', ''),      'ruby.rb'
    assert_equal FasterPath.basename('ruby.rbx', '.rb*'), 'ruby.rbx'
    assert_equal FasterPath.basename('ruby.rbx'), 'ruby.rbx'

    # Try some extensions w/o a '.'
    assert_equal FasterPath.basename('ruby.rbx', 'rbx'), 'ruby.'
    assert_equal FasterPath.basename('ruby.rbx', 'x'),   'ruby.rb'
    assert_equal FasterPath.basename('ruby.rbx', '*'),   'ruby.rbx'

    # A couple of regressions:
    assert_equal FasterPath.basename('', ''),           ''
    assert_equal FasterPath.basename('/'),              '/'
    assert_equal FasterPath.basename('//'),             '/'
    assert_equal FasterPath.basename('//dir///base//'), 'base'
  end

  def test_basename_official
    assert_equal(FasterPath.basename(regular_file).sub(/\.test$/, ""), FasterPath.basename(regular_file, ".test"))
    assert_equal(FasterPath.basename(utf8_file).sub(/\.test$/, ""), FasterPath.basename(utf8_file, ".test"))
    assert_equal("", s = FasterPath.basename(""))
    refute_predicate(s, :frozen?, '[ruby-core:24199]')
    assert_equal("foo", s = FasterPath.basename("foo"))
    refute_predicate(s, :frozen?, '[ruby-core:24199]')
    assert_equal("foo", FasterPath.basename("foo", ".ext"))
    assert_equal("foo", FasterPath.basename("foo.ext", ".ext"))
    assert_equal("foo", FasterPath.basename("foo.ext", ".*"))
  end

  def test_basename_official_ntfs
    if NTFS
      [regular_file, utf8_file].each do |file|
        basename = FasterPath.basename(file)
        assert_equal(basename, FasterPath.basename(file + " "))
        assert_equal(basename, FasterPath.basename(file + "."))
        assert_equal(basename, FasterPath.basename(file + "::$DATA"))
        basename.chomp!(".test")
        assert_equal(basename, FasterPath.basename(file + " ", ".test"))
        assert_equal(basename, FasterPath.basename(file + ".", ".test"))
        assert_equal(basename, FasterPath.basename(file + "::$DATA", ".test"))
        assert_equal(basename, FasterPath.basename(file + " ", ".*"))
        assert_equal(basename, FasterPath.basename(file + ".", ".*"))
        assert_equal(basename, FasterPath.basename(file + "::$DATA", ".*"))
      end
    else
      [regular_file, utf8_file].each do |file|
        basename = FasterPath.basename(file)
        assert_equal(basename + " ", FasterPath.basename(file + " "))
        assert_equal(basename + ".", FasterPath.basename(file + "."))
        assert_equal(basename + "::$DATA", FasterPath.basename(file + "::$DATA"))
        assert_equal(basename + " ", FasterPath.basename(file + " ", ".test"))
        assert_equal(basename + ".", FasterPath.basename(file + ".", ".test"))
        assert_equal(basename + "::$DATA", FasterPath.basename(file + "::$DATA", ".test"))
        assert_equal(basename, FasterPath.basename(file + ".", ".*"))
        basename.chomp!(".test")
        assert_equal(basename, FasterPath.basename(file + " ", ".*"))
        assert_equal(basename, FasterPath.basename(file + "::$DATA", ".*"))
      end
    end
  end

  def test_basename_official_encoding
    if File::ALT_SEPARATOR == '\\'
      a = "foo/\225\\\\"
      [%W[cp437 \225], %W[cp932 \225\\]].each do |cp, expected|
        assert_equal(expected.force_encoding(cp), FasterPath.basename(a.dup.force_encoding(cp)), cp)
      end
    end
    assert_incompatible_encoding {|d| FasterPath.basename(d)}
    assert_incompatible_encoding {|d| FasterPath.basename(d, ".*")}
    assert_raises(Encoding::CompatibilityError) {FasterPath.basename("foo.ext", ".*".encode("utf-16le"))}
    s = "foo\x93_a".force_encoding("cp932")
    assert_equal(s, FasterPath.basename(s, "_a"))
    s = "\u4032.\u3024"
    assert_equal(s, FasterPath.basename(s, ".\x95\\".force_encoding("cp932")))
  end if ENV['ENCODING'].to_s['true']
end
