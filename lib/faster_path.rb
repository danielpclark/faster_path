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

  FasterPathname.class_eval do
    private :add_trailing_separator
    private :basename
    private :chop_basename
    private :dirname
    private :entries
    private :extname
    private :plus
  end

  def self.rust_arch_bits
    Rust.rust_arch_bits
  end

  def self.ruby_arch_bits
    1.size * 8
  end

  def self.absolute?(pth)
    FasterPathname.allocate.send(:absolute?, pth)
  end

  def self.add_trailing_separator(pth)
    FasterPathname.allocate.send(:add_trailing_separator, pth)
  end

  def self.blank?(str)
    FasterPathname.allocate.send(:blank?, str)
  end

  def self.basename(pth, ext="")
    FasterPathname.allocate.send(:basename, pth, ext)
  end

  def self.chop_basename(pth)
    result = FasterPathname.allocate.send(:chop_basename, pth)
    result unless result.empty?
  end

  def self.directory?(pth)
    FasterPathname.allocate.send(:directory?, pth)
  end

  def self.dirname(pth)
    FasterPathname.allocate.send(:dirname, pth)
  end

  def self.entries(pth)
    FasterPathname.allocate.send(:entries, pth)
  end

  def self.extname(pth)
    FasterPathname.allocate.send(:extname, pth)
  end

  def self.has_trailing_separator?(pth)
    FasterPathname.allocate.send(:has_trailing_separator?, pth)
  end

  def self.plus(pth, pth2)
    FasterPathname.allocate.send(:plus, pth.to_s, pth2.to_s)
  end

  def self.relative?(pth)
    FasterPathname.allocate.send(:relative?, pth)
  end

  module Rust
    extend FFI::Library
    ffi_lib ::FasterPath::FFI_LIBRARY

    attach_function :rust_arch_bits, [], :int32
  end
  private_constant :Rust
end
