source 'https://rubygems.org' do
  # Specify your gem's dependencies in faster_path.gemspec
  gemspec
end

begin
# https://github.com/ruby/spec dependencies
eval_gemfile File.expand_path('spec/ruby_spec/Gemfile', File.dirname(__FILE__))
rescue
  `git submodule update --init`
  eval_gemfile File.expand_path('spec/ruby_spec/Gemfile', File.dirname(__FILE__))
end
