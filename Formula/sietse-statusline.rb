class SietseStatusline < Formula
  desc "my little statusline for Claude Code."
  homepage "https://github.com/ohataken/sietse-statusline"
  version "0.11.0"

  on_arm do
    url "https://github.com/ohataken/sietse-statusline/releases/download/v#{version}/sietse-statusline-aarch64-apple-darwin.tar.gz"
    sha256 "71086442f4a920b689ecce67a7fde5ca4a1178b7c287ecb2f37067fa43a37cde"
  end

  def install
    bin.install "sietse-statusline"
  end
end
