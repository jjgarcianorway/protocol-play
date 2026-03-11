#!/bin/bash
cd "$(dirname "$0")"
xattr -cr protocol-player-macos 2>/dev/null
./protocol-player-macos
