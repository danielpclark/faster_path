require 'test_helper'

class BasenameTest < Minitest::Test
  # Tests adapted (MSpec -> Minitest) from:
  # https://github.com/ruby/spec/blob/8190a7c915fbcf4776b99390418fe63fc6086c02/core/file/basename_spec.rb

  def test_returns_the_basename_of_a_path
      assert_basename_eq 'test.txt', '/Some/path/to/test.txt'
      assert_basename_eq 'tmp', File.join('/tmp')
      assert_basename_eq 'b', File.join(*%w( g f d s a b))
      assert_basename_eq 'tmp', '/tmp', '.*'
      assert_basename_eq 'tmp', '/tmp', '.c'
      assert_basename_eq 'tmp', '/tmp.c', '.c'
      assert_basename_eq 'tmp', '/tmp.c', '.*'
      assert_basename_eq 'tmp.c', '/tmp.c', '.?'
      assert_basename_eq 'tmp', '/tmp.cpp', '.*'
      assert_basename_eq 'tmp.cpp', '/tmp.cpp', '.???'
      assert_basename_eq 'tmp.o', '/tmp.o', '.c'
      assert_basename_eq 'tmp', File.join('/tmp/')
      assert_basename_eq '/', '/'
      assert_basename_eq '/', '//'
      assert_basename_eq 'base', 'dir///base', '.*'
      assert_basename_eq 'base', 'dir///base', '.c'
      assert_basename_eq 'base', 'dir///base.c', '.c'
      assert_basename_eq 'base', 'dir///base.c', '.*'
      assert_basename_eq 'base.o', 'dir///base.o', '.c'
      assert_basename_eq 'base', 'dir///base///'
      assert_basename_eq 'base', 'dir//base/', '.*'
      assert_basename_eq 'base', 'dir//base/', '.c'
      assert_basename_eq 'base', 'dir//base.c/', '.c'
      assert_basename_eq 'base', 'dir//base.c/', '.*'
    end

  def test_returns_the_last_component_of_the_filename
    assert_basename_eq 'a', 'a'
    assert_basename_eq 'a', '/a'
    assert_basename_eq 'b', '/a/b'
    assert_basename_eq 'bag', '/ab/ba/bag'
    assert_basename_eq 'bag.txt', '/ab/ba/bag.txt'
    assert_basename_eq '/', '/'
    assert_basename_eq 'baz', '/foo/bar/baz.rb', '.rb'
    assert_basename_eq 'ba', 'baz.rb', 'z.rb'
  end

  def test_returns_a_string
    assert_kind_of File.basename('foo'), String
    assert_kind_of FasterPath.basename('foo'), String
  end

  def test_returns_the_basename_for_unix_format
    assert_basename_eq 'bar', '/foo/bar'
    assert_basename_eq 'bar.txt', '/foo/bar.txt'
    assert_basename_eq 'bar.c', 'bar.c'
    assert_basename_eq 'bar', '/bar'
    assert_basename_eq 'bar', '/bar/'

    # Considered UNC paths on Windows
    if platform_windows?
      assert_basename_eq 'foo', 'baz//foo'
      assert_basename_eq 'baz', '//foo/bar/baz'
    end
  end

  def test_returns_the_basename_for_edge_cases
    assert_basename_eq '', ''
    assert_basename_eq '.', '.'
    assert_basename_eq '..', '..'
    unless platform_windows?
      assert_basename_eq 'foo', '//foo/'
      assert_basename_eq 'foo', '//foo//'
    end
    assert_basename_eq 'foo', 'foo/'
  end

  def test_ignores_a_trailing_directory_separator
    assert_basename_eq 'foo', 'foo.rb/', '.rb'
    assert_basename_eq 'bar', 'bar.rb///', '.*'
  end

  def test_returns_the_basename_for_unix_suffix
    assert_basename_eq 'bar', 'bar.c', '.c'
    assert_basename_eq 'bar', 'bar.txt', '.txt'
    assert_basename_eq 'bar', '/bar.txt', '.txt'
    assert_basename_eq 'bar', '/foo/bar.txt', '.txt'
    assert_basename_eq 'bar.txt', 'bar.txt', '.exe'
    assert_basename_eq 'bar.txt', 'bar.txt.exe', '.exe'
    assert_basename_eq 'bar.txt.exe', 'bar.txt.exe', '.txt'
    assert_basename_eq 'bar', 'bar.txt', '.*'
    assert_basename_eq 'bar.txt', 'bar.txt.exe', '.*'
    assert_basename_eq 'bar', 'bar.txt.exe', '.txt.exe'
    assert_basename_eq 'bar', 'bar.txt.exe', '.txt.*'
  end

  def test_raises_a_type_error_if_the_arguments_are_not_string_types
    assert_raises(TypeError) { File.basename(nil)          }
    assert_raises(TypeError) { File.basename(1)            }
    assert_raises(TypeError) { File.basename('bar.txt', 1) }
    assert_raises(TypeError) { File.basename(true)         }

    assert_raises(TypeError) { FasterPath.basename(nil)          }
    assert_raises(TypeError) { FasterPath.basename(1)            }
    assert_raises(TypeError) { FasterPath.basename('bar.txt', 1) }
    assert_raises(TypeError) { FasterPath.basename(true)         }
  end

  class ToPathMock < Struct.new(:to_path); end

  def test_accepts_an_object_that_has_a_to_path_method
    assert_basename_eq('foo.txt', ToPathMock.new('foo.txt'))
  end

  def test_raises_an_argument_error_if_passed_more_than_two_arguments
    assert_raises(ArgumentError) { File.basename('bar.txt', '.txt', '.txt') }
    assert_raises(ArgumentError) { FasterPath.basename('bar.txt', '.txt', '.txt') }
  end

  # specific to MS Windows
  if platform_windows?
    def test_returns_the_basename_for_windows
      assert_basename_eq 'baz.txt', 'C:\\foo\\bar\\baz.txt'
      assert_basename_eq 'bar', 'C:\\foo\\bar'
      assert_basename_eq 'bar', 'C:\\foo\\bar\\'
      assert_basename_eq 'foo', 'C:\\foo'
      assert_basename_eq '\\', 'C:\\'
    end

    def test_returns_basename_windows_unc
      assert_basename_eq 'baz.txt', '\\\\foo\\bar\\baz.txt'
      assert_basename_eq 'baz', '\\\\foo\\bar\\baz'
    end

    def test_returns_basename_windows_forward_slash
      assert_basename_eq '/', 'C:/'
      assert_basename_eq 'foo', 'C:/foo'
      assert_basename_eq 'bar', 'C:/foo/bar'
      assert_basename_eq 'bar', 'C:/foo/bar/'
      assert_basename_eq 'bar', 'C:/foo/bar//'
    end

    def test_returns_basename_with_windows_suffix
      assert_basename_eq 'bar', 'c:\\bar.txt', '.txt'
      assert_basename_eq 'bar', 'c:\\foo\\bar.txt', '.txt'
      assert_basename_eq 'bar.txt', 'c:\\bar.txt', '.exe'
      assert_basename_eq 'bar.txt', 'c:\\bar.txt.exe', '.exe'
      assert_basename_eq 'bar.txt.exe', 'c:\\bar.txt.exe', '.txt'
      assert_basename_eq 'bar', 'c:\\bar.txt', '.*'
      assert_basename_eq 'bar.txt', 'c:\\bar.txt.exe', '.*'
    end
  end

  private

  def assert_basename_eq(expected, *basename_args)
    assert_equal expected, File.basename(*basename_args), 'Unexpected result from Ruby'
    assert_equal expected, FasterPath.basename(*basename_args), 'Unexpected result from Rust'
  end
end
