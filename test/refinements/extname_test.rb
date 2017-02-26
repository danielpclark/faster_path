require 'test_helper'

class RefinedFile
  using FasterPath::RefineFile
  def extname(path)
    File.extname(path)
  end
end

class ExtnameTest < Minitest::Test
  # Tests copied from https://searchcode.com/codesearch/view/12785140/
  def test_extname
    assert_equal ".rb", RefinedFile.new.extname("foo.rb")
    assert_equal ".rb", RefinedFile.new.extname("/foo/bar.rb")
    assert_equal ".c", RefinedFile.new.extname("/foo.rb/bar.c")
    assert_equal "", RefinedFile.new.extname("bar")
    assert_equal "", RefinedFile.new.extname(".bashrc")
    assert_equal "", RefinedFile.new.extname("./foo.bar/baz")
    assert_equal ".conf", RefinedFile.new.extname(".app.conf")
  end

  def test_extname_edge_cases
    assert_equal "", RefinedFile.new.extname("")
    assert_equal "", RefinedFile.new.extname(".")
    assert_equal "", RefinedFile.new.extname("/")
    assert_equal "", RefinedFile.new.extname("/.")
    assert_equal "", RefinedFile.new.extname("..")
    assert_equal "", RefinedFile.new.extname("...")
    assert_equal "", RefinedFile.new.extname("....")
    assert_equal "", RefinedFile.new.extname(".foo.")
    assert_equal "", RefinedFile.new.extname("foo.")
  end
end
