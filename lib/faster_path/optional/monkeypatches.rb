module FasterPath
  def self.monkeypatch_pathname
    class << ::Pathname
      def absolute?
        FasterPath.absolute?(@path)
      end
    end
  end
end

