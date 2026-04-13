class SietseStatusline < Formula
  desc "My little utilities for Claude Code / Codex hooks"
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.4.0"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "b1a4a27ceccdfd54287829766a3bccb970852cdd1920a94238871a87b2a64d4f"
  end

  def install
    bin.install "sietse-statusline"
  end
end
