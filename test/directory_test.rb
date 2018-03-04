require 'test_helper'

class DirectoryTest < Minitest::Test
  def setup
    @dir = Dir.mktmpdir("rubytest-file")
    File.chown(-1, Process.gid, @dir)
  end

  def teardown
    GC.start
    FileUtils.remove_entry_secure @dir
  end

  def test_nil_for_directory?
    refute FasterPath.directory? nil
  end

  def test_of_ayes
    result_pair = lambda do |str|
      [
        Pathname.new(str).send(:directory?),
        FasterPath.directory?(str)
      ]
    end
    assert_equal( *result_pair.("aa/a//a")                 )
    assert_equal( *result_pair.("/aaa/a//a")               )
    assert_equal( *result_pair.("/aaa/a//a/a")             )
    assert_equal( *result_pair.("/aaa/a//a/a")             )
    assert_equal( *result_pair.("a//aaa/a//a/a")           )
    assert_equal( *result_pair.("a//aaa/a//a/aaa")         )
    assert_equal( *result_pair.("/aaa/a//a/aaa/a")         )
    assert_equal( *result_pair.("a//aaa/a//a/aaa/a")       )
    assert_equal( *result_pair.("a//aaa/a//a/aaa////")     )
    assert_equal( *result_pair.("a/a//aaa/a//a/aaa/a")     )
    assert_equal( *result_pair.("////a//aaa/a//a/aaa/a")   )
    assert_equal( *result_pair.("////a//aaa/a//a/aaa////") )
  end

  def test_of_bees
    result_pair = lambda do |str|
      [
        Pathname.new(str).send(:directory?),
        FasterPath.directory?(str)
      ]
    end
    assert_equal( *result_pair.(".")                        )
    assert_equal( *result_pair.(".././")                    )
    assert_equal( *result_pair.(".///..")                   )
    assert_equal( *result_pair.("/././/")                   )
    assert_equal( *result_pair.("//../././")                )
    assert_equal( *result_pair.(".///.../..")               )
    assert_equal( *result_pair.("/././/.//.")               )
    assert_equal( *result_pair.("/...//../././")            )
    assert_equal( *result_pair.("/..///.../..//")           )
    assert_equal( *result_pair.("/./././/.//...")           )
    assert_equal( *result_pair.("/...//.././././/.")        )
    assert_equal( *result_pair.("./../..///.../..//")       )
    assert_equal( *result_pair.("///././././/.//...")       )
    assert_equal( *result_pair.("./../..///.../..//././")   )
    assert_equal( *result_pair.("///././././/.//....///")   )
  end

  def test_of_seas
    result_pair = lambda do |str|
      [
        Pathname.new(str).send(:directory?),
        FasterPath.directory?(str)
      ]
    end
    assert_equal( *result_pair.("http://www.example.com")   )
    assert_equal( *result_pair.("foor for thought")         )
    assert_equal( *result_pair.("2gb63b@%TY25GHawefb3/g3qb"))
  end

  def test_directory?
    with_tmpchdir('rubytest-pathname') do |_dir|
      open("f", "w") {|f| f.write "abc" }
      assert_equal(false, FasterPath.directory?("f"))
      Dir.mkdir("d")
      assert_equal(true, FasterPath.directory?("d"))
    end
  end

  def test_directory_p
    assert FasterPath.directory?(@dir)
    refute FasterPath.directory?(@dir+"/...")
    refute FasterPath.directory?(regular_file)
    refute FasterPath.directory?(utf8_file)
    refute FasterPath.directory?(nofile)
  end

  def test_stat_directory_p
    assert_predicate(File::Stat.new(@dir), :directory?)
    refute_predicate(File::Stat.new(regular_file), :directory?)
  end
end
