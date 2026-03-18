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
echo "Building converter..."
cargo build --release --features converter
cp target/release/protocol-play target/release/protocol-play-converter
echo "Building delivery..."
cargo build --release --features delivery
cp target/release/protocol-play target/release/protocol-play-delivery
echo "Building mission control..."
cargo build --release --features mission
cp target/release/protocol-play target/release/protocol-play-mission
echo "Building orben..."
cargo build --release --features orben
cp target/release/protocol-play target/release/protocol-play-orben
echo "Copying campaign levels and icons next to player binary..."
cp campaign_levels/*.json target/release/
mkdir -p target/release/assets/icons target/release/assets/textures
cp assets/icons/*.png target/release/assets/icons/ 2>/dev/null || true
cp assets/textures/*.png target/release/assets/textures/ 2>/dev/null || true

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
echo "  target/release/protocol-play-converter"
echo "  target/release/protocol-play-delivery"
echo "  target/release/protocol-play-mission"
echo "  target/release/protocol-play-orben"
