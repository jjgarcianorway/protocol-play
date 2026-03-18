// SPDX-License-Identifier: GPL-3.0-or-later
//! Sound palette — all procedurally generated sounds, created at startup.
//!
//! Mini Metro-inspired: every interaction has a short, musical, satisfying sound.

use bevy::prelude::*;
use bevy::audio::{PlaybackSettings, Volume};
use super::{SynthSound, SoundSettings, make_sound, tone, sweep, chord, arpeggio, bounce, pulse, pad};

// ── Musical constants (Hz) ───────────────────────────────────────────

const C3: f32 = 130.81;
const C4: f32 = 261.63;
const E4: f32 = 329.63;
const G4: f32 = 392.00;
const C5: f32 = 523.25;
const E5: f32 = 659.25;
const G5: f32 = 783.99;

// Crystal pitches — one per colour index (pentatonic scale for pleasant chains)
const CRYSTAL_PITCHES: [f32; 6] = [
    C5,          // 0 — blue
    587.33,      // 1 — D5 — green
    E5,          // 2 — red
    G5,          // 3 — yellow
    880.0,       // 4 — A5 — purple
    987.77,      // 5 — B5 — white
];

// ── SoundType enum ───────────────────────────────────────────────────

/// All available sound effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundType {
    // UI
    Click,
    Hover,
    Typewriter,
    ChoiceAppear,
    ChoiceSelect,
    Remember,
    // Bot Game — tile placement (each tile type has a distinct sound)
    TileFloor,
    TileSource,
    TileGoal,
    TileTurn,
    TileTurnBut,
    TileTeleport,
    TileTeleportBut,
    TileBounce,
    TileBounceBut,
    TileArrow,
    TileArrowBut,
    TileDoor,
    TileSwitch,
    TilePainter,
    // Bot Game — simulation
    BotMove,
    BotReachGoal,
    BotTurn,
    BotBounce,
    LevelComplete,
    LevelFail,
    // Gathering
    CrystalCollect(usize), // index into CRYSTAL_PITCHES
    ChainBonus(usize),     // chain level (1..=5)
    ShieldHit,
    NearMiss,
    Warning,
    // Mission Control
    DialogOpen,
    DialogClose,
}

// ── SoundPalette resource ────────────────────────────────────────────

/// Pre-generated sound handles, loaded at startup.
#[derive(Resource)]
pub struct SoundPalette {
    // UI
    pub click: Handle<SynthSound>,
    pub hover: Handle<SynthSound>,
    pub typewriter: Handle<SynthSound>,
    pub choice_appear: Handle<SynthSound>,
    pub choice_select: Handle<SynthSound>,
    pub remember: Handle<SynthSound>,
    // Bot Game — distinct tile-placement sounds
    pub tile_floor: Handle<SynthSound>,
    pub tile_source: Handle<SynthSound>,
    pub tile_goal: Handle<SynthSound>,
    pub tile_turn: Handle<SynthSound>,
    pub tile_turnbut: Handle<SynthSound>,
    pub tile_teleport: Handle<SynthSound>,
    pub tile_teleportbut: Handle<SynthSound>,
    pub tile_bounce: Handle<SynthSound>,
    pub tile_bouncebut: Handle<SynthSound>,
    pub tile_arrow: Handle<SynthSound>,
    pub tile_arrowbut: Handle<SynthSound>,
    pub tile_door: Handle<SynthSound>,
    pub tile_switch: Handle<SynthSound>,
    pub tile_painter: Handle<SynthSound>,
    // Bot Game — simulation
    pub bot_move: Handle<SynthSound>,
    pub bot_reach_goal: Handle<SynthSound>,
    pub bot_turn: Handle<SynthSound>,
    pub bot_bounce: Handle<SynthSound>,
    pub level_complete: Handle<SynthSound>,
    pub level_fail: Handle<SynthSound>,
    // Gathering
    pub crystal_collect: Vec<Handle<SynthSound>>,
    pub chain_bonus: Vec<Handle<SynthSound>>,
    pub shield_hit: Handle<SynthSound>,
    pub near_miss: Handle<SynthSound>,
    pub warning: Handle<SynthSound>,
    // Mission Control
    pub dialog_open: Handle<SynthSound>,
    pub dialog_close: Handle<SynthSound>,
}

// ── Setup system ─────────────────────────────────────────────────────

/// Startup system: generates all sounds and inserts `SoundPalette`.
pub fn setup_sound_palette(mut commands: Commands, mut assets: ResMut<Assets<SynthSound>>) {
    let add = |a: &mut Assets<SynthSound>, s: Vec<f32>| a.add(make_sound(s));

    // UI sounds
    let click = add(&mut assets, tone(800.0, 0.05, 0.4));
    let hover = add(&mut assets, tone(1200.0, 0.03, 0.15));
    let typewriter = add(&mut assets, tone(600.0, 0.02, 0.25));
    let choice_appear = add(&mut assets, sweep(400.0, 600.0, 0.15, 0.3));
    let choice_select = add(&mut assets, tone(500.0, 0.08, 0.4));
    let remember = add(&mut assets, chord(&[C5, E5], 0.5, 0.3));

    // Bot Game — distinct tile-placement sounds (Mini Metro: each tile = its own note)
    let tile_floor = add(&mut assets, tone(C3, 0.08, 0.15));                     // soft low note
    let tile_source = add(&mut assets, tone(E4, 0.12, 0.30));                    // warm starting note
    let tile_goal = add(&mut assets, tone(G5, 0.15, 0.35));                      // bright destination
    let tile_turn = add(&mut assets, sweep(350.0, 450.0, 0.08, 0.25));           // quick swoosh
    let tile_turnbut = add(&mut assets, sweep(250.0, 350.0, 0.10, 0.25));        // deeper swoosh
    let tile_teleport = add(&mut assets, sweep(200.0, 800.0, 0.08, 0.30));       // sci-fi warp
    let tile_teleportbut = add(&mut assets, sweep(800.0, 200.0, 0.08, 0.30));    // reverse warp
    let tile_bounce_snd = add(&mut assets, bounce(400.0, 0.10, 0.30));           // spring
    let tile_bouncebut = add(&mut assets, bounce(300.0, 0.12, 0.30));            // heavier spring
    let tile_arrow = add(&mut assets, sweep(400.0, 600.0, 0.10, 0.25));          // forward thrust
    let tile_arrowbut = add(&mut assets, sweep(500.0, 700.0, 0.10, 0.25));       // sharp thrust
    let tile_door = add(&mut assets, { // two quick tones: mechanical click-open
        let mut s = tone(300.0, 0.04, 0.30); s.extend(tone(500.0, 0.04, 0.30)); s });
    let tile_switch = add(&mut assets, tone(400.0, 0.04, 0.30));                 // toggle click
    let tile_painter = add(&mut assets, chord(&[C4, E4], 0.10, 0.25));           // color shift

    // Bot Game — simulation sounds
    let bot_move = add(&mut assets, tone(200.0, 0.03, 0.12));
    let bot_reach_goal = add(&mut assets, chord(&[C4, E4, G4], 0.3, 0.4));
    let bot_turn = add(&mut assets, sweep(300.0, 500.0, 0.08, 0.25));
    let bot_bounce_snd = add(&mut assets, bounce(400.0, 0.1, 0.35));
    let level_complete = add(&mut assets, arpeggio(&[C4, E4, G4, C5], 0.5, 0.45));
    let level_fail = add(&mut assets, sweep(400.0, 200.0, 0.3, 0.35));

    // Gathering sounds
    let crystal_collect: Vec<_> = CRYSTAL_PITCHES.iter()
        .map(|&f| add(&mut assets, tone(f, 0.1, 0.35)))
        .collect();
    let chain_bonus: Vec<_> = (1..=5usize).map(|level| {
        let freqs: Vec<f32> = CRYSTAL_PITCHES[..level.min(CRYSTAL_PITCHES.len())]
            .iter().copied().collect();
        add(&mut assets, arpeggio(&freqs, 0.12 + level as f32 * 0.06, 0.35))
    }).collect();
    let shield_hit = add(&mut assets, tone(100.0, 0.15, 0.4));
    let near_miss = add(&mut assets, sweep(600.0, 900.0, 0.1, 0.2));
    let warning = add(&mut assets, pulse(300.0, 0.2, 0.3));

    // Mission Control sounds
    let dialog_open = add(&mut assets, pad(180.0, 0.3, 0.25));
    let dialog_close = add(&mut assets, pad(160.0, 0.2, 0.2));

    commands.insert_resource(SoundPalette {
        click, hover, typewriter, choice_appear, choice_select, remember,
        tile_floor, tile_source, tile_goal, tile_turn, tile_turnbut,
        tile_teleport, tile_teleportbut,
        tile_bounce: tile_bounce_snd, tile_bouncebut,
        tile_arrow, tile_arrowbut, tile_door, tile_switch, tile_painter,
        bot_move, bot_reach_goal, bot_turn,
        bot_bounce: bot_bounce_snd,
        level_complete, level_fail,
        crystal_collect, chain_bonus, shield_hit, near_miss, warning,
        dialog_open, dialog_close,
    });
}

// ── Play helpers ─────────────────────────────────────────────────────

/// Look up the handle for a `SoundType` in the palette.
fn handle_for(palette: &SoundPalette, sound: SoundType) -> Handle<SynthSound> {
    match sound {
        SoundType::Click => palette.click.clone(),
        SoundType::Hover => palette.hover.clone(),
        SoundType::Typewriter => palette.typewriter.clone(),
        SoundType::ChoiceAppear => palette.choice_appear.clone(),
        SoundType::ChoiceSelect => palette.choice_select.clone(),
        SoundType::Remember => palette.remember.clone(),
        SoundType::TileFloor => palette.tile_floor.clone(),
        SoundType::TileSource => palette.tile_source.clone(),
        SoundType::TileGoal => palette.tile_goal.clone(),
        SoundType::TileTurn => palette.tile_turn.clone(),
        SoundType::TileTurnBut => palette.tile_turnbut.clone(),
        SoundType::TileTeleport => palette.tile_teleport.clone(),
        SoundType::TileTeleportBut => palette.tile_teleportbut.clone(),
        SoundType::TileBounce => palette.tile_bounce.clone(),
        SoundType::TileBounceBut => palette.tile_bouncebut.clone(),
        SoundType::TileArrow => palette.tile_arrow.clone(),
        SoundType::TileArrowBut => palette.tile_arrowbut.clone(),
        SoundType::TileDoor => palette.tile_door.clone(),
        SoundType::TileSwitch => palette.tile_switch.clone(),
        SoundType::TilePainter => palette.tile_painter.clone(),
        SoundType::BotMove => palette.bot_move.clone(),
        SoundType::BotReachGoal => palette.bot_reach_goal.clone(),
        SoundType::BotTurn => palette.bot_turn.clone(),
        SoundType::BotBounce => palette.bot_bounce.clone(),
        SoundType::LevelComplete => palette.level_complete.clone(),
        SoundType::LevelFail => palette.level_fail.clone(),
        SoundType::CrystalCollect(i) => {
            let idx = i.min(palette.crystal_collect.len().saturating_sub(1));
            palette.crystal_collect[idx].clone()
        }
        SoundType::ChainBonus(level) => {
            let idx = (level.saturating_sub(1)).min(palette.chain_bonus.len().saturating_sub(1));
            palette.chain_bonus[idx].clone()
        }
        SoundType::ShieldHit => palette.shield_hit.clone(),
        SoundType::NearMiss => palette.near_miss.clone(),
        SoundType::Warning => palette.warning.clone(),
        SoundType::DialogOpen => palette.dialog_open.clone(),
        SoundType::DialogClose => palette.dialog_close.clone(),
    }
}

/// Spawn a one-shot sound effect entity.
///
/// Usage from any system:
/// ```ignore
/// play_sound(&mut commands, &palette, SoundType::Click, &settings);
/// ```
pub fn play_sound(
    commands: &mut Commands,
    palette: &SoundPalette,
    sound: SoundType,
    settings: &SoundSettings,
) {
    if settings.muted { return; }
    let vol = settings.master_volume * settings.sfx_volume;
    if vol <= 0.0 { return; }
    let handle = handle_for(palette, sound);
    commands.spawn((
        bevy::audio::AudioPlayer::<SynthSound>(handle),
        PlaybackSettings::DESPAWN.with_volume(Volume::Linear(vol)),
    ));
}

/// Convenience: play sound only if palette is available (Option<Res<..>>).
pub fn try_play_sound(
    commands: &mut Commands,
    palette: Option<&SoundPalette>,
    sound: SoundType,
    settings: Option<&SoundSettings>,
) {
    if let (Some(p), Some(s)) = (palette, settings) {
        play_sound(commands, p, sound, s);
    }
}
