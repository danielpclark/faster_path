load 'spec/ruby_spec/default.mspec'

class MSpecScript
  set :requires, ["-r#{File.expand_path('init', File.dirname(__FILE__))}"]
  set :prefix, 'spec/ruby_spec'
end

MSpec.disable_feature :encoding unless ENV['ENCODING']
