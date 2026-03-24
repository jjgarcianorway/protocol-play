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

echo "Building protocol play: repairing..."
cargo build --release --features player
cp target/release/protocol-play target/release/protocol-play-repairing

echo "Copying campaign levels and assets..."
cp campaign_levels/*.json target/release/
mkdir -p target/release/assets/icons target/release/assets/textures target/release/assets/fonts
cp assets/icons/*.png target/release/assets/icons/ 2>/dev/null || true
cp assets/textures/*.png target/release/assets/textures/ 2>/dev/null || true
cp assets/fonts/*.ttf target/release/assets/fonts/ 2>/dev/null || true

# Copy i18n files
mkdir -p target/release/i18n
cp i18n/*.json target/release/i18n/ 2>/dev/null || true

# Restore progress files
if ls "$PROGRESS_BAK"/*.json "$PROGRESS_BAK"/*.jsonl 2>/dev/null | head -1 > /dev/null 2>&1; then
    echo "Restoring progress files..."
    cp "$PROGRESS_BAK"/* target/release/
fi
rm -rf "$PROGRESS_BAK"

echo ""
echo "Done! Binary:"
echo "  target/release/protocol-play-repairing (+ 149 campaign levels)"
echo ""
echo "To run:  cd target/release && ./protocol-play-repairing"
