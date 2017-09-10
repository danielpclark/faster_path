require 'ffi'

class FasterPath::Basename < FFI::AutoPointer
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

    attach_function :basename, :basename, [ :string, :string ], ::FasterPath::Basename
    attach_function :free, :free_string, [ ::FasterPath::Basename ], :void
  end
end
