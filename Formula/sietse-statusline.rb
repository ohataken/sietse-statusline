class SietseStatusline < Formula
  desc "My little utilities for Claude Code / Codex hooks"
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.9.0"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "6e3131a2ec6b0afbb4379c96979c7011d1fa5af5ba6f6578e1e5146cc54bd514"
  end

  def install
    bin.install "sietse-statusline"
  end
end
