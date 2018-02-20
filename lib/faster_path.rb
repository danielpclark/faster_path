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

  FasterPathname::Public.class_eval do
    private_class_method :absolute?
    private_class_method :add_trailing_separator
    private_class_method :basename
    private_class_method :children # String results
    private_class_method :children_compat # wrap Pathname on each
    private_class_method :chop_basename
    private_class_method :cleanpath_aggressive
    private_class_method :directory?
    private_class_method :dirname
    private_class_method :entries # String results
    private_class_method :entries_compat # wrap Pathname on each
    private_class_method :extname
    private_class_method :has_trailing_separator?
    private_class_method :plus
    private_class_method :relative?
  end

  FasterPathname.class_eval do
    def initialize(arg)
      unless arg.is_a? String
        arg = arg.to_s
      end
      raise(ArgumentError, "null byte found") if arg.include?("\0")
      @path = arg
    end

    # BAD; exposes private methods
    # Need to reprivatize specific methods
    def method_missing(method_name, *a, &b)
      Public.send(method_name, @path, *a, &b)
    end

    def respond_to?(method_name, include_private = false)
      Public.send(:respond_to?, method_name) || super
    end
  end

  def self.rust_arch_bits
    Rust.rust_arch_bits
  end

  def self.ruby_arch_bits
    1.size * 8
  end

  def self.absolute?(pth)
    FasterPathname::Public.send(:absolute?, pth)
  end

  def self.add_trailing_separator(pth)
    FasterPathname::Public.send(:add_trailing_separator, pth)
  end

  def self.blank?(str)
    "#{str}".strip.empty?
  end

  def self.basename(pth, ext="")
    FasterPathname::Public.send(:basename, pth, ext)
  end

  def self.children(pth, with_directory=true)
    FasterPathname::Public.send(:children, pth, with_directory)
  end

  def self.chop_basename(pth)
    result = FasterPathname::Public.send(:chop_basename, pth)
    result unless result.empty?
  end

  def self.cleanpath_aggressive(pth)
    FasterPathname::Public.send(:cleanpath_aggressive, pth)
  end

  def self.directory?(pth)
    FasterPathname::Public.send(:directory?, pth)
  end

  def self.dirname(pth)
    FasterPathname::Public.send(:dirname, pth)
  end

  def self.entries(pth)
    FasterPathname::Public.send(:entries, pth)
  end

  def self.extname(pth)
    FasterPathname::Public.send(:extname, pth)
  end

  def self.has_trailing_separator?(pth)
    FasterPathname::Public.send(:has_trailing_separator?, pth)
  end

  def self.plus(pth, pth2)
    FasterPathname::Public.send(:plus, pth, pth2)
  end

  def self.relative?(pth)
    FasterPathname::Public.send(:relative?, pth)
  end

  module Rust
    extend Fiddle::Importer
    dlload FFI_LIBRARY
    extern 'int rust_arch_bits()'
  end
  private_constant :Rust
end
