require 'bundler/gem_tasks'
require 'rake/testtask'
require 'thermite/tasks'
require 'faster_path/thermite_initialize'

desc 'System Details'
task :sysinfo do
  puts "faster_path #{FasterPath::VERSION}"
  puts
  puts `rustc -Vv`
  puts `cargo -Vv`
  IO.read('Cargo.toml').split('[dependencies]').last.split("\n").
    select {|line| line =~ /=/ }.
    each {|line| puts "%s\t%s" % line.match(/(\S+)[ ="']{1,4}([\d.]+)["']?/).captures }
  puts
  puts RUBY_DESCRIPTION
  puts "bundler\t#{Bundler::VERSION}"
  puts "rake\t#{Rake::VERSION}"
  require 'ffi/version';
  puts "ffi\t#{FFI::VERSION}"
  begin
    puts "%s\t%s" % IO.read('Gemfile.lock').match(/(mspec) \(([\d\.]+)\)/).captures
  rescue Errno::ENOENT => _
    puts "\nNo Gemfile.lock"
  end
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
  t.libs = %w[lib test test/pbench]
  t.pattern = 'test/pbench/pbench_suite.rb'
end

task default: :test
