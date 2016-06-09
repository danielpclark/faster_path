module FasterPath
  module RefinePathname
    refine Pathname do
      def absolute?
        FasterPath.absolute?(@path)
      end
    end
  end
end
