require 'ffi'

class FasterPath::ChopBasename < FFI::AutoPointer
  def self.release(ptr)
    !null? and Binding.free(ptr)
  end

  def to_s
    @str ||= begin
      null? ? nil : read_string.force_encoding('UTF-8')
    end
  end

  module Binding
    extend FFI::Library
    ffi_lib ::FasterPath::FFI_LIBRARY

    attach_function :chop_basename, :chop_basename, [ :string ], ::FasterPath::ChopBasename
    attach_function :free, :free_string, [ ::FasterPath::ChopBasename ], :void
  end
end
