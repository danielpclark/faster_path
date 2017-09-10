require 'ffi'

class FasterPath::Dirname < FFI::AutoPointer
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

    attach_function :dirname, :dirname, [ :string ], ::FasterPath::Dirname
    attach_function :free, :free_string, [ ::FasterPath::Dirname ], :void
  end
end
