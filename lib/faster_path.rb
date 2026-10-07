require 'faster_path/version'
require 'pathname'
require 'rutie'
require 'fiddle'
require 'fiddle/import'

# FasterPath module behaves as a singleton object with the alternative method
# implementations for `Pathname`, and some for `File`, available directly on it.
#
# New projects are recommend to reference methods defined directly on `FasterPath`.
# Existing websites may use the `FasterPath.sledgehammer_everything!` method to
# directly injet the more performant implementations of path handling in to their
# existing code ecosystem.  To do so you will need to
# `require 'faster_path/optional/monkeypatches'` beforehand.
module FasterPath
  # The Rust extension: in the project's target/release after `rake rutie:build`,
  # or in lib/faster_path, where `gem install` puts it (see ext/Rakefile)
  RUTIE = [Rutie.new(:faster_path), Rutie.new(:faster_path, lib_path: 'faster_path')].
    find { |rutie| File.exist?(rutie.ffi_library(__dir__)) } || Rutie.new(:faster_path)
  FFI_LIBRARY = RUTIE.ffi_library(__dir__)
  RUTIE.init('Init_faster_pathname', __dir__)

  Public.class_eval do
    private_class_method :basename
    private_class_method :children
    private_class_method :children_compat
  end

  # The Rust architecture bit width: 64bits or 32bits
  # @return [Integer]
  def self.rust_arch_bits
    Rust.rust_arch_bits
  end

  # The Ruby architecture bit width: 64bits or 32bits
  # @return [Integer]
  def self.ruby_arch_bits
    # The width of a pointer; `1.size` is the width of a C `long`, which is
    # 4 bytes on 64-bit Windows.
    [0].pack('J').bytesize * 8
  end

  # A very fast way to check if a string is blank
  # @param str [#to_s]
  # @return [true, false]
  def self.blank?(str)
    "#{str}".strip.empty?
  end

  # Rust implementation of File.basename
  # @param pth [String] the path to evaluate
  # @param ext [String] any extension to remove
  # @return [String]
  def self.basename(pth, ext="")
    Public.send(:basename, pth, ext)
  end

  # Rust implementation of Pathname.children
  # @param pth [String] the path to evaluate
  # @param with_directory [true, false] whether to include directory in results
  # @return [Array<String>]
  def self.children(pth, with_directory=true)
    Public.send(:children, pth, with_directory)
  end

  # Rust implementation of Pathname.children
  # @param pth [String] the path to evaluate
  # @param with_directory [true, false] whether to include directory in results
  # @return [Array<Pathname>]
  def self.children_compat(pth, with_directory=true)
    Public.send(:children_compat, pth, with_directory)
  end

  # @private
  module Rust
    extend Fiddle::Importer
    dlload FFI_LIBRARY
    extern 'int rust_arch_bits()'
  end
  private_constant :Rust
end
