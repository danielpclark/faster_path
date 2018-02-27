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

# SOME DEFAULTS
PATHNAME_A = Pathname.new('a').freeze
PATHNAME_DOT = Pathname.new('.').freeze
PATHNAME_PWD = Pathname.new('./').freeze
PATHNAME_SRC = Pathname.new('./src').freeze
PATHNAME_ABC = Pathname.new('a//b/c').freeze
PATHNAME_ABS = Pathname.new('/hello').freeze
PATHNAME_REL = Pathname.new('goodbye').freeze

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
      PATHNAME_ABC.absolute?
      PATHNAME_REL.absolute?
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
      PATHNAME_DOT.send(:add_trailing_separator, '/hello/world')
      PATHNAME_DOT.send(:add_trailing_separator, '/hello/world/')
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
      PATHNAME_DOT.children
    end
  end
}
PBENCHES[:children_compat] = {
 new: lambda do |x|
   (x/5).times do
     FasterPath.children_compat('.')
   end
 end,
 old: lambda do |x|
   (x/5).times do
     PATHNAME_DOT.children
   end
 end
}
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
      PATHNAME_DOT.send :chop_basename, "/hello/world.txt"
      PATHNAME_DOT.send :chop_basename, "world.txt"
      PATHNAME_DOT.send :chop_basename, ""
    end
  end
}
PATHNAME_CA1 = Pathname.new('/../.././../a').freeze
PATHNAME_CA2 = Pathname.new('a/b/../../../../c/../d').freeze
PBENCHES[:cleanpath_aggressive] = {
  new: lambda do |x|
    x.times do
      Pathname.new(FasterPath.cleanpath_aggressive '/../.././../a')
      Pathname.new(FasterPath.cleanpath_aggressive 'a/b/../../../../c/../d')
    end
  end,
  old: lambda do |x|
    x.times do
      PATHNAME_CA1.send :cleanpath_aggressive
      PATHNAME_CA2.send :cleanpath_aggressive
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
      PATHNAME_ABS.directory?
      PATHNAME_REL.directory?
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
      PATHNAME_PWD.entries
      PATHNAME_SRC.entries
    end
  end
}
PBENCHES[:entries_compat] = {
  new: lambda do |x|
    (x/5).times do
      FasterPath.entries_compat("./")
      FasterPath.entries_compat("./src")
    end
  end,
  old: lambda do |x|
    (x/5).times do
      PATHNAME_PWD.entries
      PATHNAME_SRC.entries
    end
  end
}
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
      PATHNAME_DOT.send :has_trailing_separator?, '////a//aaa/a//a/aaa////'
      PATHNAME_DOT.send :has_trailing_separator?, 'hello/'
    end
  end
}
PBENCHES[:join] = {
  new: lambda do |x|
    x.times do
      FasterPath.join('a', 'b')
      FasterPath.join('.', 'b')
      FasterPath.join('a//b/c', '../d//e')
    end
  end,
  old: lambda do |x|
    x.times do
      PATHNAME_A.send(:join, 'b')
      PATHNAME_DOT.send(:join, 'b')
      PATHNAME_ABC.send(:join, '../d//e')
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
      PATHNAME_A.send(:plus, 'a', 'b')
      PATHNAME_DOT.send(:plus, '.', 'b')
      PATHNAME_ABC.send(:plus, 'a//b/c', '../d//e')
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
      PATHNAME_ABS.relative?
      PATHNAME_REL.relative?
    end
  end
}
Pbench.new(nil).run(PBENCHES)
