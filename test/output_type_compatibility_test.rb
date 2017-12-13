require 'test_helper'

class OutputTypeCompatibilityTest < Minitest::Test
  ::Minitest::Assertions.module_eval do
    def assert_same_output_kind(mthd, opt_mthd = nil)
      a = Pathname.instance_method(mthd).arity
      args = []
      a.abs.times do
        args << "a"
      end

      b = FasterPath.method(opt_mthd || mthd).arity
      bargs = []
      b.abs.times do
        bargs << "a"
      end

      pth = Pathname.new("a").send(mthd, *args)
      fpth = FasterPath.send(opt_mthd || mthd, *bargs)
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
    end

    # Do NOT remove; waiting for fix in ruru
    # def test_compat_methods
    #   mthds = [
    #     [:children, :children_compat],
    #     [:entries, :entries_compat]
    #   ]

    #   mthds.each do |m|
    #     assert_same_output_kind(*m)
    #   end
    # end
  end

  describe "test output types are not the same" do
    def pth
      "."
    end

    def test_children
      refute_equal Pathname.new(pth).children.first.class,
        FasterPathname::Public.send(:children, pth).first.class
    end

    def test_entries
      refute_equal Pathname.new(pth).entries.first.class,
        FasterPathname::Public.send(:entries, pth).first.class
    end
  end
end
