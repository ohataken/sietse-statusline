class SietseStatusline < Formula
  desc "My little utilities for Claude Code / Codex hooks"
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.3.0"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "c8010b3398bee1d2bdf5a236458157a647dce127406a9ed7f5c04a31db243f3d"
  end

  def install
    bin.install "sietse-statusline"
  end
end
