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
  end

  describe "test output types are the same" do
    def test_direct_method_calls
      # not
      # * basename
      # * dirname
      # * extname
      # because of their definition belonging to `File`
      mthds = %i[
        absolute?
        add_trailing_separator
        chop_basename
        directory?
        has_trailing_separator?
        plus
        relative?
      ]

      mthds.each do |m|
        assert_same_output_kind(m)
      end

      mthds.each do |m|
        assert_same_output_kind(m, (), false)
      end
    end

    def test_compat_methods
      mthds = [
        [:children, :children_compat],
        [:entries, :entries_compat]
      ]

      mthds.each do |m|
        assert_same_output_kind(*m)
      end

      mthds.each do |m|
        assert_same_output_kind(*m, false)
      end
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
