#!/bin/sh
# install.sh - installs hash-bait to ~/.local/bin.
#
#   curl -fsSL https://raw.githubusercontent.com/Real-Fruit-Snacks/hash-bait/main/install.sh | sh
#
# Safe to re-run: it just downloads the latest release over the old one.
set -eu

repo='Real-Fruit-Snacks/hash-bait'
arch=$(uname -m)
os=$(uname -s)

case "$os-$arch" in
    Linux-x86_64)  asset='hash-bait-x86_64-unknown-linux-musl.tar.gz' ;;
    Darwin-arm64)  asset='hash-bait-aarch64-apple-darwin.tar.gz' ;;
    Darwin-x86_64) asset='hash-bait-x86_64-apple-darwin.tar.gz' ;;
    *) echo "unsupported platform: $os-$arch" >&2; exit 1 ;;
esac

dest="$HOME/.local/bin"
mkdir -p "$dest"

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

echo "Downloading latest hash-bait release..."
curl -fsSL "https://github.com/$repo/releases/latest/download/$asset" -o "$tmp/pkg.tar.gz"
tar -xzf "$tmp/pkg.tar.gz" -C "$dest"
chmod +x "$dest/hash-bait"

"$dest/hash-bait" --version
echo "hash-bait installed to $dest"
case ":$PATH:" in
    *":$dest:"*) ;;
    *) echo "note: $dest is not on your PATH" ;;
esac
