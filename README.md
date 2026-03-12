# protocol: play

A game built with [Bevy 0.18](https://bevyengine.org/) (Rust).

## Status

**v0.17.0** - Upgraded to Bevy 0.18. Bot puzzle game (6 levels) + The Gathering space runner minigame.

### Bot Game (Editor + Player)
- Resizable board (3x3 to 12x12) with smooth scale animations
- Adaptive camera (30° isometric view, resolution-independent)
- 14 tile types: Floor, Source, Goal, Turn, TurnBut, Teleport, TeleportBut, Bounce, BounceBut, Door, Switch, Painter, Arrow, ArrowBut
- Multi-level accordion inventory with auto-selection and color memory
- Phase-based bot movement with lane formation and merge flash effects
- Test mode, player mode, level save/load, 6 built-in puzzle levels
- Player stats tracking with creative solution detection
- Smooth UI animations throughout

### The Gathering
- Vertical endless space runner — navigate a mining ship through asteroid fields
- 3D ship follows mouse with inertia, tilt, and pitch
- Procedural asteroid generation (6 mesh types, 5 colors) with rotation
- Asteroid velocity vectors with lateral drift and side-entry spawning
- Asteroid-asteroid marble-like deflection physics
- Crystal clouds: absorb by proximity, shrink as collected
- Shield + life damage system with glancing/direct hit calculation
- Hit cooldowns prevent multi-frame damage, control loss on impact
- Difficulty curve scales spawn rate, speed, and side-entry chance based on time and crystals
- 4-layer parallax star background
- HUD: crystal counter (K/M format), distance (AU), time (days)
- Game over screen with stats and try again
- Screen shake on impact

## Downloads

### Player (for playtesting levels)

Download from [v0.15.0-player release](https://github.com/jjgarcianorway/protocol-play/releases/tag/v0.15.0-player):
- **Windows**: `protocol-player-windows.zip` — extract and run `protocol-player-windows.exe`
- **macOS**: `protocol-player-macos.tar.gz` — extract, then `xattr -cr protocol-player-macos && ./protocol-player-macos`

Includes 6 puzzle levels. Stats logged to `stats.json` and `stats.jsonl` next to the executable. Use `--reset-stats` to clear progress between testers.

### Editor (for creating levels)

Download from [v0.14.0 release](https://github.com/jjgarcianorway/protocol-play/releases/tag/v0.14.0):
- **Linux**: `protocol-play-editor`

## Building

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
# Build all (editor + player + gathering)
./build-all.sh

# Individual builds
cargo build --release                      # Editor
cargo build --release --features player    # Player
cargo build --release --features gathering # The Gathering
```

## Running

```sh
# Editor
cargo run --release

# Player (place level.json next to the binary)
cargo run --release --features player

# The Gathering
cargo run --release --features gathering
```

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
