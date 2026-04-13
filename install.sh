#!/bin/sh
set -e

REPO="yofabr/pkll"
BIN="pkll"

# Detect OS
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
  Linux)
    case "$ARCH" in
      x86_64)  TARGET="x86_64-unknown-linux-gnu" ;;
      aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    EXT="tar.xz"
    ;;
  Darwin)
    case "$ARCH" in
      x86_64)  TARGET="x86_64-apple-darwin" ;;
      arm64)   TARGET="aarch64-apple-darwin" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    EXT="tar.xz"
    ;;
  *)
    echo "Unsupported OS: $OS"
    echo "For Windows, run: irm https://github.com/$REPO/releases/latest/download/install.ps1 | iex"
    exit 1
    ;;
esac

# Get latest version tag
VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
  | grep '"tag_name"' \
  | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')

if [ -z "$VERSION" ]; then
  echo "Failed to fetch latest version. Check your internet connection."
  exit 1
fi

FILENAME="${BIN}-${TARGET}.${EXT}"
URL="https://github.com/${REPO}/releases/download/${VERSION}/${FILENAME}"

echo ""
echo "  pkll installer"
echo "  Version : $VERSION"
echo "  Target  : $TARGET"
echo ""

echo "Downloading $FILENAME..."
curl -fsSL --progress-bar "$URL" | tar -xJ --strip-components=1 "pkll-${TARGET}/pkll"

chmod +x "$BIN"

# Install location
INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
fi

mv "$BIN" "$INSTALL_DIR/$BIN"

# Warn if not in PATH
case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *) echo "  Note: Add $INSTALL_DIR to your PATH" ;;
esac

echo ""
echo "  pkll $VERSION installed to $INSTALL_DIR/$BIN"
echo "  Run: pkll <port>"
echo ""
