require "bundler/gem_tasks"
require "rake/testtask"
require 'fileutils'

desc "Build Rust extension"
task :build_src do
  puts "Building extension..."
  system("cargo build --release")
end

desc "Clean up Rust build"
task :clean_src do
  puts "Cleaning up build..."
  # Remove all but library file
  FileUtils.
    rm_rf(
      Dir.
      glob('target/release/*').
      keep_if {|f|
        !f[/\.(?:so|dll|dylib|deps)\z/]
      }
  )
end

desc "Build + clean up Rust extension"
task build_lib: [:build_src, :clean_src] do
  puts "Completed build!"
end

Rake::TestTask.new(minitest: :build_lib) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList['test/**/*_test.rb']
end

task :test => :minitest do |t|
  exec 'mspec --format spec core/file library/pathname'
end

Rake::TestTask.new(bench: :build_lib) do |t|
  t.libs = %w(lib test)
  t.pattern = 'test/**/*_benchmark.rb'
end

Rake::TestTask.new(pbench: :build_lib) do |t|
  t.libs = %w(lib test test/pbench)
  t.pattern = 'test/pbench/pbench_suite.rb'
end

task default: :test
