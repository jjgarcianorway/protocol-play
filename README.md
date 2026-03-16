# protocol: play

A game built with [Bevy 0.18](https://bevyengine.org/) (Rust).

## Status

**v0.28.1** — Silenced harmless warnings, assets bundled in releases

### Bot Game (Editor + Player)
- **149 campaign levels** across 13 chapters, each introducing a new mechanic
- Up to 9 bots per level — progressive scaling from 1 bot (Ch1) to 9 bots (Ch13 finals)
- Gradual difficulty progression with multi-bot from Chapter 2, confusion tiles, and board holes
- Unique background color per chapter for visual variety
- Replaceable tiles: fix mistakes by placing over your own tiles
- Resizable board (3x3 to 12x12) with smooth scale animations
- Adaptive camera (30° isometric view, resolution-independent)
- 14 tile types: Floor, Source, Goal, Turn, TurnBut, Teleport, TeleportBut, Bounce, BounceBut, Door, Switch, Painter, Arrow, ArrowBut, ColorSwitch, ColorSwitchBut
- Multi-level accordion inventory with auto-selection and color memory
- Phase-based bot movement with lane formation and merge flash effects
- In-editor level generator with presets (Easy/Medium/Hard/Expert/Chaos), tile weights, best-of-N selection, confusion tiles, and animated generate button
- Test mode, player mode, level save/load with metadata (seed, difficulty)
- Player stats tracking with creative solution detection
- Load dialog shows level summary (board size, tile counts, difficulty score)
- Smooth UI animations throughout

### The Gathering
- Vertical endless space runner — navigate a mining ship through asteroid fields
- 3D ship follows mouse with inertia, tilt, and pitch
- Procedural asteroid generation with rotation and marble-like deflection physics
- Crystal nebula clouds: absorb by proximity with particle effects
- Shield + life damage system with glancing/direct hit calculation
- Difficulty curve scales spawn rate, speed, and side-entry chance
- 4-layer parallax star background
- HUD: crystal counter (K/M format), distance (AU), time (days)
- Game over screen with stats and try again

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

## Download & Play

Pre-built binaries for **Windows**, **macOS**, and **Linux**:
[**Download latest release**](https://github.com/jjgarcianorway/protocol-play/releases/latest)

### Windows
Extract the zip, open the folder, double-click `protocol-play-player.exe`.
If Windows Defender blocks it, click "More info" → "Run anyway".

### macOS
Double-click the `.tar.gz` to extract. Open Terminal (Cmd+Space → type "Terminal" → Enter):
```sh
cd ~/Downloads/protocol-play-player-macos && xattr -cr . && chmod +x protocol-play-player && ./protocol-play-player
```
Next time, just:
```sh
cd ~/Downloads/protocol-play-player-macos && ./protocol-play-player
```

### Linux
```sh
tar xzf protocol-play-player-linux.tar.gz
cd protocol-play-player-linux && chmod +x protocol-play-player && ./protocol-play-player
```

### Multiple players on the same computer
Copy the extracted folder for each player (e.g. `paula/`, `barbara/`). Each folder keeps its own progress.

### Options
- `--reset-stats` — clear all progress and start fresh

## Building from source

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
# Build all (editor + player + gathering) and package levels
./build-all.sh

# Individual builds
cargo build --release                      # Editor
cargo build --release --features player    # Player
cargo build --release --features gathering # The Gathering

# Regenerate campaign levels
cargo build --release --bin generate-campaign
./target/release/generate-campaign
```

## Running from source

```sh
# Editor
cargo run --release

# Player (levels must be next to the binary)
cargo run --release --features player

# The Gathering
cargo run --release --features gathering
```

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
