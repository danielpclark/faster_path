module FasterPath
  module RefineFile
    refine File do
      def basename(pth)
        FasterPath.basename(pth)
      end if FasterPath.respond_to? :basename
    end
  end unless true # No need to open class when we're not using it yet

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
