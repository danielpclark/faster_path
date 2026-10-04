require "test_helper"

class ArchBitsTest < Minitest::Test
  def test_ruby_arch_bits_is_32_or_64
    assert_includes [32, 64], FasterPath.ruby_arch_bits
  end

  def test_rust_arch_bits_is_32_or_64
    assert_includes [32, 64], FasterPath.rust_arch_bits
  end

  def test_the_extension_is_built_for_the_running_ruby
    # `1.size` used to report 32 for a 64-bit Ruby on Windows, where a C `long` is 4 bytes
    assert_equal FasterPath.rust_arch_bits, FasterPath.ruby_arch_bits
  end
end
