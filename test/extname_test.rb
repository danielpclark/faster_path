require 'test_helper'

class ExtnameTest < Minitest::Test
  def test_nil_inputs
    assert_nil FasterPath.extname(nil)
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
    result_pair = ->str{
      [
          File.send(:extname, str),
          FasterPath.extname(str)
      ]
    }
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
