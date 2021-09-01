# frozen_string_literal: true

require "bundler/setup"

require 'rspec/retry'

require 'pathname'
require 'tempfile'
require 'stringio'
require 'securerandom'
require 'timeout'

require "dead_end"

require "cutlass"


def root_dir
  Pathname(__dir__).join("..")
end

def fixtures_dir
  root_dir.join("fixtures").expand_path
end

PROCFILE_BUILDPACK = Cutlass::LocalBuildpack.new(directory: root_dir)

Cutlass.config do |config|
  config.default_builder = "heroku/buildpacks:20"

  # Where do your test fixtures live?
  config.default_repo_dirs = [fixtures_dir]

  # Where does your buildpack live?
  # Can be a directory or a Cutlass:LocalBuildpack instance
  config.default_buildpack_paths = [PROCFILE_BUILDPACK]
end

RSpec.configure do |config|
  # Enable flags like --only-failures and --next-failure
  config.example_status_persistence_file_path = ".rspec_status"
  config.display_try_failure_messages = true
  config.verbose_retry       = true # show retry status in spec process
  config.default_retry_count = 2 if ENV['IS_RUNNING_ON_CI'] # retry all tests that fail again

  config.expect_with :rspec do |c|
    c.syntax = :expect
  end

  ## ENV var set and persist
  config.before(:suite) do
    LOAD_PATH_DUP = $LOAD_PATH.dup

    Cutlass::CleanTestEnv.record
  end

  ## ENV var check
  config.after(:suite) do
    Cutlass::CleanTestEnv.check
  end
end

def run!(cmd)
  out = `#{cmd}`
  raise "Error running #{cmd}, output: #{out}" unless $?.success?
  out
end

