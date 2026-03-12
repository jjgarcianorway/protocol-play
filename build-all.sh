#!/bin/bash
set -e
echo "Building editor..."
cargo build --release
cp target/release/protocol-play target/release/protocol-play-editor
echo "Building player..."
cargo build --release --features player
cp target/release/protocol-play target/release/protocol-play-player
echo "Building gathering..."
cargo build --release --features gathering
cp target/release/protocol-play target/release/protocol-play-gathering
echo "Done! Binaries:"
echo "  target/release/protocol-play-editor"
echo "  target/release/protocol-play-player"
echo "  target/release/protocol-play-gathering"
