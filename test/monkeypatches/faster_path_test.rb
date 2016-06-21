require 'test_helper'
require 'method_source'

if ENV['TEST_MONKEYPATCHES'].to_s['true']
  require 'faster_path/optional/monkeypatches'
  FasterPath.sledgehammer_everything!
end

class MonkeyPatchesTest < Minitest::Test
  def setup
    @path = Pathname.new(".")
  end

  if ENV['TEST_MONKEYPATCHES'].to_s['true']
    def test_it_redefines_absolute?
      assert @path.method(:absolute?).source[/FasterPath/]
    end

    def test_it_redefines_directory?
      @path.method(:directory?).source[/FasterPath/]
    end

    def test_it_redefines_chop_basename
      assert @path.method(:chop_basename).source[/FasterPath/] 
    end

    def test_it_redefines_relative?
      assert @path.method(:relative?).source[/FasterPath/] 
    end

    def test_it_redefines_add_trailing_separator
      assert @path.method(:add_trailing_separator).source[/FasterPath/]
    end

    def test_it_redefines_has_trailing_separator
      assert @path.method(:has_trailing_separator?).source[/FasterPath/]
    end
  else
    def test_it_redefines_absolute?
      refute @path.method(:absolute?).source[/FasterPath/]
    end

    def test_it_does_not_redefine_directory?
      assert_raises(MethodSource::SourceNotFoundError){ @path.method(:directory?).source[/FasterPath/] }
    end

    def test_it_redefines_chop_basename
      refute @path.method(:chop_basename).source[/FasterPath/] 
    end

    def test_it_redefines_relative?
      refute @path.method(:relative?).source[/FasterPath/] 
    end

    def test_it_redefines_add_trailing_separator
      refute @path.method(:add_trailing_separator).source[/FasterPath/]
    end

    def test_it_redefines_has_trailing_separator
      refute @path.method(:has_trailing_separator?).source[/FasterPath/]
    end
  end
end
