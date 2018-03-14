require 'rubygems'
require 'rake/tasklib'
require_relative './version'

# @private :nodoc:
module Thermite
  # @private :nodoc:
  class Config
    # @private :nodoc:
    def ruby_version
      "ruby#{RUBY_VERSION}"
    end
  end

  # @private :nodoc:
  class Tasks < Rake::TaskLib
    # @private :nodoc:
    def github_download_uri(_tag, version)
      "#{github_uri}/releases/download/v#{FasterPath::VERSION}/#{config.tarball_filename(version)}"
    end
  end
end
