require 'fileutils'
require 'mkmf'

have_header('Dummy Makefile')

unless find_executable('cargo')
  puts "You need to have Rust installed for this gem to build natively."
  puts "Please install the latest nightly build:"
  puts
  puts "curl -sSf https://static.rust-lang.org/rustup.sh | sudo sh -s -- --channel=nightly"
  puts
  END { puts "Exiting..."}
end

#File.touch(File.join(Dir.pwd, 'faster_path.' + RbConfig::CONFIG['DLEXT']))

Dir.chdir('../../') do
  #puts "PWD: #{`pwd`}"
  #puts "ENV: #{}"
  system("rake build_src")
  system("rake clean_src")
end

#$extmk=false
#$makefile_created = true
create_makefile('faster_path/dummy')
