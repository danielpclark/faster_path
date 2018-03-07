require 'rubygems'

module Thermite
  class Config
    def ruby_version
      "ruby#{RUBY_VERSION}"
    end
  end

  module GithubReleaseBinary
    def github_download_uri(_tag, version)
      spec = Gem::Specification.load("faster_path.gemspec")
      "#{github_uri}/releases/download/v#{spec.version}/#{config.tarball_filename(version)}"
    end
  end
end
