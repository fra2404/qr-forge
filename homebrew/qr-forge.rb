class QrForge < Formula
  desc "Professional QR code generator built with Rust"
  homepage "https://github.com/fra2404/qr-forge"
  url "https://github.com/fra2404/qr-forge/archive/refs/tags/v1.0.4.tar.gz"
  sha256 "PLACEHOLDER_SHA256"  # This will be updated with the actual release
  license "MIT"
  head "https://github.com/fra2404/qr-forge.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test CLI functionality
    system bin/"qr-forge", "--url", "https://example.com", "--output", "test"
    assert_predicate testpath/"test.png", :exist?
    
    # Test help command
    assert_match "Professional QR code generator", shell_output("#{bin}/qr-forge --help")
  end
end
