class SietseStatusline < Formula
  desc "My little utilities for Claude Code / Codex hooks"
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.5.0"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "fc6cfb7adfc36729d47ad8ee61b657c78df9fae31cac092d37ad67a141d96aa3"
  end

  def install
    bin.install "sietse-statusline"
  end
end
