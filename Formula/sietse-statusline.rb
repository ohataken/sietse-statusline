class SietseStatusline < Formula
  desc "my little statusline for Claude Code."
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.10.0"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "7566946470f71418be3640b8a059029c05e299251d2e26e1a15cc348d54eb719"
  end

  def install
    bin.install "sietse-statusline"
  end
end
