class SietseStatusline < Formula
  desc "My little utilities for Claude Code / Codex hooks"
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.7.0"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "9f93939ef9f34847c4e4a546ee6bc1c255ede37dbb0b027cdc957c9b524c3da7"
  end

  def install
    bin.install "sietse-statusline"
  end
end
