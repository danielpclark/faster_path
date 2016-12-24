# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'faster_path/version'

Gem::Specification.new do |spec|
  spec.name          = "faster_path"
  spec.version       = FasterPath::VERSION
  spec.authors       = ["Daniel P. Clark"]
  spec.email         = ["6ftdan@gmail.com"]
  spec.summary       = %q{Reimplementation of Pathname for better performance}
  spec.description   = %q{FasterPath is a reimplementation of Pathname for better performance.}
  spec.homepage      = "https://github.com/danielpclark/faster_path"
  spec.license       = "MIT OR Apache-2.0"

  spec.files         = [
    ".gitignore", ".travis.yml", "Cargo.lock", "Cargo.toml", "Gemfile",
    "MIT-LICENSE.txt", "README.md", "Rakefile", "bin/console", "bin/setup",
    "ext/faster_path/extconf.rb", "faster_path.gemspec", "lib/faster_path.rb",
    "lib/faster_path/optional/monkeypatches.rb", "lib/faster_path/optional/refinements.rb",
    "lib/faster_path/version.rb", "src/add_trailing_separator.rs", "src/basename.rs",
    "src/basename_for_chop.rs", "src/both_are_blank.rs", "src/dirname.rs",
    "src/dirname_for_chop.rs", "src/extname.rs", "src/has_trailing_separator.rs",
    "src/is_absolute.rs", "src/is_blank.rs", "src/is_directory.rs", "src/is_relative.rs",
    "src/lib.rs", "src/path_parsing.rs", "src/ruby_array.rs", "src/ruby_string.rs",
    "src/rust_arch_bits.rs"
  ]
  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.extensions    << "ext/faster_path/extconf.rb"
  spec.require_paths = ["lib"]

  spec.add_dependency "bundler", "~> 1.12"
  spec.add_dependency "rake", "~> 12.0"
  spec.add_dependency "ffi", "~> 1.9"
  spec.add_development_dependency "method_source", "~> 0.8.2"
  spec.add_development_dependency "minitest", "~> 5.10"
  spec.add_development_dependency "minitest-reporters", "~> 1.1"
  spec.add_development_dependency "color_pound_spec_reporter", "~> 0.0.5"
end
