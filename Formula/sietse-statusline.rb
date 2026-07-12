class SietseStatusline < Formula
  desc "My little utilities for Claude Code / Codex hooks"
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.8.0"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "812027c90e06d2496d959e72c429352c77b6a17b38737f6ee8e8876399285591"
  end

  def install
    bin.install "sietse-statusline"
  end
end
