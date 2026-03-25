class SietseStatusline < Formula
  desc "My little utilities for Claude Code / Codex hooks"
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.2.0"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "8e8a65faf4ffca5292b9e8899e2c606fac4d8a5b6a040e8c05c987dd1f1754fc"
  end

  def install
    bin.install "sietse-statusline"
  end
end
