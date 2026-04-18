class SietseStatusline < Formula
  desc "My little utilities for Claude Code / Codex hooks"
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.6.0"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "f1672a1bb0b1c41fcb762e379acfe362f5138043d734e4a2680e5dd96b3d0e33"
  end

  def install
    bin.install "sietse-statusline"
  end
end
