require 'test_helper'

class EntriesTest < Minitest::Test
  def test_pathnames_existing_behavior
    assert_kind_of Pathname, Pathname.new('.').entries.first
  end

  def test_entries_compat_returns_pathname_objects
    assert_kind_of Pathname, FasterPathname::Public.allocate.send(:entries_compat, '.').first
  end

  def test_entries_returns_string_objects
    assert_kind_of String, FasterPath.entries('.').first
  end

  def test_it_returns_similar_results_to_pathname_entries_as_strings
    ['.', 'lib', 'src'].each do |pth|
      assert_equal Pathname.new(pth).entries.sort,
        FasterPathname::Public.allocate.send(:entries_compat, pth).sort
    end
  end
end
