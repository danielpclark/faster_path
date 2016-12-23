

describe "Gemspec has valid file list" do
  it "is up to date" do
    gemspec = eval IO.read( File.join(File.expand_path(Pathname(__FILE__).parent.parent), 'faster_path.gemspec') )

    _(Dir.chdir('.') do
      `git ls-files -z`.split("\x0").reject { |f|
        f.match(%r{^(test|spec|features|assets|benches)/})
      }
    end).must_equal gemspec.files
  end
end
