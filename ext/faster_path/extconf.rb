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

Dir.chdir('../../') do
  system("rake build_src")
  system("rake clean_src")
end

create_makefile('faster_path/dummy')
