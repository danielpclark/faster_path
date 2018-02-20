require "faster_path/version"
require 'pathname'
require 'faster_path/platform'
require 'faster_path/asset_resolution'
require 'fiddle'
require 'fiddle/import'

module FasterPath
  FFI_LIBRARY = FasterPath::Platform.ffi_library()

  Fiddle::Function.
    new(Fiddle.dlopen(FFI_LIBRARY)['Init_faster_pathname'], [], Fiddle::TYPE_VOIDP).
    call

  Public.class_eval do
    private_class_method :basename
    private_class_method :children
    private_class_method :children_compat
    private_class_method :chop_basename
    private_class_method :entries
    private_class_method :entries_compat
  end

  def self.rust_arch_bits
    Rust.rust_arch_bits
  end

  def self.ruby_arch_bits
    1.size * 8
  end

  def self.blank?(str)
    "#{str}".strip.empty?
  end

  def self.basename(pth, ext="")
    Public.send(:basename, pth, ext)
  end

  def self.children(pth, with_directory=true)
    result = Public.send(:children, pth, with_directory)
    return result if result
    raise Errno::NOENT, "No such file or directory @ dir_initialize - #{pth}"
  end

  def self.children_compat(pth, with_directory=true)
    result = Public.send(:children_compat, pth, with_directory)
    return result if result
    raise Errno::NOENT, "No such file or directory @ dir_initialize - #{pth}"
  end

  def self.chop_basename(pth)
    result = Public.send(:chop_basename, pth)
    result unless result.empty?
  end

  def self.entries(pth)
    result = Public.send(:entries, pth)
    return result if result
    raise Errno::NOENT, "No such file or directory @ dir_initialize - #{pth}"
  end

  def self.entries_compat(pth)
    result = Public.send(:entries_compat, pth)
    return result if result
    raise Errno::NOENT, "No such file or directory @ dir_initialize - #{pth}"
  end

  module Rust
    extend Fiddle::Importer
    dlload FFI_LIBRARY
    extern 'int rust_arch_bits()'
  end
  private_constant :Rust
end
