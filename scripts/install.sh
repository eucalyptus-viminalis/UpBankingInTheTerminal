#!/usr/bin/env bash
set -euo pipefail

REPO="eucalyptus-viminalis/UpBankingInTheTerminal"
BINARY="upbank"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BOLD='\033[1m'
NC='\033[0m'

info() { echo -e "${BOLD}${GREEN}==>${NC} ${BOLD}$1${NC}"; }
warn() { echo -e "${YELLOW}warning:${NC} $1"; }
error() { echo -e "${RED}error:${NC} $1" >&2; exit 1; }

# Detect OS
OS="$(uname -s)"
case "$OS" in
  Linux*)  PLATFORM="linux-gnu" ;;
  Darwin*) PLATFORM="apple-darwin" ;;
  *)       error "Unsupported operating system: $OS" ;;
esac

# Detect architecture
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64|amd64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *)             error "Unsupported architecture: $ARCH" ;;
esac

# Windows/Linux ARM not supported yet
if [ "$PLATFORM" = "linux-gnu" ] && [ "$ARCH" = "aarch64" ]; then
  error "Linux ARM64 binaries are not available yet. You can build from source: cargo install --git https://github.com/$REPO"
fi

ASSET_NAME="upbank-${ARCH}-${PLATFORM}"
ARCHIVE="${ASSET_NAME}.tar.gz"

# Get latest release tag
info "Fetching latest release..."
if command -v gh &>/dev/null; then
  TAG="$(gh release view --repo "$REPO" --json tagName -q '.tagName' 2>/dev/null || true)"
fi

if [ -z "${TAG:-}" ]; then
  TAG="$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | head -1 | cut -d'"' -f4)"
fi

if [ -z "${TAG:-}" ]; then
  error "Could not determine latest release. Check https://github.com/$REPO/releases"
fi

info "Installing $BINARY $TAG ($ARCH $PLATFORM)..."

# Download
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$TAG/$ARCHIVE"
TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

info "Downloading $ARCHIVE..."
curl -fsSL "$DOWNLOAD_URL" -o "$TMPDIR/$ARCHIVE" || error "Download failed. Check that the release exists: $DOWNLOAD_URL"

# Extract
info "Extracting..."
tar xzf "$TMPDIR/$ARCHIVE" -C "$TMPDIR"

# Install
if [ -w "$INSTALL_DIR" ]; then
  mv "$TMPDIR/$BINARY" "$INSTALL_DIR/$BINARY"
else
  info "Installing to $INSTALL_DIR (requires sudo)..."
  sudo mv "$TMPDIR/$BINARY" "$INSTALL_DIR/$BINARY"
fi

chmod +x "$INSTALL_DIR/$BINARY"

info "Installed $BINARY to $INSTALL_DIR/$BINARY"
echo ""
echo -e "  Run ${BOLD}$BINARY --help${NC} to get started."
echo -e "  Set your token with ${BOLD}$BINARY config set-token <your-token>${NC}"
echo ""
