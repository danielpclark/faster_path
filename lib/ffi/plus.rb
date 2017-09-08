require 'ffi'

class FasterPath::Plus < FFI::AutoPointer
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

    attach_function :plus, :plus, [ :string, :string ], ::FasterPath::Plus
    attach_function :free, :free_string, [ ::FasterPath::Plus ], :void
  end
end
