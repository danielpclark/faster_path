require 'pathname'

module FasterPath
  module RefineFile
    refine File do
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

  module RefinePathname
    refine Pathname do
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

      def directory?
        FasterPath.directory?(@path)
      end

      def entries
        FasterPath.entries_compat(@path)
      end if !!ENV['WITH_REGRESSION']

      def has_trailing_separator?(pth)
        FasterPath.has_trailing_separator?(pth)
      end

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
