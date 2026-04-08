#!/usr/bin/env bash
set -euo pipefail

BINARY="upbank"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

CONFIG_DIR="$HOME/.config/upbanking"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BOLD='\033[1m'
NC='\033[0m'

info() { echo -e "${BOLD}${GREEN}==>${NC} ${BOLD}$1${NC}"; }
warn() { echo -e "${YELLOW}warning:${NC} $1"; }

BINARY_PATH="$INSTALL_DIR/$BINARY"

if [ ! -f "$BINARY_PATH" ]; then
  warn "$BINARY not found at $BINARY_PATH"
  warn "If you installed to a custom location, run: INSTALL_DIR=/your/path bash uninstall.sh"
else
  info "Removing $BINARY_PATH..."
  if [ -w "$INSTALL_DIR" ]; then
    rm -f "$BINARY_PATH"
  else
    sudo rm -f "$BINARY_PATH"
  fi
  info "Removed binary."
fi

# Ask about config
if [ -n "$CONFIG_DIR" ] && [ -d "$CONFIG_DIR" ]; then
  echo ""
  echo -e "  Config directory found at ${BOLD}$CONFIG_DIR${NC}"
  echo -e "  This contains your saved API token."
  echo ""
  printf "  Remove config directory? (y/N) "
  read -r REPLY
  if [[ "$REPLY" =~ ^[Yy]$ ]]; then
    rm -rf "$CONFIG_DIR"
    info "Removed config directory."
  else
    info "Kept config directory (safe to remove later)."
  fi
fi

echo ""
info "Uninstall complete."
