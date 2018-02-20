# This is a redundancy check for the rust compiled library needed for this gem.
# If the asset is not available and we can't compile it from this code then FAIL
# on require of 'faster_path' with a very clear message as to why."

require_relative './platform'

module FasterPath
  module AssetResolution # BREAK IN CASE OF EMERGENCY ;-)
    class << self
      def verify!
        return lib_file if file?

        if rust?
          compile!
          raise "Rust failed to compile asset! The dynamic library for this package was not found." unless file?
          return lib_file
        end

        raise "The dynamic library for this package was not found nor was Rust's cargo executable found. This package will not work without it!"
      end

      private

      def compile!
        require 'open3'
        Dir.chdir(File.expand_path('../../', __dir__)) do
          Open3.popen3("rake build_lib") do |stdin, stdout, stderr, wait_thr|
            stdin.close

            wait_thr && wait_thr.value.exitstatus
            out = Thread.new { stdout.read }.value.strip
            Thread.new { stderr.read }.value
            out
          end
        end
        File.exist? lib_file
      end

      def rust?
        require 'mkmf'
        MakeMakefile::Logging.instance_variable_set(:@log, File.open(File::NULL, 'w'))
        MakeMakefile.instance_eval "undef :message; def message(*); end"
        MakeMakefile.find_executable('cargo')
      end

      def file?
        File.exist? lib_file
      end

      def lib_dir
        File.expand_path("../../target/release/", __dir__)
      end

      def lib_file
        Platform.ffi_library()
      end
    end
  end
end
FasterPath::AssetResolution.verify! unless ENV['TEST']
