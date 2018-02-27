require 'test_helper'

class OutputTypeCompatibilityTest < Minitest::Test
  ::Minitest::Assertions.module_eval do
    # rubocop:disable Metrics/CyclomaticComplexity
    # rubocop:disable Metrics/PerceivedComplexity
    # rubocop:disable Metrics/MethodLength
    def assert_same_output_kind(mthd, opt_mthd = nil, valid=true)
      vpath = valid ? "." : "a"
      a = Pathname.instance_method(mthd).arity
      args = []
      a.abs.times do
        args << vpath
      end

      b = FasterPath.method(opt_mthd || mthd).arity
      bargs = []
      b.abs.times do
        bargs << vpath
      end

      pathname_raised = false
      fasterpath_raised = false
      if !valid
        begin
          Pathname.new(vpath).send(mthd, *args)
        rescue
          pathname_raised = true
        end
        begin
          FasterPath.send(opt_mthd || mthd, *bargs)
        rescue
          fasterpath_raised = true
        end

        if pathname_raised && fasterpath_raised
          return
        elsif pathname_raised
          raise "Only Pathname raised when invalid path"
        elsif fasterpath_raised
          raise "Only FasterPath raised when invalid path"
        end
      else
        pth = Pathname.new(vpath).send(mthd, *args)
        fpth = FasterPath.send(opt_mthd || mthd, *bargs)
      end

      assert_equal pth.is_a?(Array), fpth.is_a?(Array)
      if pth.is_a?(Array) && fpth.is_a?(Array)
        assert_equal pth.first.class, fpth.first.class,
          "Array output kind not equal for method '#{mthd}':\nExpected: #{pth.inspect}\nGot: #{fpth.inspect}"
      else
        assert_kind_of pth.class, fpth,
          "Output kind not equal for method '#{mthd}':\nExpected: #{pth.inspect}\nGot: #{fpth.inspect}"
      end
    end

    def assert_happy_path(m)
      assert_same_output_kind(m)
    end

    def assert_unhappy_path(m)
      assert_same_output_kind(m, (), false)
    end

    def assert_happy_compat_path(m, c)
      assert_same_output_kind(m, c)
    end

    def assert_unhappy_compat_path(m, c)
      assert_same_output_kind(m, c, false)
    end

  end

  describe "TestForIdenticalOutput" do
    def test_absolute?
      assert_happy_path :absolute?
      assert_unhappy_path :absolute?
    end

    def test_add_trailing_separator
      assert_happy_path :add_trailing_separator
      assert_unhappy_path :add_trailing_separator
    end

    def test_chop_basename
      assert_happy_path :chop_basename
      assert_unhappy_path :chop_basename
    end

    def test_directory?
      assert_happy_path :directory?
      assert_unhappy_path :directory?
    end

    def test_has_trailing_separator?
      assert_happy_path :has_trailing_separator?
      assert_unhappy_path :has_trailing_separator?
    end

    def test_join
      assert_happy_path :join
      assert_unhappy_path :join
    end

    def test_plus
      assert_happy_path :plus
      assert_unhappy_path :plus
    end

    def test_relative?
      assert_happy_path :relative?
      assert_unhappy_path :relative?
    end

    def test_children_compat
      assert_happy_compat_path :children, :children_compat
      assert_unhappy_compat_path :children, :children_compat
    end

    def test_entries_compat
      assert_happy_compat_path :entries, :entries_compat
      assert_unhappy_compat_path :entries, :entries_compat
    end
  end

  describe "test output types are not the same" do
    def pth
      "."
    end

    def test_children
      refute_equal Pathname.new(pth).children.first.class,
        FasterPath.children(pth).first.class
    end

    def test_entries
      refute_equal Pathname.new(pth).entries.first.class,
        FasterPath.entries(pth).first.class
    end
  end
end
