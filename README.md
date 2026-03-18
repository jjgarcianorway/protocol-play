# protocol: play

A narrative puzzle game about an AI, an ark, and the choices that define humanity.
Built with [Bevy 0.18](https://bevyengine.org/) (Rust).

## Status

**v0.48.0** — Bot Game in beta, other games in early development

## The Repairing (Bot Game) — Beta

**149 campaign levels** across 13 chapters. Guide colored bots from sources to goals by placing path tiles on the board.

### Features
- 14 tile types: Turn, Arrow, Teleport, Bounce, Painter, Door, Switch, and their inventory variants
- Progressive difficulty: from 1 bot on a 3×3 board to 9 bots on a 12×12 board
- Each chapter introduces a new mechanic with intro levels, then ramps up to boss levels
- 3D rendered inventory icons matching in-game tile appearance
- Replaceable tiles: fix mistakes by placing over your own tiles
- Yellow markers show which tiles you placed
- Smooth UI animations, chapter title overlays, background color transitions
- Auto-save progress per level, stats tracking
- Level seed displayed for bug reporting
- 9 distinct bot colors, confusion tiles, board holes for added challenge
- Version label and level seed always visible

### Download & Play

Pre-built binaries for **Windows**, **macOS**, and **Linux**:
[**Download latest release**](https://github.com/jjgarcianorway/protocol-play/releases/latest)

#### Windows
Extract the zip, open the folder, double-click `protocol-play-player.exe`.
If Windows Defender blocks it, click "More info" → "Run anyway".

#### macOS
Extract, then in Terminal:
```sh
cd ~/Downloads/protocol-play-player-macos && xattr -cr . && chmod +x protocol-play-player && ./protocol-play-player
```

#### Linux
```sh
tar xzf protocol-play-player-linux.tar.gz
cd protocol-play-player-linux && chmod +x protocol-play-player && ./protocol-play-player
```

#### Multiple players on the same computer
Copy the extracted folder for each player (e.g. `paula/`, `barbara/`). Each folder keeps its own progress.

#### Options
- `--reset-stats` — clear all progress and start fresh

### Campaign Structure

| Chapter | Mechanic | Levels |
|---------|----------|--------|
| 1. Turns | Basic path building | 8 + 3 boss |
| 2. Turn Tiles | Inventory placement | 8 + 3 boss |
| 3. Arrows | Forced direction | 8 + 3 boss |
| 4. Arrow Tiles | Arrow inventory | 8 + 3 boss |
| 5. Teleports | Warp mechanics | 8 + 3 boss |
| 6. Teleport Tiles | Teleport inventory | 8 + 3 boss |
| 7. Bounce | Reflection | 8 + 3 boss |
| 8. Bounce Tiles | Bounce inventory | 8 + 3 boss |
| 9. Painters | Color changing | 8 + 3 boss |
| 10. Doors & Switches | Toggle timing | 8 + 3 boss |
| 11. Color Switches | Color-gated toggling | 8 + 3 boss |
| 12. Color Switch Tiles | Color switch inventory | 8 + 3 boss |
| 13. Grand Mastery | All combined | 12 + 3 final + 2 secret |

## In Development

The full game will be a narrative experience connecting multiple minigames:

| Game | Description | Status |
|------|-------------|--------|
| 🔧 The Repairing | Tile puzzle — repair ship systems | **Beta** |
| 🚀 The Gathering | Asteroid dodge — collect crystal resources | Pre-alpha |
| ⚗️ The Converter | Chain reaction puzzle — process crystals into resources | Pre-alpha |
| 📦 The Delivery | Resource routing under time pressure | Pre-alpha |
| 🖥️ Mission Control | Hub — ship status, story, decisions | Pre-alpha |

All games are connected through a shared save system and a narrative driven by Anna, the ship's AI.

## Building from Source

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
# Build everything
./build-all.sh

# Bot Game only (recommended for playing)
cargo build --release --features player

# Generate campaign levels
cargo build --release --bin generate-campaign
./target/release/generate-campaign

# Generate inventory icons (requires display)
cargo build --release --bin generate-icons
./target/release/generate-icons
```

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
