# protocol: play

A narrative puzzle game about an AI, an ark, and the choices that define humanity.

## Status

**v0.41.0** — Story complete: 10 questions, 6 endings, full narrative arc with New Game+

## The Story

You wake up. You don't know where you are. Anna — the ship's AI — needs your help.
Systems are failing. 14,892 people sleep in cryogenic pods, dreaming of a world that no longer exists.

You repair. You gather. You convert. You deliver. You make choices you can't take back.
And somewhere out there, New Earth is waiting. If you can survive long enough to find it.

## The Games

### The Repairing (Bot Game)
149 tile puzzles. Guide colored bots from sources to goals by placing path tiles.
Each puzzle repairs a ship system. When all 149 are complete, the journey ends.

### The Gathering
Dodge asteroids. Collect crystal clouds. Your ship takes damage every run.
5 crystal colors = 5 resource types. Better dodging = more crystals = longer survival.

### The Converter
Chain reaction puzzle. Process crystals into ship resources.
Bigger chains = better efficiency. Your skill determines how far your crystals go.

### The Delivery
Route resource pods to the correct ship sections under time pressure.
Streaks earn bonuses. Every missed delivery is a wasted resource.

### Mission Control
Your command center. See ship status. Talk to Anna. Make decisions.
Resources drain in real-time. Choose wisely — the crew depends on you.

## Download & Play

Pre-built binaries for **Windows**, **macOS**, and **Linux**:
[**Download latest release**](https://github.com/jjgarcianorway/protocol-play/releases/latest)

### How to Play
1. Extract the archive
2. Run `protocol-play-mission` (the main game)
3. All other game binaries should be in the same folder

### Individual Games (standalone)
- `protocol-play-player` — Bot Game with 149 campaign levels
- `protocol-play-gathering` — The Gathering standalone
- `protocol-play-converter` — The Converter standalone
- `protocol-play-delivery` — The Delivery standalone

### Options
- `--reset-stats` — Clear all progress (Bot Game player only)

## Building from Source

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
# Build everything
./build-all.sh

# Individual builds
cargo build --release                       # Editor
cargo build --release --features player     # Bot Game
cargo build --release --features gathering  # The Gathering
cargo build --release --features converter  # The Converter
cargo build --release --features delivery   # The Delivery
cargo build --release --features mission    # Mission Control

# Generate campaign levels
cargo build --release --bin generate-campaign
./target/release/generate-campaign

# Generate inventory icons
cargo build --release --bin generate-icons
./target/release/generate-icons
```

## The Choices

Anna asks you questions. There are no right answers.
Your decisions — and how you play — determine which of 6 endings you reach.
Some endings are hopeful. Some are not. All are earned.

## Credits

Built with [Bevy 0.18](https://bevyengine.org/) (Rust).

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
