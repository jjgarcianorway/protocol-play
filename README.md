# protocol: play

A game built with [Bevy 0.18](https://bevyengine.org/) (Rust).

## Status

**v0.18.0** — 149-level campaign, level generator with presets, animated UI, The Gathering space runner.

### Bot Game (Editor + Player)
- **149 campaign levels** across 13 chapters, each introducing a new mechanic
- Gradual difficulty progression from simple turns to multi-bot, multi-mechanic boss levels
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

## Building

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

## Running

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
