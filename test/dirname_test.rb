require 'test_helper'

class DirnameTest < Minitest::Test
  def test_it_returns_all_the_components_of_filename_except_the_last_one
    assert_equal('/home', FasterPath.dirname('/home/jason'))
    assert_equal('/home/jason', FasterPath.dirname('/home/jason/poot.txt'))
    assert_equal('.', FasterPath.dirname('poot.txt'))
    assert_equal('/holy///schnikies', FasterPath.dirname('/holy///schnikies//w00t.bin'))
    assert_equal('.', FasterPath.dirname(''))
    assert_equal('/', FasterPath.dirname('/'))
    assert_equal('/foo', FasterPath.dirname('/foo/foo'))
  end

  def test_it_returns_a_string
    assert_kind_of(String, FasterPath.dirname("foo"))
  end

  def test_it_does_not_modify_its_argument
    x = "/usr/bin"
    FasterPath.dirname(x)
    assert_equal("/usr/bin", x)
  end

  def test_it_ignores_a_trailing_slash
    assert_equal('/foo', FasterPath.dirname('/foo/bar/'))
  end

  def test_it_returns_the_return_all_the_components_of_filename_except_the_last_one_unix_format
    assert_equal(".", FasterPath.dirname("foo"))
    assert_equal("/", FasterPath.dirname("/foo"))
    assert_equal("/foo", FasterPath.dirname("/foo/bar"))
    assert_equal("/foo", FasterPath.dirname("/foo/bar.txt"))
    assert_equal("/foo/bar", FasterPath.dirname("/foo/bar/baz"))
  end

  def test_it_returns_the_return_all_the_components_of_filename_except_the_last_one_edge_cases
    assert_equal(".", FasterPath.dirname(""))
    assert_equal(".", FasterPath.dirname("."))
    assert_equal(".", FasterPath.dirname("./"))
    assert_equal("./b", FasterPath.dirname("./b/./"))
    assert_equal(".", FasterPath.dirname(".."))
    assert_equal(".", FasterPath.dirname("../"))
    assert_equal("/", FasterPath.dirname("/"))
    assert_equal("/", FasterPath.dirname("/."))
    assert_equal("/", FasterPath.dirname("/foo/"))
    assert_equal("/foo", FasterPath.dirname("/foo/."))
    assert_equal("/foo", FasterPath.dirname("/foo/./"))
    assert_equal("/foo/..", FasterPath.dirname("/foo/../."))
    assert_equal("foo", FasterPath.dirname("foo/../"))
  end

  def test_dirname_official
    @dir = Dir.mktmpdir("rubytest-file")
    File.chown(-1, Process.gid, @dir)

    assert_equal(@dir, FasterPath.dirname(regular_file))
    assert_equal(@dir, FasterPath.dirname(utf8_file))
    assert_equal(".", FasterPath.dirname(""))
    assert_incompatible_encoding {|d| FasterPath.dirname(d)} if ENV['ENCODING'].to_s['true']
    if File::ALT_SEPARATOR == '\\'
      a = "\225\\\\foo"
      [%W[cp437 \225], %W[cp932 \225\\]].each do |cp, expected|
        assert_equal(expected.force_encoding(cp), FasterPath.dirname(a.dup.force_encoding(cp)), cp)
      end
    end

    GC.start
    FileUtils.remove_entry_secure @dir
  end
end
