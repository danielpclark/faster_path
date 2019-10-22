# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'faster_path/version'

Gem::Specification.new do |spec|
  spec.name          = 'faster_path'
  spec.version       = FasterPath::VERSION
  spec.authors       = ['Daniel P. Clark']
  spec.email         = ['6ftdan@gmail.com']
  spec.summary       = 'Reimplementation of Pathname for better performance'
  spec.description   = 'FasterPath is a reimplementation of Pathname for better performance.'
  spec.homepage      = 'https://github.com/danielpclark/faster_path'
  spec.license       = 'MIT OR Apache-2.0'

  spec.files         = [
    'Cargo.lock', 'Cargo.toml', 'Gemfile', 'MIT-LICENSE.txt', 'README.md', 'Rakefile',
    'bin/console', 'bin/setup', 'ext/Rakefile', 'faster_path.gemspec', 'lib/faster_path.rb',
    'lib/faster_path/version.rb', 'lib/faster_path/thermite_initialize.rb',
    'lib/faster_path/optional/monkeypatches.rb', 'lib/faster_path/optional/refinements.rb'
  ]
  spec.files += Dir['src/**/*']

  spec.extensions    = ['ext/Rakefile']
  spec.require_paths = ['lib']

  spec.add_dependency 'bundler', '>= 1.12'
  spec.add_dependency 'rake', '>= 12.3'
  spec.add_dependency 'thermite', '0.13.0'
  spec.add_development_dependency 'read_source', '~> 0.2.6'
  spec.add_development_dependency 'minitest', '~> 5.11'
  spec.add_development_dependency 'minitest-reporters', '1.1'
  spec.add_development_dependency 'color_pound_spec_reporter', '~> 0.0.9'
  spec.add_development_dependency 'rubocop', '0.53'
  spec.add_development_dependency 'stop_watch', '~> 1.0'
  if !ENV['CI'] && ENV['GRAPH']
    spec.add_development_dependency 'gruff', '~> 0.7.0'
  end
end
