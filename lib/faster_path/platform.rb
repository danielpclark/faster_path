module FasterPath
  module Platform
    class << self
      def ffi_library
        file = [
          lib_prefix,
          "faster_path.",
          lib_suffix
        ]

        File.join(rust_release, file.join())
      end

      def operating_system
        case host_os()
        when /linux|bsd|solaris/
          "linux"
        when /darwin/
          "darwin"
        when /mingw|mswin/
          "windows"
        else
          host_os()
        end
      end

      def lib_prefix
        case operating_system()
        when /windows/
          ''
        when /cygwin/
          'cyg'
        else
          'lib'
        end
      end

      def lib_suffix
        case operating_system()
        when /darwin/
          'dylib'
        when /linux/
          'so'
        when /windows|cygwin/
          'dll'
        else
          'so'
        end
      end

      def rust_release
        File.expand_path("../../target/release/", __dir__)
      end

      def host_os
        RbConfig::CONFIG['host_os'].downcase
      end
    end
  end
end
