require "faster_path/version"
require 'pathname'
require "ffi"

module FasterPath
  # Spec to Pathname#absolute?
  def self.absolute?(pth)
    Rust.is_absolute(pth)
  end

  # Spec to Pathname#relative?
  def self.relative?(pth)
    Rust.is_relative(pth)
  end

  # Spec to Pathname#chop_basename
  # WARNING! Pathname#chop_basename in STDLIB doesn't handle blank strings correctly!
  # This implementation correctly handles blank strings just as Pathname had intended
  # to handle non-path strings.
  def self.chop_basename(pth)
    d,b = [Rust.dirname_for_chop(pth), Rust.basename_for_chop(pth)]
    [d,b] unless Rust.both_are_blank(d,b)
  end

  def self.blank?(str)
    Rust.is_blank(str)
  end

  # EXAMPLE
  #def self.one_and_two
  #  Rust.one_and_two.to_a
  #end

  private
  module Rust
    extend FFI::Library
    ffi_lib begin
      prefix = Gem.win_platform? ? "" : "lib"
      "#{File.expand_path("../target/release/", File.dirname(__FILE__))}/#{prefix}faster_path.#{FFI::Platform::LIBSUFFIX}"
    end

    class FromRustArray < FFI::Struct
      layout :len,    :size_t, # dynamic array layout
             :data,   :pointer #

      def to_a
        self[:data].get_array_of_string(0, self[:len]).compact
      end
    end

    attach_function :is_absolute, [ :string ], :bool
    attach_function :is_relative, [ :string ], :bool
    attach_function :is_blank, [ :string ], :bool
    attach_function :both_are_blank, [ :string, :string ], :bool
    attach_function :basename, [ :string ], :string
    attach_function :dirname, [ :string ], :string
    attach_function :basename_for_chop, [ :string ], :string # decoupling behavior
    attach_function :dirname_for_chop, [ :string ], :string # decoupling behavior

    # EXAMPLE
    #attach_function :one_and_two, [], FromRustArray.by_value
  end
  private_constant :Rust
end
