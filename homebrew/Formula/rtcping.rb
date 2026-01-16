class Rtcping < Formula
  desc "TCP latency measurement tool similar to ping but for TCP connections"
  homepage "https://github.com/cuteribs/rtcping"
  version "0.1.0"
  license "MIT"

  if OS.mac?
    if Hardware::CPU.intel?
      url "https://github.com/cuteribs/rtcping/releases/download/v#{version}/rtcping-macos-x86_64"
      sha256 "PLACEHOLDER_INTEL_SHA256"
    else
      url "https://github.com/cuteribs/rtcping/releases/download/v#{version}/rtcping-macos-aarch64"
      sha256 "PLACEHOLDER_ARM_SHA256"
    end
  elsif OS.linux?
    if Hardware::CPU.intel?
      url "https://github.com/cuteribs/rtcping/releases/download/v#{version}/rtcping-linux-x86_64"
      sha256 "PLACEHOLDER_LINUX_SHA256"
    else
      url "https://github.com/cuteribs/rtcping/releases/download/v#{version}/rtcping-linux-aarch64"
      sha256 "PLACEHOLDER_LINUX_ARM_SHA256"
    end
  end

  def install
    bin.install Dir["*"].first => "rtcping"
  end

  test do
    system "#{bin}/rtcping", "--version"
  end
end
