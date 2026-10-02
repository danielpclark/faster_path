require 'test_helper'

# Inputs that used to crash the Ruby process (aborts from Rust panics, or
# segfaults) must now return a value or raise a Ruby exception.
class SafetyTest < Minitest::Test
  STRING_METHODS = %i[
    absolute? add_trailing_separator basename chop_basename cleanpath_aggressive
    cleanpath_conservative del_trailing_separator directory? dirname entries
    entries_compat extname has_trailing_separator? plus relative? relative_path_from
    children children_compat
  ].freeze

  class RaisingPath
    def to_path
      raise IOError, "to_path failed"
    end

    def to_s
      raise IOError, "to_s failed"
    end
  end

  class IntegerPath
    def to_path
      42
    end
  end

  def call(method, *args)
    FasterPath.public_send(method, *args)
  rescue StandardError => e
    e
  end

  def test_join_without_arguments_raises_argument_error
    assert_raises(ArgumentError) { FasterPath.join }
  end

  def test_join_under_gc_stress
    GC.stress = true
    result = FasterPath.join(Pathname("a"), "b", Pathname("c"), "d")
    GC.stress = false
    assert_equal Pathname("a/b/c/d"), result
  ensure
    GC.stress = false
  end

  def test_children_and_entries_under_gc_stress
    GC.stress = true
    children = FasterPath.children_compat(__dir__, true)
    entries = FasterPath.entries_compat(__dir__)
    GC.stress = false
    assert_equal Pathname(__dir__).children.sort, children.sort
    assert_equal Pathname(__dir__).entries.sort, entries.sort
  ensure
    GC.stress = false
  end

  def test_exceptions_from_ruby_code_propagate
    assert_raises(IOError) { FasterPath.join("a", RaisingPath.new) }
    assert_raises(IOError) { FasterPath.relative_path_from("a", RaisingPath.new) }
    assert_raises(IOError) { FasterPath.basename(RaisingPath.new) }
  end

  def test_to_path_must_return_a_string
    assert_raises(TypeError) { FasterPath.basename(IntegerPath.new) }
    assert_raises(TypeError) { FasterPath.join("a", IntegerPath.new) }
  end

  def test_to_path_is_used
    assert_equal "b", FasterPath.basename(Pathname("a/b"))
    assert_equal ".rb", FasterPath.extname(Pathname("a/b.rb"))
    assert_equal "a", FasterPath.dirname(Pathname("a/b"))
  end

  def test_add_trailing_separator_requires_a_path
    assert_raises(TypeError) { FasterPath.add_trailing_separator(nil) }
    assert_raises(TypeError) { FasterPath.add_trailing_separator(1) }
    assert_equal "a/", FasterPath.add_trailing_separator("a")
  end

  def test_too_many_arguments_raise_argument_error
    STRING_METHODS.each do |method|
      assert_raises(ArgumentError, method.to_s) { FasterPath.public_send(method, "a", "b", "c") }
    end
  end

  def test_odd_arguments_never_crash
    odd = [nil, 1, :sym, Object.new, [], "a\0b", "\xFF\xFE/\xFD".b, "a/b".encode("UTF-16LE"),
           RaisingPath.new, IntegerPath.new, "", "/", "//", "."]
    STRING_METHODS.each do |method|
      odd.each do |a|
        call(method, a)
        odd.each { |b| call(method, a, b) }
      end
    end
    odd.each { |a| odd.each { |b| call(:join, a, b) } }
  end

  def test_null_bytes_raise_argument_error
    assert_raises(ArgumentError) { FasterPath.basename("a\0b") }
    assert_raises(ArgumentError) { FasterPath.directory?("a\0b") }
    assert_raises(ArgumentError) { FasterPath.entries("a\0b") }
  end

  def test_non_utf8_paths_keep_their_bytes_and_encoding
    path = "\xFF\xFE/\xFD.\xFC".b
    assert_equal "\xFD.\xFC".b, FasterPath.basename(path)
    assert_equal Encoding::ASCII_8BIT, FasterPath.basename(path).encoding
    assert_equal "\xFF\xFE".b, FasterPath.dirname(path)
    assert_equal ".\xFC".b, FasterPath.extname(path)
    assert_equal ["\xFF\xFE/".b, "\xFD.\xFC".b], FasterPath.chop_basename(path)
    assert_equal File.basename(path), FasterPath.basename(path)
    assert_equal File.dirname(path), FasterPath.dirname(path)
    assert_equal File.extname(path), FasterPath.extname(path)
  end

  def test_result_encoding_matches_ruby
    path = "dir/fé.rb".encode("ISO-8859-1")
    assert_equal File.basename(path).encoding, FasterPath.basename(path).encoding
    assert_equal File.dirname(path).encoding, FasterPath.dirname(path).encoding
    assert_equal File.extname(path).encoding, FasterPath.extname(path).encoding
  end

  def test_missing_directories_raise_errno
    missing = File.join(__dir__, "no_such_directory")
    error = assert_raises(Errno::ENOENT) { FasterPath.entries(missing) }
    assert_includes error.message, missing
    assert_raises(Errno::ENOENT) { FasterPath.entries_compat(missing) }
    assert_raises(Errno::ENOENT) { FasterPath.children(missing) }
    assert_raises(Errno::ENOENT) { FasterPath.children_compat(missing) }
    assert_raises(Errno::ENOTDIR) { FasterPath.entries(__FILE__) }
  end

  def test_relative_path_from_errors_match_pathname
    error = assert_raises(ArgumentError) { FasterPath.relative_path_from("a", "/b") }
    pathname_error = assert_raises(ArgumentError) { Pathname("a").relative_path_from(Pathname("/b")) }
    assert_equal pathname_error.message, error.message
  end
end
