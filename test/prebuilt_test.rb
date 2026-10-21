require 'test_helper'
require 'faster_path/build_task'
require 'faster_path/prebuilt'
require 'tmpdir'

# The prebuilt extension of a release, as the release workflow packs it and
# `gem install` unpacks it
class PrebuiltTest < Minitest::Test
  LINUX = { 'host_os' => 'linux', 'target_cpu' => 'x86_64', 'ruby_version' => '3.4.0' }.freeze
  MACOS = { 'host_os' => 'darwin24', 'target_cpu' => 'arm64', 'ruby_version' => '3.4.0' }.freeze
  WINDOWS = { 'host_os' => 'mingw32', 'target_cpu' => 'x64', 'ruby_version' => '3.4.0' }.freeze

  def test_the_asset_is_named_after_the_gem_version_ruby_minor_version_os_and_cpu
    assert_equal 'faster_path-0.5.0-ruby34-linux-x86_64.tar.gz', FasterPath::Prebuilt.asset_name(LINUX, '0.5.0')
    assert_equal 'faster_path-0.5.0-ruby34-darwin-arm64.tar.gz', FasterPath::Prebuilt.asset_name(MACOS, '0.5.0')
    assert_equal 'faster_path-0.5.0-ruby34-windows-x86_64.tar.gz', FasterPath::Prebuilt.asset_name(WINDOWS, '0.5.0')
    assert_equal 'faster_path-0.5.0-ruby33-linux-arm64.tar.gz',
                 FasterPath::Prebuilt.asset_name(LINUX.merge('target_cpu' => 'aarch64', 'ruby_version' => '3.3.0'), '0.5.0')
  end

  def test_the_asset_for_this_ruby_uses_the_gem_version
    assert_match(/\Afaster_path-#{Regexp.escape(FasterPath::VERSION)}-ruby\d\d-[a-z]+-[a-z0-9_]+\.tar\.gz\z/,
                 FasterPath::Prebuilt.asset_name)
  end

  def test_packing_and_unpacking_round_trips_the_built_library
    Dir.mktmpdir do |dir|
      tarball = FasterPath::Prebuilt.package(FasterPath::FFI_LIBRARY, dir)

      assert_equal File.join(dir, FasterPath::Prebuilt.asset_name), tarball
      assert_match(/\A\h{64}  #{Regexp.escape(FasterPath::Prebuilt.asset_name)}\n\z/, File.read("#{tarball}.sha256"))

      data = File.binread(tarball)
      FasterPath::Prebuilt.verify(data, File.read("#{tarball}.sha256"), 'asset')
      installed = FasterPath::Prebuilt.unpack(data, File.join(dir, 'gem'))

      assert_equal File.join(dir, 'gem', 'lib', 'faster_path', File.basename(FasterPath::FFI_LIBRARY)), installed
      assert_equal File.binread(FasterPath::FFI_LIBRARY), File.binread(installed)
    end
  end

  def test_a_wrong_checksum_is_refused
    error = assert_raises(RuntimeError) { FasterPath::Prebuilt.verify('data', "#{'0' * 64}  asset\n", 'asset') }
    assert_match(/does not match its checksum/, error.message)
  end

  def test_a_tarball_without_the_library_is_refused
    data = Zlib.gzip(Gem::Package::TarWriter.new(StringIO.new).then { |t| t.add_file_simple('other', 0o644, 0) { |io| io }; t.close; '' })
    assert_raises(RuntimeError) { FasterPath::Prebuilt.unpack(data, Dir.mktmpdir) }
  end

  def test_cargo_is_found_on_the_path
    assert FasterPath::BuildTask.cargo?

    original = ENV['PATH']
    ENV['PATH'] = Dir.mktmpdir
    refute FasterPath::BuildTask.cargo?
  ensure
    ENV['PATH'] = original
  end
end
