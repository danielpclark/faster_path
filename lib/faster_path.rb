require "faster_path/version"
require 'pathname'
require "ffi"
require 'faster_path/asset_resolution'

module FasterPath
  FFI_LIBRARY = begin
    prefix = Gem.win_platform? ? "" : "lib"
    "#{File.expand_path("../target/release/", __dir__)}/#{prefix}faster_path.#{FFI::Platform::LIBSUFFIX}"
  end
  require 'fiddle'
  library = Fiddle.dlopen(FFI_LIBRARY)
  Fiddle::Function.
    new(library['Init_faster_pathname'], [], Fiddle::TYPE_VOIDP).
    call

  FasterPathname::Public.class_eval do
    private :absolute?
    private :add_trailing_separator
    private :basename
    private :children
    private :children_compat
    private :chop_basename
    private :directory?
    private :dirname
    private :entries
    private :entries_compat
    private :extname
    private :has_trailing_separator?
    private :plus
    private :relative?
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
    FasterPathname::Public.allocate.send(:absolute?, pth)
  end

  def self.add_trailing_separator(pth)
    FasterPathname::Public.allocate.send(:add_trailing_separator, pth)
  end

  def self.blank?(str)
    "#{str}".strip.empty?
  end

  def self.basename(pth, ext="")
    FasterPathname::Public.allocate.send(:basename, pth, ext)
  end

  def self.children(pth, with_directory=true)
    FasterPathname::Public.allocate.send(:children, pth, with_directory)
  end

  def self.chop_basename(pth)
    result = FasterPathname::Public.allocate.send(:chop_basename, pth)
    result unless result.empty?
  end

  def self.directory?(pth)
    FasterPathname::Public.allocate.send(:directory?, pth)
  end

  def self.dirname(pth)
    FasterPathname::Public.allocate.send(:dirname, pth)
  end

  def self.entries(pth)
    FasterPathname::Public.allocate.send(:entries, pth)
  end

  def self.extname(pth)
    FasterPathname::Public.allocate.send(:extname, pth)
  end

  def self.has_trailing_separator?(pth)
    FasterPathname::Public.allocate.send(:has_trailing_separator?, pth)
  end

  def self.plus(pth, pth2)
    FasterPathname::Public.allocate.send(:plus, pth, pth2)
  end

  def self.relative?(pth)
    FasterPathname::Public.allocate.send(:relative?, pth)
  end

  module Rust
    extend FFI::Library
    ffi_lib ::FasterPath::FFI_LIBRARY

    attach_function :rust_arch_bits, [], :int32
  end
  private_constant :Rust
end
