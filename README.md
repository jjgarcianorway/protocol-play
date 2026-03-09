# protocol: play

A game built with [Bevy](https://bevyengine.org/) (Rust).

## Status

**v0.10.0** - Multiple same-color sources/goals allowed; bots form visible clusters instead of z-fighting.

### Features
- Resizable board (3x3 to 12x12) with smooth scale animations
- Adaptive camera (30° isometric view, resolution-independent)
- **Floor tiles** with procedural gray textures and dark edge borders
- **Source tiles** with circle+arrow symbol, 10 distinct colors, directional rotation, replaceable, unlimited per color
- **Goal tiles** with star symbol, 10 colors, unlimited per color
- **Turn tiles** with L-shaped corner piece, 11 colors (10 + grey for all bots), directional, unlimited
- **TurnBut tiles** with L-shape + forbidden symbol, affects all bots except matching color, 10 colors
- **Teleport tiles** with ring + 7-segment number, max 2 per number (10 pairs), bot shrinks/grows between paired teleports
- **Bouncer tiles** with diamond shape, reverses bot direction 180°, 11 colors (10 + grey), unlimited
- **BounceBut tiles** with diamond + forbidden symbol, reverses all except matching color, 10 colors
- **Door tiles** (open/closed) with switch tile interaction
- **Arrow tiles** with arrow symbol, forces all bots to follow its direction (including 180° U-turns), 11 colors (10 + grey)
- **ArrowBut tiles** with arrow + forbidden symbol, bounces matching color bots, redirects all others, 10 colors
- **Delete tool** with smooth red overlay fade-in/out and fade trails between cells
- **Two-texture emission system**: base texture for shape/stroke, color mask for programmatic coloring via emissive
- **Editable file-based textures** (`assets/textures/`): base and mask PNGs auto-generated on first run, editable afterwards
- **Color memory**: switching tools remembers last placed color instead of auto-cycling
- **Multi-level accordion inventory** with auto-selection:
  - L1: Floor, Source, Goal, Turn, TurnBut, Teleport, Bounce, BounceBut, Door, Switch, Painter, Arrow, ArrowBut, Delete
  - L2: 4 directional variants (N/E/S/W) for Source, Turn, TurnBut, Arrow, ArrowBut; Open/Closed for Door
  - L3: Color/number selection with count indicators and availability tracking
- **Test mode**: mark tiles for inventory, flat sorted test inventory, remove tool, reset
- **Level save/load** with fade-in/out dialog animations
- **Isometric 3D-rendered inventory icons** with correct direction and color matching
- **Ghost preview** with smooth fade-in/out highlight on hover
- **Seamless tile placement**: placed tiles inherit ghost preview scale for smooth transitions
- **Orchestrated UI animations**: slide-in/out for inventory bars, fade-in/out for dialogs, coordinated startup entrance
- Phase-based bot movement: accelerate, cruise, decelerate, rotate, bounce, teleport, spin at goal
- Auto-color cycling: next available color selected after placement, freed color pre-selected on delete
- Placed source/goal/teleport colors collapse out of inventory, deleted ones smoothly reappear
- **UI helper library** (`ui_helpers.rs`): reusable color helpers, node builders, dialog spawner for consistent UX
- **Comprehensive constants** (`constants.rs`): all colors, sizes, speeds, thresholds centralized
- **Bot cluster rendering**: each bot gets a unique fixed XZ offset so overlapping bots form a visible cluster instead of z-fighting
- Modular codebase: 14 modules, all files ≤400 lines

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
