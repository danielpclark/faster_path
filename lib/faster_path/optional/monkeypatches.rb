require 'pathname'

# :nodoc:
module FasterPath
  # Core module for applying monkeypatches to `Pathname` and `File`
  module MonkeyPatches
    # Monkeypatch just `File` specific methods
    # rubocop:disable Metrics/CyclomaticComplexity
    # rubocop:disable Metrics/PerceivedComplexity
    def self._ruby_core_file!
      ::File.class_eval do
        # @private :nodoc:
        def self.basename(pth, ext = '')
          pth = pth.to_path if pth.respond_to? :to_path
          raise TypeError unless pth.is_a?(String) && ext.is_a?(String)
          FasterPath.basename(pth, ext)
        end

        # @private :nodoc:
        def self.extname(pth)
          pth = pth.to_path if pth.respond_to? :to_path
          raise TypeError unless pth.is_a? String
          FasterPath.extname(pth)
        end

        # @private :nodoc:
        def self.dirname(pth)
          pth = pth.to_path if pth.respond_to? :to_path
          raise TypeError unless pth.is_a? String
          FasterPath.dirname(pth)
        end
      end
    end

    # Monkeypatch just `Pathname` specific methods
    # rubocop:disable Metrics/MethodLength
    def self._ruby_library_pathname!
      ::Pathname.class_eval do
        # @private :nodoc:
        def absolute?
          FasterPath.absolute?(@path)
        end

        # @private :nodoc:
        def add_trailing_separator(pth)
          FasterPath.add_trailing_separator(pth)
        end
        private :add_trailing_separator

        # @private :nodoc:
        def children(with_dir=true)
          FasterPath.children_compat(@path, with_dir)
        end if !!ENV['WITH_REGRESSION']

        # @private :nodoc:
        def chop_basename(pth)
          FasterPath.chop_basename(pth)
        end
        private :chop_basename

        # @private :nodoc:
        def cleanpath_aggressive
          Pathname.new(FasterPath.cleanpath_aggressive(@path))
        end
        private :cleanpath_aggressive

        # @private :nodoc:
        def cleanpath_conservative
          Pathname.new(FasterPath.cleanpath_conservative(@path))
        end
        private :cleanpath_conservative

        # @private :nodoc:
        def del_trailing_separator(pth)
          FasterPath.del_trailing_separator(pth)
        end
        private :del_trailing_separator

        # @private :nodoc:
        def directory?
          FasterPath.directory?(@path)
        end

        # @private :nodoc:
        def entries
          FasterPath.entries_compat(@path)
        end if !!ENV['WITH_REGRESSION']

        # @private :nodoc:
        def has_trailing_separator?(pth)
          FasterPath.has_trailing_separator?(pth)
        end
        private :has_trailing_separator?

        # @private :nodoc:
        def join(*args)
          FasterPath.join(self, *args)
        end

        # @private :nodoc:
        def plus(pth, pth2)
          FasterPath.plus(pth, pth2)
        end
        private :plus

        # @private :nodoc:
        def relative?
          FasterPath.relative?(@path)
        end

        # @private :nodoc:
        def relative_path_from(other)
          FasterPath.relative_path_from(@path, other)
        end
      end
    end
  end
  private_constant :MonkeyPatches

  # Applies all performant monkeypatches to both `Pathname` and `File`
  def self.sledgehammer_everything!
    MonkeyPatches._ruby_core_file!
    MonkeyPatches._ruby_library_pathname!
  end
end
