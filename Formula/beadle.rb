# Homebrew formula for beadle
# Updated automatically by the release workflow when signing is enabled
# macOS only (arm64). Linux users: download from GitHub releases.

class Beadle < Formula
  desc "Intent-aligned issue triage and dashboard for orchestrator targets"
  homepage "https://github.com/arcavenae/beadle"
  url "https://github.com/arcavenae/beadle/releases/download/TAG_PLACEHOLDER/beadle-darwin-arm64"
  version "VERSION_PLACEHOLDER"
  sha256 "SHA256_ARM64_PLACEHOLDER"
  license "MIT"

  def install
    bin.install "beadle-darwin-arm64" => "beadle"
  end

  test do
    assert_match "beadle", shell_output("#{bin}/beadle --version 2>&1")
  end
end
