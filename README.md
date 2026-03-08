# protocol: play

A game built with [Bevy](https://bevyengine.org/) (Rust).

## Status

**v0.4.0** - Turn tiles, modular codebase, and constants-driven styling.

### Features
- Resizable board (3x3 to 12x12) with smooth scale animations
- Adaptive camera (30° isometric view, resolution-independent)
- **Floor tiles** with procedural gray textures and dark edge borders
- **Source tiles** with circle+arrow symbol, 10 distinct colors, directional rotation, replaceable
- **Turn tiles** with L-shaped corner piece and dark-colored center circle, 10 colors, unlimited placement per color
- **Delete tool** with smooth red overlay fade-in/out and fade trails between cells
- **Multi-level accordion inventory** with auto-selection:
  - L1: Floor, Source, Turn, Delete — switching tools collapses/expands as needed
  - L2: 4 directional variants (N/E/S/W) — auto-selected on tool click
  - L3: 10 distinct colors with count indicators — auto-selected on direction pick
- **Isometric 3D-rendered inventory icons** with correct direction and color matching
- **Ghost preview** with smooth fade-in/out highlight on hover
- **Seamless tile placement**: placed tiles inherit ghost preview scale for smooth transitions
- Direction-consistent bots with colored bodies and eyes
- Auto-color cycling: next available color selected after placement, freed color pre-selected on delete
- Placed source colors collapse out of inventory, deleted ones smoothly reappear
- Modular codebase: constants, types, textures, board, inventory, systems (all files ≤400 lines)

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
