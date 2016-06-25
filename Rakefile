require "bundler/gem_tasks"
require "rake/testtask"
require 'fileutils'

desc "Building extension..."
task :build_src do
  puts "Building extension..."
  system("cargo build --release")
end

desc "Cleaning up build..."
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

desc "Compiling Rust extension..."
task build_lib: [:build_src, :clean_src] do
  puts "Completed build!"
end

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList['test/**/*_test.rb']
end

Rake::TestTask.new(:bench) do |t|
  t.libs = %w(lib test)
  t.pattern = 'test/**/*_benchmark.rb'
end

Rake::TestTask.new(:pbench) do |t|
  t.libs = %w(lib test test/pbench)
  t.pattern = 'test/pbench/pbench_suite.rb'
end

task default: :test
