class SietseStatusline < Formula
  desc "My little utilities for Claude Code / Codex hooks"
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.0.0"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "0000000000000000000000000000000000000000000000000000000000000000"
  end

  def install
    bin.install "sietse-statusline"
  end
end
