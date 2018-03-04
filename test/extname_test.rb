require 'test_helper'

class ExtnameTest < Minitest::Test
  def setup
    @dir = Dir.mktmpdir("rubytest-file")
    File.chown(-1, Process.gid, @dir)
  end

  def teardown
    GC.start
    FileUtils.remove_entry_secure @dir
  end

  def test_extname_official
    assert_equal(".test", FasterPath.extname(regular_file))
    assert_equal(".test", FasterPath.extname(utf8_file))
    prefixes = ["", "/", ".", "/.", "bar/.", "/bar/."]
    infixes = ["", " ", "."]
    infixes2 = infixes + [".ext "]
    appendixes = [""]
    if NTFS
      appendixes << " " << "." << "::$DATA" << "::$DATA.bar"
    end
    prefixes.each do |prefix|
      appendixes.each do |appendix|
        infixes.each do |infix|
          path = "#{prefix}foo#{infix}#{appendix}"
          assert_equal("", FasterPath.extname(path), "FasterPath.extname(#{path.inspect})")
        end
        infixes2.each do |infix|
          path = "#{prefix}foo#{infix}.ext#{appendix}"
          assert_equal(".ext", FasterPath.extname(path), "FasterPath.extname(#{path.inspect})")
        end
      end
    end
    # bug3175 = '[ruby-core:29627]'
    # assert_equal(".rb", FasterPath.extname("/tmp//bla.rb"), bug3175)
    assert_incompatible_encoding {|d| FasterPath.extname(d)} if ENV['ENCODING'] == true
  end

  def test_extname
    assert_equal ".rb", FasterPath.extname("foo.rb")
    assert_equal ".rb", FasterPath.extname("/foo/bar.rb")
    assert_equal ".c", FasterPath.extname("/foo.rb/bar.c")
    assert_equal "", FasterPath.extname("bar")
    assert_equal "", FasterPath.extname(".bashrc")
    assert_equal "", FasterPath.extname("./foo.bar/baz")
    assert_equal ".conf", FasterPath.extname(".app.conf")
  end

  def test_extname_edge_cases
    assert_equal "", FasterPath.extname("")
    assert_equal "", FasterPath.extname(".")
    assert_equal "", FasterPath.extname("/")
    assert_equal "", FasterPath.extname("/.")
    assert_equal "", FasterPath.extname("..")
    assert_equal "", FasterPath.extname("...")
    assert_equal "", FasterPath.extname("....")
    assert_equal "", FasterPath.extname(".foo.")
    assert_equal "", FasterPath.extname("foo.")
    assert_equal "", FasterPath.extname("..foo")
  end

  def test_substitutability_of_rust_and_ruby_impls
    result_pair = lambda do |str|
      [
        File.send(:extname, str),
        FasterPath.extname(str)
      ]
    end
    assert_equal( *result_pair.("foo.rb")                    )
    assert_equal( *result_pair.("/foo/bar.rb")               )
    assert_equal( *result_pair.("/foo.rb/bar.c")             )
    assert_equal( *result_pair.("bar")                       )
    assert_equal( *result_pair.(".bashrc")                   )
    assert_equal( *result_pair.("./foo.bar/baz")             )
    assert_equal( *result_pair.(".app.conf")                 )
    assert_equal( *result_pair.("")                          )
    assert_equal( *result_pair.(".")                         )
    assert_equal( *result_pair.("/")                         )
    assert_equal( *result_pair.("/.")                        )
    assert_equal( *result_pair.("..")                        )
    assert_equal( *result_pair.("...")                       )
    assert_equal( *result_pair.("....")                      )
    assert_equal( *result_pair.(".foo.")                     )
    assert_equal( *result_pair.("foo.")                      )
    assert_equal( *result_pair.("foo.rb/")                   )
    assert_equal( *result_pair.("foo.rb//")                  )
  end
end
