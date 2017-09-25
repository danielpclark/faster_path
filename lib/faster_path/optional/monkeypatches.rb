require 'pathname'

module FasterPath
  module MonkeyPatches
    # rubocop:disable Metrics/CyclomaticComplexity
    # rubocop:disable Metrics/PerceivedComplexity
    def self._ruby_core_file!
      ::File.class_eval do
        def self.basename(pth, ext = '')
          pth = pth.to_path if pth.respond_to? :to_path
          raise TypeError unless pth.is_a?(String) && ext.is_a?(String)
          FasterPath.basename(pth, ext)
        end

        def self.extname(pth)
          pth = pth.to_path if pth.respond_to? :to_path
          raise TypeError unless pth.is_a? String
          FasterPath.extname(pth)
        end

        def self.dirname(pth)
          pth = pth.to_path if pth.respond_to? :to_path
          raise TypeError unless pth.is_a? String
          FasterPath.dirname(pth)
        end
      end
    end
    def self._ruby_library_pathname!
      ::Pathname.class_eval do
        def absolute?
          FasterPath.absolute?(@path)
        end

        def add_trailing_separator(pth)
          FasterPath.add_trailing_separator(pth)
        end
        private :add_trailing_separator

        # Do NOT remove; waiting for fix in ruru
        # def children(with_dir=true)
        #   FasterPathname::Public.allocate.send(:children_compat, @path, with_dir)
        # end if !!ENV['WITH_REGRESSION']

        def chop_basename(pth)
          FasterPath.chop_basename(pth)
        end
        private :chop_basename

        def directory?
          FasterPath.directory?(@path)
        end

        # Do NOT remove; waiting for fix in ruru
        # def entries
        #   FasterPathname::Public.allocate.send(:entries_compat, @path)
        # end if !!ENV['WITH_REGRESSION']

        def has_trailing_separator?(pth)
          FasterPath.has_trailing_separator?(pth)
        end
        private :has_trailing_separator?

        def plus(pth, pth2)
          FasterPath.plus(pth, pth2)
        end
        private :plus

        def relative?
          FasterPath.relative?(@path)
        end
      end
    end
  end
  private_constant :MonkeyPatches

  def self.sledgehammer_everything!(include_file = !!ENV['WITH_REGRESSION'])
    MonkeyPatches._ruby_core_file! if include_file # SLOW; DON'T AUTO INCLUDE
    MonkeyPatches._ruby_library_pathname!
    "CAUTION: Monkey patching effects everything! Be very sure you want this!"
  end
end
