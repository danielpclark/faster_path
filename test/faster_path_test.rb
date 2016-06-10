require 'test_helper'

if ENV['TEST_REFINEMENTS'].to_s["true"]
  require "faster_path/optional/refinements"
  require "faster_path/optional/monkeypatches"
end

class RefinedPathname
  using FasterPath::RefinePathname
  def a?(v)
    Pathname.new(v).absolute?
  end
end if ENV["TEST_REFINEMENTS"].to_s["true"]

class FasterPathTest < Minitest::Test
  def test_it_determins_absolute_path
    assert FasterPath.absolute?("/hello")
    refute FasterPath.absolute?("goodbye")
  end

  def test_it_returns_similar_results_to_pathname_absolute?
    ["",".","/",".asdf","/asdf/asdf","/asdf/asdf.asdf","asdf/asdf.asd"].each do |pth|
      assert_equal Pathname.new(pth).absolute?,
                   FasterPath.absolute?(pth)
    end
  end

  def test_it_is_blank?
    assert FasterPath.blank? "  "
    assert FasterPath.blank? ""
  end

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
        refute_equal a, b, "a: #{a} and b: #{b}"
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

  def test_refines_pathname_absolute?
    assert RefinedPathname.new.a?("/")
  end if ENV["TEST_REFINEMENTS"].to_s["true"]

  def test_monkeypatches_pathname_absolute?
    FasterPath.sledgehammer_everything!
    assert Pathname.new("/").absolute?
  end if ENV["TEST_REFINEMENTS"].to_s["true"]
end
