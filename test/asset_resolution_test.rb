require 'test_helper'

class AssetResolutionTest < Minitest::Test
  def ar
    FasterPath::AssetResolution
  end

  def test_flow_of_asset_resolution
    file = ar.send(:lib_file)
    assert File.exist?(file), 'Rust asset not compiled!'
    assert ar.send(:rust?), 'Rust cargo not found!'
    assert_equal file, ar.verify!, 'Rust & asset file not found!'
    old_method = ar.method(:file?)
    ar.instance_eval "undef :file?; def file?; false end"
    refute ar.send(:file?), 'Rewritten :file? should have been false!'
    assert_equal "Building extension...\nCleaning up build...\nCompleted build!", ar.send(:compile!)
    assert_raises {ar.verify!} # Test path where cargo exists but asset won't compile
    ar.instance_eval 'undef :rust?; def rust?; false end'
    assert_raises {ar.verify!} # Test path where both cargo and asset file do not exist
    ar.instance_exec { undef :file? }
    ar.define_singleton_method(:file?, old_method)
    assert ar.send(:file?), ':file? method should have been restored!'
  end
end
