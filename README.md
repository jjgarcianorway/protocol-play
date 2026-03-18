# protocol: play

> *"The beauty isn't in perfection. It's in continuing after the mistake."*

A narrative puzzle game about an AI named Anna, an ark carrying 14,892 sleeping humans, and the choices that define what it means to be human. Built with [Bevy 0.18](https://bevyengine.org/) (Rust).

Inspired by **Baldur's Gate 3**, **Detroit: Become Human**, **Telltale Games**, **Life is Strange**, **Battlestar Galactica**, and **The Expanse**.

## What is this?

Earth has fallen. An ark ship carries the last of humanity through interstellar space. You are the one person awake, maintaining the ship through puzzles while an AI named Anna tells you the stories of the 14,892 people sleeping in cryogenic pods.

Every decision matters. Every character has a history. Every playthrough reveals something new.

**212 dialog scenes. 26 named characters. 7 collapse archetypes. No magic, no aliens — just hard science and hard choices.**

## Status

**v0.52.0** — ~51,000 lines of code across 193 source files

| Game | Description | Status |
|------|-------------|--------|
| The Repairing | 149 tile puzzles across 13 chapters | **Beta** |
| The Gathering | Asteroid dodge, crystal collection | Pre-alpha |
| The Converter | Chain reaction crystal processing | Pre-alpha |
| The Delivery | Resource routing under pressure | Pre-alpha |
| Orben | Card game (Ronda-based) for staying human | Pre-alpha |
| Mission Control | Hub — story, decisions, crew management | Pre-alpha |

**Story:** 212 dialog scenes, extensive but largely untested. 26 named characters with multi-episode arcs (Lost-style). 7 Earth collapse archetypes with severity system. All grounded in real science.

## Features

### Narrative
- **212 dialog scenes** with branching choices across 70+ files
- **26 named characters** — each with backstory, moral dilemma, and multi-scene arc
- **"Anna will remember that"** — Telltale-style choice notifications
- **7 collapse archetypes** — Climate, Resource Wars, Pandemic, Nuclear, AI, Political, Economic
- **Severity system** (1-5) — same collapse type, different intensity, different story
- **World seed** — every playthrough generates a unique world (shareable between friends)
- **NG+ exclusive content** — replay reveals new scenes, Anna acknowledges the loop
- **Character codex** — Crew Manifest with 26 unlockable entries
- **Decision tree** — Journey Map visualizing your choices with parallax
- **6 endings** shaped by your decisions throughout the game

### Systems
- **Procedural sound** — 22 Mini Metro-style synthesized sounds (no external audio files)
- **Anna's mood glow** — 12 color states, narrator-driven, smooth transitions
- **Profile system** — 5 player slots for family members
- **i18n** — English (US) + Español (España), 633 translated strings
- **Cinematic main menu** — parallax starfield, staggered animations
- **Loading screen** — "Seeding the universe..." world generation
- **Settings** — language, display, audio, custom seed input
- **Stats screen** — Detroit-style decision summary
- **Credits** — Kojima-style scrolling with character epitaphs

### The Repairing (Bot Game) — Beta
- **149 campaign levels** across 13 chapters
- 14 tile types with progressive difficulty
- 3D rendered inventory icons
- Auto-save, stats tracking, chapter transitions
- Each tile type has a distinct procedural sound

### Download & Play

Pre-built binaries for **Windows**, **macOS**, and **Linux**:
[**Download latest release**](https://github.com/jjgarcianorway/protocol-play/releases/latest)

#### Windows
Extract the zip, double-click `protocol-play-player.exe`.

#### macOS
```sh
cd ~/Downloads/protocol-play-player-macos && xattr -cr . && chmod +x protocol-play-player && ./protocol-play-player
```

#### Linux
```sh
tar xzf protocol-play-player-linux.tar.gz && cd protocol-play-player-linux && ./protocol-play-player
```

## Building from Source

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
./build-all.sh          # Build all 7 binaries
```

## The Characters

| Character | Role | Dilemma |
|-----------|------|---------|
| Anna | Ship AI | Can a machine learn to dream? |
| Dr. Amira Hassan | Hydrologist | Built a system nobody would use |
| Viktor Petrov | Nuclear engineer | Wakes at 4:17 AM every night |
| Mei-Lin Chen | Schoolteacher | Smuggled seeds in her coat |
| Kwame & Kofi Asante | Bridge engineers | One stayed so a stranger could go |
| Dr. Elena Vasquez | Surgeon | 847 judgment calls in 3 days |
| Carlos Mendoza | Electrician | Stole a dying man's boarding pass |
| Sister Magdalena Santos | Nun/astrophysicist | Burned a lab to save souls |
| General Fatou Diallo | Commander | Closed the gates on 8,200 people |
| Priya Nair | Software engineer | Found the bug. Didn't fix it. |
| Youssef Karam | Intelligence officer | Planted spy among refugees |

*...and 15 more, each with their own story.*

## License

[GNU General Public License v3.0](LICENSE)

## Credits

Created by **jjgarcianorway**
Story & code written with **Claude** (Anthropic)
Engine: **Bevy 0.18** | Language: **Rust** | Font: **Fira Sans** (Mozilla, SIL OFL)
