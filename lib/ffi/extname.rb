require 'ffi'

class FasterPath::Extname < FFI::AutoPointer
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

    attach_function :extname, :extname, [ :string ], ::FasterPath::Extname
    attach_function :free, :free_string, [ ::FasterPath::Extname ], :void
  end
end
