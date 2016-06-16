require 'test_helper'

class ChopBasenameTest < Minitest::Test
  def test_it_chops_basename_
    result = FasterPath.chop_basename("/hello/world.txt")
    assert_equal result.length, 2
    assert_equal result.last, "world.txt"
    assert_equal result.first, "/hello/"
    result = FasterPath.chop_basename("world.txt")
    assert_equal result.length, 2
    assert_equal result.last, "world.txt"
    assert_equal result.first, ""
    result = FasterPath.chop_basename("")
    refute result
    assert result.nil?
  end

  def test_it_returns_similar_results_to_pathname_chop_basename
    ["hello/world/123.txt","/hello/world.txt","hello.txt","h"].
      each do |str|
      pcb = Pathname.new("").send :chop_basename, str
      fpcb = FasterPath.chop_basename str
      Array(pcb).zip(Array(fpcb)).each do |a,b|
        assert_equal a, b, "a: #{a} and b: #{b}"
      end
    end
  end

  def test_it_fixes_blank_results_to_pathname_chop_basename
    # THIS IS THE ONLY BREAKING BEHAVIOR ON Pathname#chop_basename
    # THIS IS THE INTENTION OF Pathname's DESIGN FOR NON-PATHS
    [" ", "   "].
      each do |str|
      pcb = Pathname.new("").send :chop_basename, str
      fpcb = FasterPath.chop_basename str
      Array(pcb).zip(Array(fpcb)).each do |a,b|
        if a 
          refute_equal a, b, "a: #{a} and b: #{b}"
        else
          assert_nil a
          assert_nil b
        end
      end
    end
  end

  def test_it_returns_similar_results_to_pathname_chop_basename_for_slash
    ["",File::SEPARATOR, File::SEPARATOR*2].
      each do |str|
      pcb = Pathname.new("").send :chop_basename, str
      fpcb = FasterPath.chop_basename str
      Array(pcb).zip(Array(fpcb)).each do |a,b|
        assert_equal a, b, "a: #{a} and b: #{b}"
        assert_nil a
        assert_nil b
      end
    end
  end

  def test_it_returns_similar_results_to_pathname_chop_basename_for_dot_files
    [".hello/world/123.txt","./hello/world.txt",".hello.txt",".h",
     ".hello/.world/.123.txt","./hello/.world.txt",".hello.txt","../.h"].
      each do |str|
      pcb = Pathname.new("").send :chop_basename, str
      fpcb = FasterPath.chop_basename str
      Array(pcb).zip(Array(fpcb)).each do |a,b|
        assert_equal a, b, "a: #{a} and b: #{b}"
      end
    end
  end

  def test_of_As
    result_pair = ->str{
      [
        Pathname.new("").send(:chop_basename, str),
        FasterPath.chop_basename(str)
      ]
    }
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


  def test_of_Bs
    result_pair = ->str{
      [
        Pathname.new("").send(:chop_basename, str),
        FasterPath.chop_basename(str)
      ]
    }
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


  def test_of_Cs
    result_pair = ->str{
      [
        Pathname.new("").send(:chop_basename, str),
        FasterPath.chop_basename(str)
      ]
    }
    assert_equal( *result_pair.("http://www.example.com")   )
    assert_equal( *result_pair.("foor for thought")         )
    assert_equal( *result_pair.("2gb63b@%TY25GHawefb3/g3qb"))
  end
end
