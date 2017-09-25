require 'pbench'

# PBENCHES[:""] = {
#  new: lambda do |x|
#    x.times do
#    end
#  end,
#  old: lambda do |x|
#    x.times do
#    end
#  end
# }

PBENCHES = {}
PBENCHES[:"allocate, instead of new,"] = {
  # The allocate test is both for demonstration and as a baseline for comparing
  # different results from different computer systems.  The code for these methods
  # won't change.
  new: lambda do |x|
    x.times do
      Pathname.allocate
    end
  end,
  old: lambda do |x|
    x.times do
      Pathname.new("")
    end
  end
}
PBENCHES[:"absolute?"] = {
  new: lambda do |x|
    x.times do
      FasterPath.absolute?("/hello")
      FasterPath.absolute?("goodbye")
    end
  end,
  old: lambda do |x|
    x.times do
      Pathname.new("/hello").absolute?
      Pathname.new("goodbye").absolute?
    end
  end,
}
PBENCHES[:add_trailing_separator] = {
  new: lambda do |x|
    x.times do
      FasterPath.add_trailing_separator('/hello/world')
      FasterPath.add_trailing_separator('/hello/world/')
    end
  end,
  old: lambda do |x|
    x.times do
      Pathname.allocate.send(:add_trailing_separator, '/hello/world')
      Pathname.allocate.send(:add_trailing_separator, '/hello/world/')
    end
  end
}
PBENCHES[:basename] = {
  new: lambda do |x|
    x.times do
      FasterPath.basename("/hello/world")
      FasterPath.basename('/home/gumby/work/ruby.rb', '.rb')
      FasterPath.basename('/home/gumby/work/ruby.rb', '.*')
    end
  end,
  old: lambda do |x|
    x.times do
      File.basename("/hello/world")
      File.basename('/home/gumby/work/ruby.rb', '.rb')
      File.basename('/home/gumby/work/ruby.rb', '.*')
    end
  end
}
PBENCHES[:children] = {
  new: lambda do |x|
    (x/5).times do
      FasterPath.children(".")
    end
  end,
  old: lambda do |x|
    (x/5).times do
      Pathname.new(".").children
    end
  end
}
# Do NOT remove; waiting for fix in ruru
# PBENCHES[:children_compat] = {
#  new: lambda do |x|
#    (x/5).times do
#      FasterPathname::Public.allocate.send(:children_compat, '.')
#    end
#  end,
#  old: lambda do |x|
#    (x/5).times do
#      Pathname.new('.').children
#    end
#  end
# }
PBENCHES[:chop_basename] = {
  new: lambda do |x|
    x.times do
      FasterPath.chop_basename "/hello/world.txt"
      FasterPath.chop_basename "world.txt"
      FasterPath.chop_basename ""
    end
  end,
  old: lambda do |x|
    x.times do
      Pathname.allocate.send :chop_basename, "/hello/world.txt"
      Pathname.allocate.send :chop_basename, "world.txt"
      Pathname.allocate.send :chop_basename, ""
    end
  end
}
PBENCHES[:"directory?"] = {
  new: lambda do |x|
    x.times do
      FasterPath.directory?("/hello")
      FasterPath.directory?("goodbye")
    end
  end,
  old: lambda do |x|
    x.times do
      Pathname.new("/hello").directory?
      Pathname.new("goodbye").directory?
    end
  end
}
PBENCHES[:dirname] = {
  new: lambda do |x|
    x.times do
      FasterPath.dirname "/really/long/path/name/which/ruby/doesnt/like/bar.txt"
      FasterPath.dirname "/foo/"
      FasterPath.dirname "."
    end
  end,
  old: lambda do |x|
    x.times do
      File.dirname "/really/long/path/name/which/ruby/doesnt/like/bar.txt"
      File.dirname "/foo/"
      File.dirname "."
    end
  end
}
PBENCHES[:entries] = {
  new: lambda do |x|
    (x/5).times do
      FasterPath.entries("./")
      FasterPath.entries("./src")
    end
  end,
  old: lambda do |x|
    (x/5).times do
      Pathname.new("./").entries
      Pathname.new("./src").entries
    end
  end
}
# Do NOT remove; waiting for fix in ruru
# PBENCHES[:entries_compat] = {
#   new: lambda do |x|
#     (x/5).times do
#       FasterPathname::Public.allocate.send(:entries_compat, "./")
#       FasterPathname::Public.allocate.send(:entries_compat, "./src")
#     end
#   end,
#   old: lambda do |x|
#     (x/5).times do
#       Pathname.new("./").entries
#       Pathname.new("./src").entries
#     end
#   end
# }
PBENCHES[:extname] = {
  new: lambda do |x|
    x.times do
      FasterPath.extname('verylongfilename_verylongfilename.rb')
      FasterPath.extname('/very/long/path/name/very/long/path/name/very/long/path/name/file.rb')
      FasterPath.extname('/ext/mail.rb')
    end
  end,
  old: lambda do |x|
    x.times do
      File.extname('verylongfilename_verylongfilename.rb')
      File.extname('/very/long/path/name/very/long/path/name/very/long/path/name/file.rb')
      File.extname('/ext/main.rb')
    end
  end
}
PBENCHES[:"has_trailing_separator?"] = {
  new: lambda do |x|
    x.times do
      FasterPath.has_trailing_separator? '////a//aaa/a//a/aaa////'
      FasterPath.has_trailing_separator? 'hello/'
    end
  end,
  old: lambda do |x|
    x.times do
      Pathname.allocate.send :has_trailing_separator?, '////a//aaa/a//a/aaa////'
      Pathname.allocate.send :has_trailing_separator?, 'hello/'
    end
  end
}
PBENCHES[:plus] = {
  new: lambda do |x|
    x.times do
      FasterPath.plus('a', 'b')
      FasterPath.plus('.', 'b')
      FasterPath.plus('a//b/c', '../d//e')
    end
  end,
  old: lambda do |x|
    x.times do
      Pathname.allocate.send(:plus, 'a', 'b')
      Pathname.allocate.send(:plus, '.', 'b')
      Pathname.allocate.send(:plus, 'a//b/c', '../d//e')
    end
  end
}
PBENCHES[:"relative?"] = {
  new: lambda do |x|
    x.times do
      FasterPath.relative?("/hello")
      FasterPath.relative?("goodbye")
    end
  end,
  old: lambda do |x|
    x.times do
      Pathname.new("/hello").relative?
      Pathname.new("goodbye").relative?
    end
  end
}
Pbench.new(nil).run(PBENCHES)
