module FasterPath
  module RefineFile
    refine File do
      def self.basename(pth, ext = '')
        FasterPath.basename(pth, ext)
      end if ENV['WITH_REGRESSION']

      def self.extname(pth)
        FasterPath.extname(pth)
      end
    end
  end

  module RefinePathname
    refine Pathname do
      def absolute?
        FasterPath.absolute?(@path)
      end

      def directory?
        FasterPath.directory?(@path)
      end

      def chop_basename(pth)
        FasterPath.chop_basename(pth)
      end
      private :chop_basename

      def relative?
        FasterPath.relative?(@path)
      end

      def add_trailing_separator(pth)
        FasterPath.add_trailing_separator(pth)
      end
      private :add_trailing_separator

      def has_trailing_separator?(pth)
        FasterPath.has_trailing_separator?(pth)
      end
    end
  end
end
