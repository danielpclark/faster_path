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
    private :chop_basename
    private :plus
  end

  require_relative 'ffi/basename'
  require_relative 'ffi/dirname'
  require_relative 'ffi/extname'

  def self.rust_arch_bits
    Rust.rust_arch_bits
  end

  def self.ruby_arch_bits
    1.size * 8
  end

  def self.absolute?(pth)
    FasterPathname.allocate.send(:absolute?, pth.to_s)
  end

  def self.directory?(pth)
    Rust.is_directory(pth)
  end

  def self.relative?(pth)
    Rust.is_relative(pth)
  end

  def self.dirname(pth)
    Dirname::Binding.dirname(pth).to_s
  end

  def self.chop_basename(pth)
    result = FasterPathname.allocate.send(:chop_basename, pth)
    result unless result.empty?
  end

  def self.blank?(str)
    Rust.is_blank(str)
  end

  def self.basename(pth, ext="")
    Basename::Binding.basename(pth, ext).to_s
  end

  def self.plus(pth, pth2)
    FasterPathname.allocate.send(:plus, pth.to_s, pth2.to_s)
  end

  def self.add_trailing_separator(pth)
    FasterPathname.allocate.send(:add_trailing_separator, pth.to_s)
  end

  def self.has_trailing_separator?(pth)
    Rust.has_trailing_separator(pth)
  end

  def self.extname(pth)
    Extname::Binding.extname(pth).to_s
  end

  def self.entries(pth)
    FasterPathname.allocate.send(:entries, pth)
  end

  module Rust
    extend FFI::Library
    ffi_lib ::FasterPath::FFI_LIBRARY

    attach_function :rust_arch_bits, [], :int32
    attach_function :is_directory, [ :string ], :bool
    attach_function :is_relative, [ :string ], :bool
    attach_function :is_blank, [ :string ], :bool
    attach_function :both_are_blank, [ :string, :string ], :bool
    attach_function :has_trailing_separator, [ :string ], :bool
  end
  private_constant :Rust
end
