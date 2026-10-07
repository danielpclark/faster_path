require 'test_helper'
require 'faster_path/build_task'

# How the extension is built and found
class BuildTest < Minitest::Test
  def test_the_extension_is_loaded_from_the_project_s_target_directory
    assert_equal FasterPath::BuildTask.built_library, FasterPath::FFI_LIBRARY
    assert_equal File.join(FasterPath::BuildTask::PROJECT_DIR, 'target', 'release'),
                 File.dirname(FasterPath::FFI_LIBRARY)
    assert File.exist?(FasterPath::FFI_LIBRARY)
  end

  def test_an_installed_gem_loads_the_library_from_lib
    installed = Rutie.new(:faster_path, lib_path: 'faster_path').ffi_library(File.join(FasterPath::BuildTask::PROJECT_DIR, 'lib'))

    assert_equal FasterPath::BuildTask::INSTALLED_LIBRARY_DIR, File.dirname(installed)
    assert_equal File.basename(FasterPath::FFI_LIBRARY), File.basename(installed)
  end

  def test_the_build_task_points_cargo_at_the_gem_s_manifest
    _env, *command = FasterPath::BuildTask.define.build_command

    assert_equal File.join(FasterPath::BuildTask::PROJECT_DIR, 'Cargo.toml'),
                 command[command.index('--manifest-path') + 1]
    assert_equal File.join(RbConfig::CONFIG['bindir'], RbConfig::CONFIG['ruby_install_name']),
                 ENV['RUBY'].sub(/#{Regexp.escape(RbConfig::CONFIG['EXEEXT'])}\z/, '')
  end
end
