# protocol: play

A game built with [Bevy](https://bevyengine.org/) (Rust).

## Status

**v0.13.1** - Ghost fade trails, exact-match suppression, player/editor UI consistency.

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
- **Player mode**: standalone exe reads `level.json` — just inventory, play, and reset (no editor UI)
- **Creator solution tracking**: levels store the creator's tile placements for future "creative solve" detection
- **Level save/load** with fade-in/out dialog animations
- **Switch/SwitchBut tiles** merged into 2 L1 inventory items (matching Bounce/BounceBut pattern)
- **Smooth UI animations**: hover fade-out trails (BorderFade), expansion height transitions (ExpHeightAnim), slide-in/out bars, fade dialogs, L2/L3 slot grow-in
- **Isometric 3D-rendered inventory icons** with correct direction and color matching
- **Ghost preview** with smooth fade-in/out highlight on hover, suppressed after tile placement and dialog dismissal
- **Seamless tile placement**: placed tiles inherit ghost preview scale for smooth transitions
- Phase-based bot movement: accelerate, cruise, decelerate, rotate, bounce, teleport, spin at goal
- Auto-color cycling: next available color selected after placement, freed color pre-selected on delete
- Placed source/goal/teleport colors collapse out of inventory, deleted ones smoothly reappear
- **UI helper library** (`ui_helpers.rs`): reusable color helpers, node builders, dialog spawner for consistent UX
- **Comprehensive constants** (`constants.rs`): all colors, sizes, speeds, thresholds centralized
- **Lane-based bot formation**: bots sharing a tile scale down and travel side-by-side in parallel lanes, with smooth transitions
- **Merge flash effect**: expanding white disc pulses when bots merge onto the same tile
- Modular codebase: 15 modules, all files ≤400 lines

## Downloads

### Player (for playtesting levels)

Download from [v0.12.0-player release](https://github.com/jjgarcianorway/protocol-play/releases/tag/v0.12.0-player):
- **Windows**: `protocol-player-windows.tar.gz`
- **macOS**: `protocol-player-macos.tar.gz`

Each package includes the executable and a sample `level.json`. Extract and run.

### Editor (for creating levels)

Download from [v0.13.1 release](https://github.com/jjgarcianorway/protocol-play/releases/tag/v0.13.1):
- **Linux**: `protocol-play-editor`

## Building

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
# Editor (full level editor)
cargo build --release

# Player (standalone playtester)
cargo build --release --features player
```

## Running

```sh
# Editor
cargo run --release

# Player (place level.json next to the binary)
cargo run --release --features player
```

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
