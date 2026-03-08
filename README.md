# protocol: play

A game built with [Bevy](https://bevyengine.org/) (Rust).

## Status

**v0.3.0** - Polished tile editor with smooth animations, isometric 3D inventory icons, and streamlined UX.

### Features
- Resizable board (3x3 to 12x12) with smooth scale animations
- Adaptive camera (30° isometric view, resolution-independent)
- **Floor tiles** with procedural gray textures and dark edge borders
- **Source tiles** with circle+arrow symbol (top face only), 10 distinct colors, directional rotation
- **Delete tool** with smooth red overlay fade-in/out and fade trails between cells
- **Multi-level accordion inventory** with auto-selection:
  - L1: Floor, Source, Delete — switching tools collapses/expands as needed
  - L2: 4 directional variants (N/E/S/W) — auto-selected on Source click
  - L3: 10 distinct colors (red, orange, yellow, light/dark green, light/dark blue, pink, purple, brown) — auto-selected on direction pick
- **Isometric 3D-rendered inventory icons** with correct direction and color matching
- **Ghost preview** with smooth fade-in/out highlight on hover
- **Seamless tile placement**: placed tiles inherit ghost preview scale for smooth transitions
- Direction-consistent bots with colored bodies and eyes
- Auto-color cycling: next available color selected after placement, freed color pre-selected on delete
- Placed colors collapse out of inventory, deleted ones smoothly reappear

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
