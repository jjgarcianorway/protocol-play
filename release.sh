#!/bin/bash
set -e

# Usage: ./release.sh v0.18.2 "Short description of changes"
# Automates: version bump, README update, build, commit, push, trigger release workflow

if [ $# -lt 2 ]; then
    echo "Usage: $0 <version-tag> <description>"
    echo "Example: $0 v0.19.0 \"New minigame: The Converter\""
    exit 1
fi

TAG="$1"
DESC="$2"
VERSION="${TAG#v}"  # strip leading 'v'

echo "=== Releasing $TAG ==="

# 1. Bump Cargo.toml version
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
echo "  Bumped Cargo.toml to $VERSION"

# 2. Update README status line
sed -i "s/^\*\*v[0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\*\*.*$/\*\*$TAG\*\* — $DESC/" README.md
echo "  Updated README.md status"

# 3. Campaign levels (keep existing, only generate missing)
echo "  Checking campaign levels..."
cargo build --release --bin generate-campaign 2>&1 | grep -E "^(error|warning:.*generate)" || true
./target/release/generate-campaign
rm -f levels/*.json
cp campaign_levels/*.json levels/
echo "  Campaign levels ready"

# 4. Build all binaries
echo "  Building all binaries..."
./build-all.sh

# 5. Commit and push
git add -A
git commit -m "Release $TAG — $DESC

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
git push
echo "  Committed and pushed"

# 6. Trigger release workflow
echo "  Triggering release workflow..."
gh workflow run "Build Player Release" -f tag="$TAG"
echo ""
echo "=== Done! Release $TAG triggered ==="
echo "  Monitor: gh run list --workflow=player-release.yml --limit 1"
