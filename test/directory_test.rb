require 'test_helper'

class DirectoryTest < Minitest::Test
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
end
