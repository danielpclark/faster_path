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

    # rubocop:disable Metrics/MethodLength
    def self._ruby_library_pathname!
      ::Pathname.class_eval do
        def absolute?
          FasterPath.absolute?(@path)
        end

        def add_trailing_separator(pth)
          FasterPath.add_trailing_separator(pth)
        end
        private :add_trailing_separator

        def children(with_dir=true)
          FasterPath.children_compat(@path, with_dir)
        end if !!ENV['WITH_REGRESSION']

        def chop_basename(pth)
          FasterPath.chop_basename(pth)
        end
        private :chop_basename

        def cleanpath_aggressive
          Pathname.new(FasterPath.cleanpath_aggressive(@path))
        end
        private :cleanpath_aggressive

        def cleanpath_conservative
          Pathname.new(FasterPath.cleanpath_conservative(@path))
        end
        private :cleanpath_conservative

        def directory?
          FasterPath.directory?(@path)
        end

        def entries
          FasterPath.entries_compat(@path)
        end if !!ENV['WITH_REGRESSION']

        def has_trailing_separator?(pth)
          FasterPath.has_trailing_separator?(pth)
        end
        private :has_trailing_separator?

        def join(*args)
          FasterPath.join(self, *args)
        end

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
