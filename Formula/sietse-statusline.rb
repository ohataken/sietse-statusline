class SietseStatusline < Formula
  desc "My little utilities for Claude Code / Codex hooks"
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.1.0"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "a48025a6ebcb1e3ffa94b0cfb765ccc9902350be2d98cf860b60522a8c591f72"
  end

  def install
    bin.install "sietse-statusline"
  end
end
