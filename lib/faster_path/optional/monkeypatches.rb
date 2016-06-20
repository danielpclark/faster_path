module FasterPath
  def self.sledgehammer_everything!
    ::File.class_eval do
      def basename(pth)
        FasterPath.basename(pth)
      end
    end

    ::Pathname.class_eval do
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
    end
  end
end
