require "faster_path/version"
require 'pathname'
require "ffi"
require 'faster_path/asset_resolution'

module FasterPath
  FFI_LIBRARY = begin
    prefix = Gem.win_platform? ? "" : "lib"
    "#{File.expand_path("../target/release/", __dir__)}/#{prefix}faster_path.#{FFI::Platform::LIBSUFFIX}"
  end
  require_relative 'ffi/add_trailing_separator'
  require_relative 'ffi/basename'
  # require_relative 'ffi/chop_basename'
  require_relative 'ffi/dirname'
  require_relative 'ffi/extname'
  require_relative 'ffi/plus'

  def self.rust_arch_bits
    Rust.rust_arch_bits
  end

  def self.ruby_arch_bits
    1.size * 8
  end

  # Spec to Pathname#absolute?
  def self.absolute?(pth)
    Rust.is_absolute(pth)
  end

  # Spec to Pathname#directory?
  def self.directory?(pth)
    Rust.is_directory(pth)
  end

  # Spec to Pathname#relative?
  def self.relative?(pth)
    Rust.is_relative(pth)
  end

  def self.dirname(pth)
    Dirname::Binding.dirname(pth).to_s
  end

  # Spec to Pathname#chop_basename
  # WARNING! Pathname#chop_basename in STDLIB doesn't handle blank strings correctly!
  # This implementation correctly handles blank strings just as Pathname had intended
  # to handle non-path strings.
  def self.chop_basename(pth)
    d = Rust.dirname_for_chop(pth)
    b = Rust.basename_for_chop(pth)
    [d, b] unless Rust.both_are_blank(d, b)
  end

  def self.blank?(str)
    Rust.is_blank(str)
  end

  def self.basename(pth, ext="")
    Basename::Binding.basename(pth, ext).to_s
  end

  def self.plus(pth, pth2)
    Plus::Binding.plus(pth, pth2).to_s
  end

  def self.add_trailing_separator(pth)
    AddTrailingSeparator::Binding.add_trailing_separator(pth).to_s
  end

  def self.has_trailing_separator?(pth)
    Rust.has_trailing_separator(pth)
  end

  def self.extname(pth)
    Extname::Binding.extname(pth).to_s
  end

  def self.entries(pth)
    ptr = FFI::MemoryPointer.new(:pointer)
    len = Rust.entries(pth, ptr)
    Array(Rust::FromRustArray.new(len, ptr))
  end

  module Rust
    extend FFI::Library
    ffi_lib ::FasterPath::FFI_LIBRARY

    class FromRustArray < FFI::Struct
      layout :len,    :size_t, # dynamic array layout
             :data,   :pointer #

      def initialize(a, b)
        self[:len] = a
        self[:data] = b
      end

      def to_a
        self[:data].get_array_of_string(0, self[:len]).compact
      end

      # def self.release(ptr)
      #   Rust.free_array(ptr)
      # end
    end

    attach_function :rust_arch_bits, [], :int32
    attach_function :is_absolute, [ :string ], :bool
    attach_function :is_directory, [ :string ], :bool
    attach_function :is_relative, [ :string ], :bool
    attach_function :is_blank, [ :string ], :bool
    attach_function :both_are_blank, [ :string, :string ], :bool
    attach_function :basename_for_chop, [ :string ], :string
    attach_function :dirname_for_chop, [ :string ], :string
    attach_function :has_trailing_separator, [ :string ], :bool
    attach_function :entries, [ :string, :pointer ], :size_t
    # attach_function :free_array, [ :pointer ], :void
  end
  private_constant :Rust
end
