require 'test_helper'
require "faster_path/optional/refinements"
# At the moment refinements don't allow introspection

class RefinedFile
  using FasterPath::RefineFile
  def dirname(name)
    File.dirname(name)
  end
end

class DirnameTest < Minitest::Test
  def setup
    @refined_file = RefinedFile.new
  end
  
  def test_it_gets_dirnames_correctly
    assert_equal('/home', @refined_file.dirname('/home/jason'))
    assert_equal('/home/jason', @refined_file.dirname('/home/jason/poot.txt'))
    assert_equal('.', @refined_file.dirname('poot.txt'))
    assert_equal('/holy///schnikies', @refined_file.dirname('/holy///schnikies//w00t.bin'))
    assert_equal('.', @refined_file.dirname(''))
    assert_equal('/', @refined_file.dirname('/'))
    assert_equal('/foo', @refined_file.dirname('/foo/foo'))

    assert_equal('/foo', @refined_file.dirname('/foo/bar/'))

    assert_equal(".", @refined_file.dirname("foo"))
    assert_equal("/", @refined_file.dirname("/foo"))
    assert_equal("/foo", @refined_file.dirname("/foo/bar"))
    assert_equal("/foo", @refined_file.dirname("/foo/bar.txt"))
    assert_equal("/foo/bar", @refined_file.dirname("/foo/bar/baz"))

    assert_equal(".", @refined_file.dirname(""))
    assert_equal(".", @refined_file.dirname("."))
    assert_equal(".", @refined_file.dirname("./"))
    assert_equal("./b", @refined_file.dirname("./b/./"))
    assert_equal(".", @refined_file.dirname(".."))
    assert_equal(".", @refined_file.dirname("../"))
    assert_equal("/", @refined_file.dirname("/"))
    assert_equal("/", @refined_file.dirname("/."))
    assert_equal("/", @refined_file.dirname("/foo/"))
    assert_equal("/foo", @refined_file.dirname("/foo/."))
    assert_equal("/foo", @refined_file.dirname("/foo/./"))
    assert_equal("/foo/..", @refined_file.dirname("/foo/../."))
    assert_equal("foo", @refined_file.dirname("foo/../"))    
  end
end 
