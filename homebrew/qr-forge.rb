class QrForge < Formula
  desc "Professional QR code generator built with Rust"
  homepage "https://github.com/fra2404/qr-forge"
  url "https://github.com/fra2404/qr-forge/archive/refs/tags/v1.0.4.tar.gz"
  sha256 "51ee65125930a7fc533e04c84a1442f1fd70a7f5bdecd39dde96ea6e591da941"  # This will be updated with the actual release
  license "MIT"
  head "https://github.com/fra2404/qr-forge.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test help command first
    help_output = shell_output("#{bin}/qr-forge --help")
    assert_match "QR Forge is a powerful QR code generator", help_output
    assert_match "--url", help_output
    assert_match "--format", help_output
    
    # Test version command
    version_output = shell_output("#{bin}/qr-forge --version")
    assert_match "qr-forge", version_output
    
    # Test CLI functionality with different formats
    system bin/"qr-forge", "--url", "https://example.com", "--output", "test_png", "--format", "png"
    assert_predicate testpath/"test_png.png", :exist?
    
    system bin/"qr-forge", "--url", "https://github.com", "--output", "test_svg", "--format", "svg"
    assert_predicate testpath/"test_svg.svg", :exist?
    
    # Test custom parameters
    system bin/"qr-forge", "--url", "https://rust-lang.org", "--output", "test_custom", 
           "--format", "png", "--size", "400", "--margin", "2"
    assert_predicate testpath/"test_custom.png", :exist?
    
    # Verify file sizes are reasonable (not empty, not too large)
    assert_operator File.size(testpath/"test_png.png"), :>, 100
    assert_operator File.size(testpath/"test_svg.svg"), :>, 100
    assert_operator File.size(testpath/"test_custom.png"), :>, 100
  end
end
