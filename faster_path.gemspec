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

  spec.files         = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features|assets|benches)/}) }
  spec.bindir        = 'exe'
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.extensions << 'ext/faster_path/extconf.rb'
  spec.require_paths = ['lib']

  spec.add_dependency 'ffi', '~> 1.9'
  spec.add_development_dependency 'bundler', '~> 1.12'
  spec.add_development_dependency 'method_source', '~> 0.8.2'
  spec.add_development_dependency 'rake', '~> 11.1'
  spec.add_development_dependency 'minitest', '~> 5.8'
  spec.add_development_dependency 'minitest-reporters', '~> 1.1'
  spec.add_development_dependency 'color_pound_spec_reporter', '~> 0.0.5'
end
