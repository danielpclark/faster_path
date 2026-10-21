require 'digest/sha2'
require 'fileutils'
require 'net/http'
require 'rbconfig'
require 'rubygems/package'
require 'stringio'
require 'uri'
require 'zlib'
require_relative 'version'

module FasterPath
  # The prebuilt extension that the GitHub release of each version carries
  # for a few platforms, so that the gem installs without Rust there:
  # `gem install` falls back to it when Cargo is not found (see ext/Rakefile),
  # and the release workflow builds it with `rake prebuilt`.
  module Prebuilt
    RELEASES = 'https://github.com/danielpclark/faster_path/releases/download'.freeze
    # Where the library sits in the tarball and, unpacked, in the gem
    LIBRARY_DIR = File.join('lib', 'faster_path').freeze
    MAX_REDIRECTS = 5

    # The asset for the running Ruby, e.g. "faster_path-0.5.0-ruby34-linux-x86_64.tar.gz":
    # the gem version, the Ruby minor version the library was built for (an
    # extension works with every patch release of it), the OS family and the CPU.
    def self.asset_name(rbconfig = RbConfig::CONFIG, version = FasterPath::VERSION)
      major, minor = rbconfig['ruby_version'].split('.')
      "faster_path-#{version}-ruby#{major}#{minor}-#{os(rbconfig)}-#{cpu(rbconfig)}.tar.gz"
    end

    def self.os(rbconfig)
      case rbconfig['host_os']
      when /darwin/ then 'darwin'
      when /mingw|mswin|cygwin/ then 'windows'
      else rbconfig['host_os'][/\A[a-z]+/]
      end
    end

    def self.cpu(rbconfig)
      { 'x64' => 'x86_64', 'amd64' => 'x86_64', 'aarch64' => 'arm64' }.fetch(rbconfig['target_cpu'], rbconfig['target_cpu'])
    end

    # Packs the built `library` as lib/faster_path/<file> into `dir`, with a
    # SHA-256 checksum file next to it, and returns the tarball's path.
    def self.package(library, dir, asset = asset_name)
      path = File.join(dir, asset)
      data = File.binread(library)
      File.open(path, 'wb') do |file|
        Zlib::GzipWriter.wrap(file) do |gz|
          Gem::Package::TarWriter.new(gz) do |tar|
            tar.add_file_simple(File.join(LIBRARY_DIR, File.basename(library)), 0o644, data.bytesize) { |io| io.write(data) }
          end
        end
      end
      File.write("#{path}.sha256", "#{Digest::SHA256.file(path).hexdigest}  #{asset}\n")
      path
    end

    # Downloads this platform's asset of this version's release, checks it
    # against its checksum file and unpacks the library into `project_dir`.
    # Returns the library's path, or nil when the release has no asset for
    # this platform. FASTER_PATH_PREBUILT_URL replaces the release's URL.
    def self.install(project_dir, base_url = ENV.fetch('FASTER_PATH_PREBUILT_URL', "#{RELEASES}/v#{FasterPath::VERSION}"))
      asset = asset_name
      tarball = fetch("#{base_url}/#{asset}")
      return nil unless tarball

      checksum = fetch("#{base_url}/#{asset}.sha256")
      raise "#{asset} has no checksum file at #{base_url}" unless checksum

      verify(tarball, checksum, asset)
      unpack(tarball, project_dir)
    end

    # The body at `url`, following redirects, or nil if it does not exist.
    def self.fetch(url, redirects = MAX_REDIRECTS)
      uri = URI(url)
      response = Net::HTTP.start(uri.host, uri.port, use_ssl: uri.scheme == 'https') { |http| http.get(uri.request_uri) }
      case response
      when Net::HTTPSuccess then response.body
      when Net::HTTPNotFound then nil
      when Net::HTTPRedirection
        raise "Too many redirects fetching #{url}" if redirects.zero?

        fetch(URI.join(url, response['Location']).to_s, redirects - 1)
      else raise "#{url}: #{response.code} #{response.message}"
      end
    end

    def self.verify(tarball, checksum, asset)
      expected = checksum.split.first.to_s.downcase
      actual = Digest::SHA256.hexdigest(tarball)
      raise "#{asset} does not match its checksum (expected #{expected}, got #{actual})" unless expected == actual
    end

    # Unpacks the library of the tarball `data` into `project_dir` and returns its path.
    def self.unpack(data, project_dir)
      installed = nil
      Zlib::GzipReader.wrap(StringIO.new(data)) do |gz|
        Gem::Package::TarReader.new(gz) do |tar|
          tar.each do |entry|
            next unless entry.file? && File.dirname(entry.full_name) == LIBRARY_DIR

            installed = write_library(project_dir, File.basename(entry.full_name), entry.read)
          end
        end
      end
      installed || raise("No #{LIBRARY_DIR} library in the tarball")
    end

    def self.write_library(project_dir, name, data)
      dir = File.join(project_dir, LIBRARY_DIR)
      FileUtils.mkdir_p(dir)
      path = File.join(dir, name)
      File.binwrite(path, data)
      path
    end
  end
end
