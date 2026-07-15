#!/usr/bin/env bash

set -euo pipefail

REPOSITORY="ovierdev/Noivern"
OUTPUT_FILE="audio-detector"

echo "🎧 Noivern Binary Downloader"
echo "============================"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        PLATFORM="linux"
        ;;
    *)
        echo "Error: operating system not supported: $OS"
        echo "Noivern v0.1.0 currently supports Linux only."
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64 | amd64)
        TARGET_ARCH="x86_64"
        ;;

    aarch64 | arm64)
        TARGET_ARCH="aarch64"
        ;;

    *)
        echo "Error: architecture not supported: $ARCH"
        echo "Supported architectures: x86_64 and aarch64."
        exit 1
        ;;
esac

ASSET_NAME="audio-detector-${PLATFORM}-${TARGET_ARCH}"

DOWNLOAD_URL="https://github.com/${REPOSITORY}/releases/latest/download/${ASSET_NAME}"

echo
echo "Operating system : $PLATFORM"
echo "Architecture     : $TARGET_ARCH"
echo "Asset            : $ASSET_NAME"
echo
echo "Downloading Noivern..."

curl \
    --fail \
    --location \
    --progress-bar \
    "$DOWNLOAD_URL" \
    --output "$OUTPUT_FILE"

chmod +x "$OUTPUT_FILE"

echo
echo "Download complete."
echo
echo "Binary:"
echo "  ./$OUTPUT_FILE"
echo
echo "Start with:"
echo "  ./$OUTPUT_FILE --help"
echo
echo "Configure Noivern:"
echo "  ./$OUTPUT_FILE setup"
