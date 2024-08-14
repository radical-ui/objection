class Objection < Formula
  desc "Build server-first, highly-interactive, and beautiful web applications in Rust"
  homepage "https://github.com/radical-ui/objection"
  url "https://github.com/radical-ui/objection/archive/refs/tags/VERSION.tar.gz"
  version "VERSION"
  sha256 "SHA256_HASH"
  license "MIT"

  depends_on "rust" => :build

  def install
    ENV["CARGO_NET_GIT_FETCH_WITH_CLI"] = "true"
    system "cargo", "build", "--release", "--bin", "objection"
    bin.install "target/release/objection"
  end

  test do
    output = shell_output("#{bin}/objection --version")
    assert_match "objection_cli VERSION", output
  end
end
