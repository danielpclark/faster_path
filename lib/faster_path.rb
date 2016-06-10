require "faster_path/version"
require 'pathname'
require "ffi"

module FasterPath
  # Spec to Pathname#absolute?
  def self.absolute?(pth)
    Rust.is_absolute(pth)
  end

  # Spec to Pathname#chop_basename
  # WARNING! Pathname#chop_basename in STDLIB doesn't handle blank strings correctly!
  # This implementation correctly handles blank strings just as Pathname had intended
  # to handle non-path strings.
  def self.chop_basename(pth)
    d,b = [Rust.dirname(pth), Rust.basename(pth)]
    if blank?(d) && blank?(b)
      nil
    else
      [d,b]
    end
  end

  def self.blank?(str)
    Rust.is_blank(str)
  end

  private
  module Rust
    extend FFI::Library
    ffi_lib begin
      prefix = Gem.win_platform? ? "" : "lib"
      "#{File.expand_path("../target/release/", File.dirname(__FILE__))}/#{prefix}faster_path.#{FFI::Platform::LIBSUFFIX}"
    end
    attach_function :is_absolute, [ :string ], :bool
    attach_function :is_blank, [ :string ], :bool
    attach_function :basename, [ :string ], :string
    attach_function :dirname, [ :string ], :string
  end
  private_constant :Rust
end
