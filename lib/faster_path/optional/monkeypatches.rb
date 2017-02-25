require 'pathname'

module FasterPath
  def self.sledgehammer_everything!
    ::File.class_eval do
      def self.basename(pth, ext = '')
        FasterPath.basename(pth, ext)
      end if ENV['WITH_REGRESSION']

      def self.extname(pth)
        FasterPath.extname(pth)
      end

      def self.dirname(pth)
        FasterPath.dirname(pth)
      end if ENV['WITH_REGRESSION']
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

      def has_trailing_separator?(pth)
        FasterPath.has_trailing_separator?(pth)
      end
      private :has_trailing_separator?

      def entries
        FasterPath.entries(@path)
      end if ENV['WITH_REGRESSION']
    end
    "CAUTION: Monkey patching effects everything! Be very sure you want this!"
  end
end
