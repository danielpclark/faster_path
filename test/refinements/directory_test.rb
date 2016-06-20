require 'test_helper'
require 'faster_path/optional/refinements'

class RefinedPathname
  using FasterPath::RefinePathname
  def directory?(v)
    Pathname.new(v).directory?
  end
end

class DirectoryRefinementTest < Minitest::Test
  def test_refines_pathname_directory?
    assert RefinedPathname.new.directory?('/')
  end

  def test_nil_behaves_the_same
    assert_raises(TypeError) { RefinedPathname.new.directory?(nil) }
    assert_raises(TypeError) { Pathname.new(nil).directory? }
  end
end
