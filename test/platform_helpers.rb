require 'rbconfig'

module PlatformHelpers
  IS_WINDOWS = (RbConfig::CONFIG['host_os'] =~ /mswin|mingw|cygwin/)

  def platform_windows?
    IS_WINDOWS
  end
end
