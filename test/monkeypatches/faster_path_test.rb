require 'test_helper'

if ENV['TEST_MONKEYPATCHES'].to_s['true']
  require 'method_source'
  require 'faster_path/optional/monkeypatches'
  FasterPath.sledgehammer_everything!
end

class MonkeyPatchesTest < Minitest::Test
  def setup
    @path = Pathname.new("")
  end

  def test_it_redefines_absolute?
    assert @path.method(:absolute?).source[/FasterPath/]
  end

  def test_it_redefines_directory?
    assert @path.method(:directory?).source[/FasterPath/]
  end

  def test_it_redefines_chop_basename
    assert @path.method(:chop_basename).source[/FasterPath/] 
  end

  def test_it_redefines_relative?
    assert @path.method(:relative?).source[/FasterPath/] 
  end
end if ENV['TEST_MONKEYPATCHES'].to_s['true']
