# frozen_string_literal: true
require 'open3'
module Spec
  module Helpers
    def self.bang(method)
      define_method("#{method}!") do |*args, &blk|
        send(method, *args, &blk).tap do
          if exitstatus && exitstatus != 0
            error = out + "\n" + err
            error.strip!
            raise RuntimeError,
              "Invoking #{method}!(#{args.map(&:inspect).join(", ")}) failed:\n#{error}",
              caller.drop_while {|bt| bt.start_with?(__FILE__) }
          end
        end
      end
    end

    attr_reader :out, :err, :exitstatus

    def lib
      File.expand_path("../../../lib", __FILE__)
    end

    def bundle(cmd, options = {})
      with_sudo = options.delete(:sudo)
      sudo = with_sudo == :preserve_env ? "sudo -E" : "sudo" if with_sudo

      options["no-color"] = true unless options.key?("no-color") || cmd.to_s =~ /\A(e|ex|exe|exec|conf|confi|config)(\s|\z)/

      bundle_bin = options.delete("bundle_bin") || File.expand_path("../../../exe/bundle", __FILE__)

      if system_bundler = options.delete(:system_bundler)
        bundle_bin = "-S bundle"
      end

      requires = options.delete(:requires) || []
      #if artifice = options.delete(:artifice) { "fail" unless RSpec.current_example.metadata[:realworld] }
      #  requires << File.expand_path("../artifice/#{artifice}.rb", __FILE__)
      #end
      #requires << "support/hax"
      requires_str = requires.map {|r| "-r#{r}" }.join(" ")

      load_path = []
      load_path << lib unless system_bundler
      load_path << File.expand_path("../../../spec", __FILE__)
      load_path_str = "-I#{load_path.join(File::PATH_SEPARATOR)}"

      env = (options.delete(:env) || {}).map {|k, v| "#{k}='#{v}'" }.join(" ")
      env["PATH"].gsub!("#{Path.root}/exe", "") if env["PATH"] && system_bundler
      args = options.map do |k, v|
        v == true ? " --#{k}" : " --#{k} #{v}" if v
      end.join

      cmd = "#{env} #{sudo} #{Gem.ruby} #{load_path_str} #{requires_str} #{bundle_bin} #{cmd}#{args}"
      sys_exec(cmd) {|i, o, thr| yield i, o, thr if block_given? }
    end
    bang :bundle

    def sys_exec(cmd)
      Open3.popen3(cmd.to_s) do |stdin, stdout, stderr, wait_thr|
        yield stdin, stdout, wait_thr if block_given?
        stdin.close

        @exitstatus = wait_thr && wait_thr.value.exitstatus
        @out = Thread.new { stdout.read }.value.strip
        @err = Thread.new { stderr.read }.value.strip
      end

      (@all_output ||= String.new) << [
        "$ #{cmd.to_s.strip}",
        out,
        err,
        @exitstatus ? "# $? => #{@exitstatus}" : "",
        "\n",
      ].reject(&:empty?).join("\n")

      @out
    end
    bang :sys_exec
  end
end
