require 'rbconfig'
require 'rutie'
require 'rutie/rake_task'

module FasterPath
  # Defines `rutie:build` and `rutie:clean`, which build the Rust extension
  # with Cargo. Used by the Rakefile and by ext/Rakefile when the gem is
  # installed; both may run from another directory, so Cargo is pointed at
  # the gem's Cargo.toml.
  #
  # The Rakefile leaves the library in target/release, where Rutie loads it
  # from; ext/Rakefile moves it to lib/faster_path so that the installed gem
  # keeps nothing else of the build.
  module BuildTask
    PROJECT_DIR = File.expand_path('../..', __dir__)
    # Where `gem install` puts the built library
    INSTALLED_LIBRARY_DIR = File.join(PROJECT_DIR, 'lib', 'faster_path')

    def self.define
      # Rutie's build script links the extension to the Ruby named by RUBY,
      # or the first `ruby` on the PATH; make it the Ruby running Rake.
      ENV['RUBY'] ||= File.join(RbConfig::CONFIG['bindir'],
                                RbConfig::CONFIG['ruby_install_name'] + RbConfig::CONFIG['EXEEXT'])
      Rutie::RakeTask.new(cargo_args: ['--manifest-path', File.join(PROJECT_DIR, 'Cargo.toml')])
    end

    # The library `rutie:build` builds, in the project's target/release
    def self.built_library
      Rutie.new(:faster_path).ffi_library(File.join(PROJECT_DIR, 'lib'))
    end

    # Whether Cargo can be run: named by CARGO, or on the PATH
    def self.cargo?
      return true if ENV['CARGO']

      exts = RbConfig::CONFIG['EXEEXT'].empty? ? [''] : ['', RbConfig::CONFIG['EXEEXT']]
      ENV.fetch('PATH', '').split(File::PATH_SEPARATOR).any? do |dir|
        exts.any? { |ext| File.executable?(File.join(dir, "cargo#{ext}")) }
      end
    end
  end
end
