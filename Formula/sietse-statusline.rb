class SietseStatusline < Formula
  desc "My little utilities for Claude Code / Codex hooks"
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.0.1"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "74d1841911f6d89ff216acd109510358b17bdff8b22a49ffae451d01431aa4a8"
  end

  def install
    bin.install "sietse-statusline"
  end
end
