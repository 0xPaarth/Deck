#!/usr/bin/env bash
# Deck Release Build Script
# Usage: ./scripts/build-release.sh [version]
# Example: ./scripts/build-release.sh v0.1.0

set -euo pipefail

VERSION="${1:-"v0.1.0"}"
VERSION_NUM="${VERSION#v}"
REPO="deck/deck"
WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RELEASE_DIR="$WORKSPACE_ROOT/target/release-artifacts"

echo "🎴 Building Deck $VERSION"
echo "========================"

# Determine current platform
detect_platform() {
  case "$(uname -s)" in
    Darwin) echo "darwin" ;;
    Linux)  echo "linux" ;;
    MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
    *)      echo "linux" ;;
  esac
}

detect_arch() {
  case "$(uname -m)" in
    x86_64|amd64) echo "x64" ;;
    arm64|aarch64) echo "arm64" ;;
    *) echo "x64" ;;
  esac
}

PLATFORM=$(detect_platform)
ARCH=$(detect_arch)
PLATFORM_TAG="$PLATFORM-$ARCH"

echo ""
echo "Platform: $PLATFORM ($ARCH)"
echo ""

# Clean and create release dir
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

# Build release binaries
echo "🔨 Building release binaries..."
cd "$WORKSPACE_ROOT"
cargo build --release --workspace

echo ""
echo "✅ Binaries built:"
ls -lh "$WORKSPACE_ROOT/target/release/"{deck,deck-tui} 2>/dev/null || true

# Package binaries
echo ""
echo "📦 Packaging..."

PKG_DIR="$RELEASE_DIR/$PLATFORM_TAG"
mkdir -p "$PKG_DIR"

# Copy binaries
if [ "$PLATFORM" = "windows" ]; then
  cp "$WORKSPACE_ROOT/target/release/deck.exe" "$PKG_DIR/"
  cp "$WORKSPACE_ROOT/target/release/deck-tui.exe" "$PKG_DIR/"
  cp "$WORKSPACE_ROOT/target/release/"*.dll "$PKG_DIR/" 2>/dev/null || true
  cd "$RELEASE_DIR"
  zip -r "$PLATFORM_TAG.zip" "$PLATFORM_TAG"
  echo "  Created: $PLATFORM_TAG.zip"
else
  cp "$WORKSPACE_ROOT/target/release/deck" "$PKG_DIR/"
  cp "$WORKSPACE_ROOT/target/release/deck-tui" "$PKG_DIR/"
  cd "$RELEASE_DIR"
  tar -czf "$PLATFORM_TAG.tar.gz" "$PLATFORM_TAG"
  echo "  Created: $PLATFORM_TAG.tar.gz"
fi

# Generate checksums
echo ""
echo "🔐 Generating checksums..."
cd "$RELEASE_DIR"
sha256sum *.tar.gz *.zip 2>/dev/null > checksums.txt || true
cat checksums.txt

# Show summary
echo ""
echo "📦 Release Artifacts"
echo "===================="
ls -lh "$RELEASE_DIR"

echo ""
echo "Next steps:"
echo "  1. Upload artifacts to GitHub Release: $REPO/releases/tag/$VERSION"
echo "  2. Publish to NPM: cd packages/npm && npm publish"
echo ""
echo "Artifacts in: $RELEASE_DIR"
