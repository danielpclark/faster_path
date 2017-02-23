# frozen_string_literal: true
require "pathname"

module Spec
  module Path
    def root
      @root ||= Pathname.new(File.expand_path("../../..", __FILE__))
    end

    def tmp(*path)
      root.join("tmp", *path)
    end

    def bundled_app(*path)
      root = tmp.join("bundled_app")
      FileUtils.mkdir_p(root)
      root.join(*path)
    end
  end
end
