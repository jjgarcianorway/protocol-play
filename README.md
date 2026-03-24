# protocol play: repairing

> *"The beauty isn't in perfection. It's in continuing after the mistake."*

A tile-based puzzle game where you guide colored bots from source to goal by placing turns, arrows, teleports, bouncers, painters, switches, and doors. Built with [Bevy 0.18](https://bevyengine.org/) (Rust).

## What is it?

149 hand-tuned procedural puzzles across 13 chapters. Each chapter introduces a new mechanic. An AI narrator named Anna keeps you company with facts, tips, and commentary as you play.

## Status

**v0.57.0** — Beta

- 149 campaign levels across 13 chapters
- 14 tile types (turns, arrows, teleports, bouncers, painters, color switches, doors)
- Up to 8 simultaneous bots per puzzle
- Confusion tiles (decoy inventory items)
- In-game speed control (1x / 2x / 4x)
- 9 distinct bot colors
- Path sharing between bots
- Anna AI companion with 32 gamification facts
- i18n: English + Spanish
- Procedural sound (22 synthesized effects, no audio files)
- Auto-save, stats tracking, chapter backgrounds
- 3D rendered inventory icons

## Download & Play

Pre-built binaries for **Linux** (and eventually Windows/macOS):

[**Download latest release**](https://github.com/jjgarcianorway/protocol-play/releases/latest)

### Linux
```sh
tar xzf protocol-play-player-linux.tar.gz
cd protocol-play-player-linux
./protocol-play-player
```

### Windows
Extract the zip, double-click `protocol-play-player.exe`.

### macOS
```sh
cd ~/Downloads/protocol-play-player-macos
xattr -cr . && chmod +x protocol-play-player
./protocol-play-player
```

## Building from Source

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
./build-all.sh
cd target/release
./protocol-play-player
```

## The 13 Chapters

| Ch | Mechanic | Levels |
|----|----------|--------|
| 1 | Turns | 11 |
| 2 | Turn tiles (place from inventory) | 11 |
| 3 | Arrows | 11 |
| 4 | Arrow tiles (place from inventory) | 11 |
| 5 | Teleports | 11 |
| 6 | Teleport tiles (place from inventory) | 11 |
| 7 | Bouncers | 11 |
| 8 | Bouncer tiles (place from inventory) | 11 |
| 9 | Painters (color change) | 11 |
| 10 | Doors & switches | 11 |
| 11 | Color gates | 11 |
| 12 | Gate tiles (place from inventory) | 11 |
| 13 | All mechanics combined + boss levels | 17 |

## License

[GNU General Public License v3.0](LICENSE)

## Credits

Created by **jjgarcianorway**
Story & code written with **Claude** (Anthropic)
Engine: **Bevy 0.18** | Language: **Rust** | Font: **Fira Sans** (Mozilla, SIL OFL)
