#!/bin/sh
# aVimStickies installer.
# Usage:
#   curl -fsSL https://github.com/OREOREOREOREORE/aVimStickies/releases/latest/download/install.sh | sh
#   curl -fsSL .../releases/latest/download/install.sh | sh -s v0.1.4
set -eu

REPO="OREOREOREOREORE/aVimStickies"
APP="aVimStickies.app"
VERSION="${1:-latest}"

if [ "$VERSION" = "latest" ]; then
  BASE="https://github.com/$REPO/releases/latest/download"
else
  case "$VERSION" in
    v*) TAG="$VERSION" ;;
    *) TAG="v$VERSION" ;;
  esac
  BASE="https://github.com/$REPO/releases/download/$TAG"
fi

case "$(uname -m)" in
  arm64 | aarch64) ARCH="aarch64" ;;
  x86_64) ARCH="x86_64" ;;
  *)
    echo "aVimStickies: unsupported architecture $(uname -m)" >&2
    exit 1
    ;;
esac

TARBALL="aVimStickies_${ARCH}.app.tar.gz"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

echo "Downloading aVimStickies ($ARCH)…"
curl -fsSL "$BASE/$TARBALL" -o "$TMP/$TARBALL"
curl -fsSL "$BASE/SHA256SUMS" -o "$TMP/SHA256SUMS"

echo "Verifying checksum…"
EXPECTED="$(grep " ${TARBALL}\$" "$TMP/SHA256SUMS" | awk '{print $1}')"
ACTUAL="$(shasum -a 256 "$TMP/$TARBALL" | awk '{print $1}')"
if [ -z "$EXPECTED" ] || [ "$EXPECTED" != "$ACTUAL" ]; then
  echo "aVimStickies: checksum verification failed — aborting." >&2
  exit 1
fi

echo "Installing to /Applications…"
tar -xzf "$TMP/$TARBALL" -C "$TMP"
rm -rf "/Applications/$APP"
if ! cp -R "$TMP/$APP" /Applications/ 2>/dev/null; then
  sudo rm -rf "/Applications/$APP"
  sudo cp -R "$TMP/$APP" /Applications/
fi

echo "Done. Launch aVimStickies from /Applications."
