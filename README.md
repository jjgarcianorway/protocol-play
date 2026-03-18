# protocol: play

A narrative puzzle game about an AI, an ark, and the choices that define humanity.
Built with [Bevy 0.18](https://bevyengine.org/) (Rust).

## Status

**v0.48.0** — Early development

| Game | Status | Notes |
|------|--------|-------|
| 🔧 The Repairing (Bot Game) | **Beta** | 149 campaign levels, playtested, polished |
| 🚀 The Gathering | Pre-alpha | Playable prototype, needs testing and balancing |
| ⚗️ The Converter | Pre-alpha | First implementation, needs testing |
| 📦 The Delivery | Pre-alpha | First implementation, needs testing |
| 🖥️ Mission Control | Pre-alpha | Framework with story system, needs testing |

## Download & Play

Pre-built binaries for **Windows**, **macOS**, and **Linux**:
[**Download latest release**](https://github.com/jjgarcianorway/protocol-play/releases/latest)

The Bot Game player is the most complete experience:
- Extract the archive and run `protocol-play-player`

### Options
- `--reset-stats` — Clear all progress (Bot Game player only)

## Building from Source

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
# Build everything
./build-all.sh

# Individual builds
cargo build --release                       # Editor
cargo build --release --features player     # Bot Game (recommended)
cargo build --release --features gathering  # The Gathering
cargo build --release --features converter  # The Converter
cargo build --release --features delivery   # The Delivery
cargo build --release --features mission    # Mission Control
```

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
