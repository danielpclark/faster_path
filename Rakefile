require "bundler/gem_tasks"
require "rake/testtask"
require 'fileutils'

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

desc "Build Rust extension"
task :build_src do
  puts "Building extension..."
  sh "cargo build --release"
end

desc "Build Rust extension"
task :test_build_src do
  puts "Building extension..."
  sh "cargo build"
end

desc "Clean up Rust build"
task :clean_src do
  puts "Cleaning up build..."
  # Remove all but library file
  FileUtils.
    rm_rf(
      Dir.
      glob('target/release/*').
      keep_if do |f|
        !f[/\.(?:so|dll|dylib|deps)\z/]
      end
  )
end

desc "Build + clean up Rust extension"
task build_lib: [:build_src, :clean_src] do
  puts "Completed build!"
end

desc "Code Quality Check"
task :lint do
  puts
  puts "Quality check starting..."
  sh "rubocop"
  puts
end

desc "Run Rust Tests"
task :cargo do
  sh "cargo test -- --nocapture"
end

Rake::TestTask.new(minitest: :test_build_src) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList['test/**/*_test.rb']
end

task test: [:cargo, :minitest, :lint] do |_t|
  exec 'spec/mspec/bin/mspec --format spec core/file/basename core/file/extname core/file/dirname library/pathname'
end

desc "Full mspec results w/o encoding"
task :mspec_full do
  exec %(bash -c "TEST_MONKEYPATCHES=true WITH_REGRESSION=true spec/mspec/bin/mspec --format spec core/file/basename core/file/extname core/file/dirname library/pathname")
end

desc "Full mspec results w/ encoding"
task :mspec_encoding_full do
  exec %(bash -c "ENCODING=1 TEST_MONKEYPATCHES=true WITH_REGRESSION=true mspec --format spec core/file/basename core/file/extname core/file/dirname library/pathname")
end

Rake::TestTask.new(bench: :build_lib) do |t|
  t.libs = %w[lib test]
  t.pattern = 'test/**/*_benchmark.rb'
end

Rake::TestTask.new(pbench: :build_lib) do |t|
  t.libs = %w[lib test test/pbench]
  t.pattern = 'test/pbench/pbench_suite.rb'
end

task default: :test
