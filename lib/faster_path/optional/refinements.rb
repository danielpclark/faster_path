module FasterPath
  module RefinePathname
    refine Pathname do
      def absolute?
        FasterPath.absolute?(@path)
      end

      def chop_basename(pth)
        FasterPath.chop_basename(pth)
      end
      private :chop_basename

      def relative?
        FasterPath.relative?(@path)
      end
    end
  end
end
