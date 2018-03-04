require 'test_helper'

class BasenameTest < Minitest::Test
	DRIVE = Dir.pwd[%r'\A(?:[a-z]:|//[^/]+/[^/]+)'i]
	POSIX = /cygwin|mswin|bccwin|mingw|emx/ !~ RUBY_PLATFORM
	NTFS = !(/mingw|mswin|bccwin/ !~ RUBY_PLATFORM)

	def regular_file
		return @file if defined? @file
		@file = make_tmp_filename("file")
		make_file("foo", @file)
		@file
	end

	def make_tmp_filename(prefix)
		"#{@dir}/#{prefix}.test"
	end

  def make_file(content, file)
    open(file, "w") {|fh| fh << content }
  end

	def assert_incompatible_encoding
		d = "\u{3042}\u{3044}".encode("utf-16le")
		assert_raises(Encoding::CompatibilityError) {yield d}
		m = Class.new {define_method(:to_path) {d}}
		assert_raises(Encoding::CompatibilityError) {yield m.new}
	end

  def utf8_file
    return @utf8file if defined? @utf8file
    @utf8file = make_tmp_filename("\u3066\u3059\u3068")
    make_file("foo", @utf8file)
    @utf8file
  end

	def setup
		@dir = Dir.mktmpdir("rubytest-file")
		File.chown(-1, Process.gid, @dir)
	end

	def teardown
		GC.start
		FileUtils.remove_entry_secure @dir
	end

	# Tests copied from https://searchcode.com/codesearch/view/12785140/
	def test_it_creates_basename_correctly
		assert_equal FasterPath.basename('/home/gumby/work/ruby.rb'),        'ruby.rb'
		assert_equal FasterPath.basename('/home/gumby/work/ruby.rb', '.rb'), 'ruby'
		assert_equal FasterPath.basename('/home/gumby/work/ruby.rb', '.*'),  'ruby'
		assert_equal FasterPath.basename('ruby.rb',  '.*'),  'ruby'
		assert_equal FasterPath.basename('/ruby.rb', '.*'),  'ruby'
		assert_equal FasterPath.basename('ruby.rbx', '.*'),  'ruby'
		assert_equal FasterPath.basename('ruby.rbx', '.rb'), 'ruby.rbx'
		assert_equal FasterPath.basename('ruby.rb', ''),      'ruby.rb'
		assert_equal FasterPath.basename('ruby.rbx', '.rb*'), 'ruby.rbx'
		assert_equal FasterPath.basename('ruby.rbx'), 'ruby.rbx'

		# Try some extensions w/o a '.'
		assert_equal FasterPath.basename('ruby.rbx', 'rbx'), 'ruby.'
		assert_equal FasterPath.basename('ruby.rbx', 'x'),   'ruby.rb'
		assert_equal FasterPath.basename('ruby.rbx', '*'),   'ruby.rbx'

		# A couple of regressions:
		assert_equal FasterPath.basename('', ''),           ''
		assert_equal FasterPath.basename('/'),              '/'
		assert_equal FasterPath.basename('//'),             '/'
		assert_equal FasterPath.basename('//dir///base//'), 'base'
		assert_equal FasterPath.basename('.x', '.x'), '.x'

		# returns the basename for unix suffix
		assert_equal FasterPath.basename("bar.c", ".c"), "bar"
		assert_equal FasterPath.basename("bar.txt", ".txt"), "bar"
		assert_equal FasterPath.basename("/bar.txt", ".txt"), "bar"
		assert_equal FasterPath.basename("/foo/bar.txt", ".txt"), "bar"
		assert_equal FasterPath.basename("bar.txt", ".exe"), "bar.txt"
		assert_equal FasterPath.basename("bar.txt.exe", ".exe"), "bar.txt"
		assert_equal FasterPath.basename("bar.txt.exe", ".txt"), "bar.txt.exe"
		assert_equal FasterPath.basename("bar.txt", ".*"), "bar"
		assert_equal FasterPath.basename("bar.txt.exe", ".*"), "bar.txt"
		assert_equal FasterPath.basename("bar.txt.exe", ".txt.exe"), "bar"
	end

	def test_it_does_the_same_as_file_basename
		assert_equal File.basename('/home/gumby/work/ruby.rb'),        'ruby.rb'
		assert_equal File.basename('/home/gumby/work/ruby.rb', '.rb'), 'ruby'
		assert_equal File.basename('/home/gumby/work/ruby.rb', '.*'),  'ruby'
		assert_equal File.basename('ruby.rb',  '.*'),  'ruby'
		assert_equal File.basename('/ruby.rb', '.*'),  'ruby'
		assert_equal File.basename('ruby.rbx', '.*'),  'ruby'
		assert_equal File.basename('ruby.rbx', '.rb'), 'ruby.rbx'
		assert_equal File.basename('ruby.rb', ''),      'ruby.rb'
		assert_equal File.basename('ruby.rbx', '.rb*'), 'ruby.rbx'
		assert_equal File.basename('ruby.rbx'), 'ruby.rbx'

		# Try some extensions w/o a '.'
		assert_equal File.basename('ruby.rbx', 'rbx'), 'ruby.'
		assert_equal File.basename('ruby.rbx', 'x'),   'ruby.rb'
		assert_equal File.basename('ruby.rbx', '*'),   'ruby.rbx'

		# A couple of regressions:
		assert_equal File.basename('', ''),           ''
		assert_equal File.basename('/'),              '/'
		assert_equal File.basename('//'),             '/'
		assert_equal File.basename('//dir///base//'), 'base'
	end

	def test_basename
		assert_equal(File.basename(regular_file).sub(/\.test$/, ""), File.basename(regular_file, ".test"))
		assert_equal(File.basename(utf8_file).sub(/\.test$/, ""), File.basename(utf8_file, ".test"))
		assert_equal("", s = File.basename(""))
		refute_predicate(s, :frozen?, '[ruby-core:24199]')
		assert_equal("foo", s = File.basename("foo"))
		refute_predicate(s, :frozen?, '[ruby-core:24199]')
		assert_equal("foo", File.basename("foo", ".ext"))
		assert_equal("foo", File.basename("foo.ext", ".ext"))
		assert_equal("foo", File.basename("foo.ext", ".*"))
		if NTFS
			[regular_file, utf8_file].each do |file|
				basename = File.basename(file)
				assert_equal(basename, File.basename(file + " "))
				assert_equal(basename, File.basename(file + "."))
				assert_equal(basename, File.basename(file + "::$DATA"))
				basename.chomp!(".test")
				assert_equal(basename, File.basename(file + " ", ".test"))
				assert_equal(basename, File.basename(file + ".", ".test"))
				assert_equal(basename, File.basename(file + "::$DATA", ".test"))
				assert_equal(basename, File.basename(file + " ", ".*"))
				assert_equal(basename, File.basename(file + ".", ".*"))
				assert_equal(basename, File.basename(file + "::$DATA", ".*"))
			end
		else
			[regular_file, utf8_file].each do |file|
				basename = File.basename(file)
				assert_equal(basename + " ", File.basename(file + " "))
				assert_equal(basename + ".", File.basename(file + "."))
				assert_equal(basename + "::$DATA", File.basename(file + "::$DATA"))
				assert_equal(basename + " ", File.basename(file + " ", ".test"))
				assert_equal(basename + ".", File.basename(file + ".", ".test"))
				assert_equal(basename + "::$DATA", File.basename(file + "::$DATA", ".test"))
				assert_equal(basename, File.basename(file + ".", ".*"))
				basename.chomp!(".test")
				assert_equal(basename, File.basename(file + " ", ".*"))
				assert_equal(basename, File.basename(file + "::$DATA", ".*"))
			end
		end
		if File::ALT_SEPARATOR == '\\'
			a = "foo/\225\\\\"
			[%W"cp437 \225", %W"cp932 \225\\"].each do |cp, expected|
				assert_equal(expected.force_encoding(cp), File.basename(a.dup.force_encoding(cp)), cp)
			end
		end
		assert_incompatible_encoding {|d| File.basename(d)}
		assert_incompatible_encoding {|d| File.basename(d, ".*")}
		assert_raises(Encoding::CompatibilityError) {File.basename("foo.ext", ".*".encode("utf-16le"))}
		s = "foo\x93_a".force_encoding("cp932")
		assert_equal(s, File.basename(s, "_a"))
		s = "\u4032.\u3024"
		assert_equal(s, File.basename(s, ".\x95\\".force_encoding("cp932")))
	end
end
