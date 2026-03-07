# protocol: play

A game built with [Bevy](https://bevyengine.org/) (Rust).

## Status

**v0.2.0** - Tile-based board editor with multiple tile types and a multi-level inventory system.

### Features
- Resizable board (1x1 to 12x12) with smooth scale animations
- Adaptive camera (30° isometric view, resolution-independent)
- **Floor tiles** with procedural gray textures and dark edge borders
- **Source tiles** with circle+arrow shape, emission-colored glow, and directional rotation
- **Delete tool** for removing placed tiles
- **Multi-level accordion inventory**:
  - L1: Floor, Source, Delete
  - L2: 4 directional variants (N/E/S/W)
  - L3: 10 distinct emission colors (max 1 per color on board)
- Ghost preview and highlight on hover
- Smooth accordion-style animations for inventory transitions
- Count tracking: placed colors collapse out of inventory, deleted ones reappear

## Building

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
cargo build --release
```

The binary will be at `target/release/protocol-play`.

## Running

```sh
cargo run --release
```

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
