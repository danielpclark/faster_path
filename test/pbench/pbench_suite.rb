require 'pbench'

#PBENCHES[:""] = {
#  new: ->x{
#    x.times do
#    end
#  },
#  old: ->x{
#    x.times do
#    end
#  }
#}

PBENCHES = {}
PBENCHES[:"allocate, instead of new,"] = {
  # The allocate test is both for demonstration and as a baseline for comparing
  # different results from different computer systems.  The code for these methods
  # won't change.
  new: ->x{
    x.times do
      Pathname.allocate
    end
  },
  old: ->x{
    x.times do
      Pathname.new("")
    end
  }
}
PBENCHES[:"absolute?"] = {
  new: ->x{
    x.times do
      FasterPath.absolute?("/hello")
      FasterPath.absolute?("goodbye")
    end
  },
  old: ->x{
    x.times do
      Pathname.new("/hello").absolute?
      Pathname.new("goodbye").absolute?
    end
  },
}
PBENCHES[:"add_trailing_separator"] = {
  new: ->x{
    x.times do
      FasterPath.add_trailing_separator('/hello/world')
      FasterPath.add_trailing_separator('/hello/world/')
    end
  },
  old: ->x{
    x.times do
      Pathname.allocate.send(:add_trailing_separator, '/hello/world')
      Pathname.allocate.send(:add_trailing_separator, '/hello/world/')
    end
  }
}
PBENCHES[:"basename"] = {
  new: ->x{
    x.times do
      FasterPath.basename("/hello/world")
      FasterPath.basename('/home/gumby/work/ruby.rb', '.rb')
      FasterPath.basename('/home/gumby/work/ruby.rb', '.*')
    end
  },
  old: ->x{
    x.times do
      File.basename("/hello/world")
      File.basename('/home/gumby/work/ruby.rb', '.rb')
      File.basename('/home/gumby/work/ruby.rb', '.*')
    end
  }
}
PBENCHES[:"chop_basename"] = {
  new: ->x{
    x.times do
      FasterPath.chop_basename "/hello/world.txt"
      FasterPath.chop_basename "world.txt"
      FasterPath.chop_basename ""
    end
  },
  old: ->x{
    x.times do
      Pathname.allocate.send :chop_basename, "/hello/world.txt"
      Pathname.allocate.send :chop_basename, "world.txt"
      Pathname.allocate.send :chop_basename, ""
    end
  }
}
PBENCHES[:"directory?"] = {
  new: ->x{
    x.times do
      FasterPath.directory?("/hello")
      FasterPath.directory?("goodbye")
    end
  },
  old: ->x{
    x.times do
      Pathname.new("/hello").directory?
      Pathname.new("goodbye").directory?
    end
  }
}
PBENCHES[:"dirname"] = {
  new: ->x{
    x.times do
      FasterPath.dirname "/really/long/path/name/which/ruby/doesnt/like/bar.txt"
      FasterPath.dirname "/foo/"
      FasterPath.dirname "."
    end
  },
  old: ->x{
    x.times do
      File.dirname "/really/long/path/name/which/ruby/doesnt/like/bar.txt"
      File.dirname "/foo/"
      File.dirname "."
    end
  }
}
PBENCHES[:"extname"] = {
  new: ->x{
    x.times do
      FasterPath.extname('verylongfilename_verylongfilename.rb')
      FasterPath.extname('/very/long/path/name/very/long/path/name/very/long/path/name/file.rb')
      FasterPath.extname('/ext/mail.rb')
    end
  },
  old: ->x{
    x.times do
      File.extname('verylongfilename_verylongfilename.rb')
      File.extname('/very/long/path/name/very/long/path/name/very/long/path/name/file.rb')
      File.extname('/ext/main.rb')
    end
  }
}
PBENCHES[:"has_trailing_separator?"] = {
  new: ->x{
    x.times do
      FasterPath.has_trailing_separator? '////a//aaa/a//a/aaa////'
      FasterPath.has_trailing_separator? 'hello/'
    end
  },
  old: ->x{
    x.times do
      Pathname.allocate.send :has_trailing_separator?, '////a//aaa/a//a/aaa////'
      Pathname.allocate.send :has_trailing_separator?, 'hello/'
    end
  }
}
PBENCHES[:"blank? (verses strip.empty?)"] = {
  new: ->x{
    x.times do
       FasterPath.blank? "world.txt"
       FasterPath.blank? "  "
       FasterPath.blank? ""
    end
  },
  old: ->x{
    x.times do
      "world.txt".strip.empty?
      "  ".strip.empty?
      "".strip.empty?
    end
  }
}
PBENCHES[:"relative?"] = {
  new: ->x{
    x.times do
      FasterPath.relative?("/hello")
      FasterPath.relative?("goodbye")
    end
  },
  old: ->x{
    x.times do
      Pathname.new("/hello").relative?
      Pathname.new("goodbye").relative?
    end
  }
}
PBENCHES[:"entries"] = {
  new: ->x{
    x.times do
      FasterPath.entries("./")
      FasterPath.entries("./src")
    end
  },
  old: ->x{
    x.times do
      Pathname.new("./").entries
      Pathname.new("./src").entries
    end
  }
}
Pbench.new(nil).run(PBENCHES)
