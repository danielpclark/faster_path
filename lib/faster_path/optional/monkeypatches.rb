module FasterPath
  def self.sledgehammer_everything!
    class << ::Pathname
      def absolute?
        FasterPath.absolute?(@path)
      end
    end
  end
end

