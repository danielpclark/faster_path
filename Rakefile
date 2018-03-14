require 'bundler/gem_tasks'
require 'rake/testtask'
require 'thermite/tasks'
require 'faster_path/thermite_initialize'

desc 'System Details'
task :sysinfo do
  puts "** FasterPath - faster_path v#{FasterPath::VERSION} **"
  puts RUBY_DESCRIPTION
  puts `rustc -Vv`
  puts `ldd --version`.scan(/(.*)\n/).first if RbConfig::CONFIG["host_os"].to_s[/linux/]
  puts `cargo -Vv`
  deps = Regexp.new(/name = "([\w\-]+)"\nversion = "(\d+\.\d+\.\d+)"/)
  puts "** Rust dependencies **"
  IO.read("Cargo.lock").
    scan(deps).
    delete_if {|i| i[0] == "faster_path" }.
    each {|name, version| puts "#{name.ljust(20)}#{version}"}
  puts "** Ruby dependencies **"
  deps = IO.read("Gemfile.lock")
  puts deps[(deps =~ /DEPENDENCIES/)..-1].sub("\n\n", "\n")
  puts "RAKE\n   #{Rake::VERSION}"
end

thermite = Thermite::Tasks.new

desc "Generate Contriburs.md Manifest"
task :contrib do
  puts "Generating Contriburs.md Manifest"
  exec "printf '# Contributors\n
' > Contributors.md;git shortlog -s -n -e | sed 's/^......./- /' >> Contributors.md"
end

desc "Add libruby to deps"
task :libruby_release do
  filename = RbConfig::CONFIG["LIBRUBY_ALIASES"].split(" ").first
  libfile = File.join(RbConfig::CONFIG["libdir"], filename)
  deps = "target/release/deps"

  printf "Copying libruby.so ... "
  unless File.exist? "#{deps}/#{filename}"
    FileUtils.mkdir_p deps
    FileUtils.cp libfile, deps
  end
  exit 1 unless File.exist? "#{deps}/#{filename}"
  puts "libruby.so copied."
end

desc "Add libruby to debug deps"
task :libruby_debug do
  filename = RbConfig::CONFIG["LIBRUBY_ALIASES"].split(" ").first
  libfile = File.join(RbConfig::CONFIG["libdir"], filename)
  deps = "target/debug/deps"

  printf "Copying libruby.so ... "
  unless File.exist? "#{deps}/#{filename}"
    FileUtils.mkdir_p deps
    FileUtils.cp libfile, deps
  end
  exit 1 unless File.exist? "#{deps}/#{filename}"
  puts "libruby.so copied."
end

desc 'Build + clean up Rust extension'
task build_lib: 'thermite:build' do
  thermite.run_cargo 'clean'
end

desc 'Code Quality Check'
task :lint do
  puts
  puts 'Quality check starting...'
  sh 'rubocop'
  puts
end

desc "Run Rust Tests"
task cargo: :libruby_debug do
  sh "cargo test -- --nocapture"
end

Rake::TestTask.new(minitest: :build_lib) do |t|
  t.libs = %w[lib test]
  t.pattern = 'test/**/*_test.rb'
end

task :init_mspec do |_t|
  if Dir.open('spec/mspec').entries.-([".", ".."]).empty?
    `git submodule init`
    `git submodule update`
  end
end

task test: [:cargo, :minitest, :lint, :pbench, :init_mspec] do |_t|
  exec 'spec/mspec/bin/mspec --format spec core/file/basename core/file/extname core/file/dirname library/pathname'
end

desc "Full mspec results w/o encoding"
task mspec_full: :init_mspec do
  exec %(bash -c "TEST_MONKEYPATCHES=true WITH_REGRESSION=true spec/mspec/bin/mspec --format spec core/file/basename core/file/extname core/file/dirname library/pathname")
end

desc "Full mspec results w/ encoding"
task mspec_encoding_full: :init_mspec do
  exec %(bash -c "ENCODING=1 TEST_MONKEYPATCHES=true WITH_REGRESSION=true mspec --format spec core/file/basename core/file/extname core/file/dirname library/pathname")
end

Rake::TestTask.new(bench: :build_lib) do |t|
  if ENV['GRAPH']
    `bundle install`
  end
  t.libs = %w[lib test]
  t.pattern = 'test/**/*_benchmark.rb'
end

Rake::TestTask.new(pbench: :build_lib) do |t|
  if ARGV.last == '--long-run'
    ENV['LONG_RUN'] = '10'
  end
  t.libs = %w[lib test test/pbench]
  t.pattern = 'test/pbench/pbench_suite.rb'
end

task default: :test
