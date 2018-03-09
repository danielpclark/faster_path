require 'faster_path/version'
require 'pathname'
require 'thermite/config'
require 'faster_path/thermite_initialize'
require 'fiddle'
require 'fiddle/import'

module FasterPath
  FFI_LIBRARY = begin
    toplevel_dir = File.dirname(__dir__)
    config = Thermite::Config.new(cargo_project_path: toplevel_dir,
                                  ruby_project_path: toplevel_dir)
    config.ruby_extension_path
  end

  Fiddle::Function.
    new(Fiddle.dlopen(FFI_LIBRARY)['Init_faster_pathname'], [], Fiddle::TYPE_VOIDP).
    call

  Public.class_eval do
    private_class_method :basename
    private_class_method :children
    private_class_method :children_compat
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
    Public.send(:children, pth, with_directory)
  end

  def self.children_compat(pth, with_directory=true)
    Public.send(:children_compat, pth, with_directory)
  end

  module Rust
    extend Fiddle::Importer
    dlload FFI_LIBRARY
    extern 'int rust_arch_bits()'
  end
  private_constant :Rust
end
