#!/bin/bash
set -e

# Preserve player progress files before build (cargo clean would delete them)
PROGRESS_BAK=$(mktemp -d)
if ls target/release/*.progress.json target/release/stats.json target/release/stats.jsonl 2>/dev/null | head -1 > /dev/null 2>&1; then
    echo "Backing up progress files..."
    cp target/release/*.progress.json "$PROGRESS_BAK/" 2>/dev/null || true
    cp target/release/stats.json "$PROGRESS_BAK/" 2>/dev/null || true
    cp target/release/stats.jsonl "$PROGRESS_BAK/" 2>/dev/null || true
fi

echo "Building editor..."
cargo build --release
cp target/release/protocol-play target/release/protocol-play-editor
echo "Building player..."
cargo build --release --features player
cp target/release/protocol-play target/release/protocol-play-player
echo "Building gathering..."
cargo build --release --features gathering
cp target/release/protocol-play target/release/protocol-play-gathering
echo "Copying campaign levels next to player binary..."
cp campaign_levels/*.json target/release/

# Restore progress files
if ls "$PROGRESS_BAK"/*.json "$PROGRESS_BAK"/*.jsonl 2>/dev/null | head -1 > /dev/null 2>&1; then
    echo "Restoring progress files..."
    cp "$PROGRESS_BAK"/* target/release/
fi
rm -rf "$PROGRESS_BAK"

echo "Done! Binaries:"
echo "  target/release/protocol-play-editor"
echo "  target/release/protocol-play-player (+ 149 campaign levels)"
echo "  target/release/protocol-play-gathering"
