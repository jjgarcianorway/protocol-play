// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use rand::Rng;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
use crate::board::populate_board;
use crate::level_gen_algo::*;
use crate::level_gen_ui::*;

// === Stepper interactions ===
pub fn gen_stepper_interaction(
    bot_dec: Query<&Interaction, (With<GenBotDec>, Changed<Interaction>)>,
    bot_inc: Query<&Interaction, (With<GenBotInc>, Changed<Interaction>)>,
    hole_dec: Query<&Interaction, (With<GenHoleDec>, Changed<Interaction>)>,
    hole_inc: Query<&Interaction, (With<GenHoleInc>, Changed<Interaction>)>,
    mut settings: ResMut<GenSettings>,
    mut bot_text: Query<&mut Text, (With<GenBotValue>, Without<GenHoleValue>)>,
    mut hole_text: Query<&mut Text, (With<GenHoleValue>, Without<GenBotValue>)>,
) {
    if bot_dec.iter().any(|i| *i == Interaction::Pressed) && settings.num_bots > 1 {
        settings.num_bots -= 1;
        for mut t in &mut bot_text { t.0 = format!("{}", settings.num_bots); }
    }
    if bot_inc.iter().any(|i| *i == Interaction::Pressed) && settings.num_bots < 10 {
        settings.num_bots += 1;
        for mut t in &mut bot_text { t.0 = format!("{}", settings.num_bots); }
    }
    if hole_dec.iter().any(|i| *i == Interaction::Pressed) && settings.hole_percent >= 10 {
        settings.hole_percent -= 10;
        for mut t in &mut hole_text { t.0 = format!("{}%", settings.hole_percent); }
    }
    if hole_inc.iter().any(|i| *i == Interaction::Pressed) && settings.hole_percent < 50 {
        settings.hole_percent += 10;
        for mut t in &mut hole_text { t.0 = format!("{}%", settings.hole_percent); }
    }
}

// === Hole placement stepper ===
pub fn gen_hole_place_interaction(
    dec: Query<&Interaction, (With<GenHolePlaceDec>, Changed<Interaction>)>,
    inc: Query<&Interaction, (With<GenHolePlaceInc>, Changed<Interaction>)>,
    mut settings: ResMut<GenSettings>,
    mut text: Query<&mut Text, With<GenHolePlaceValue>>,
) {
    let changed = if dec.iter().any(|i| *i == Interaction::Pressed) {
        settings.hole_placement = settings.hole_placement.prev(); true
    } else if inc.iter().any(|i| *i == Interaction::Pressed) {
        settings.hole_placement = settings.hole_placement.next(); true
    } else { false };
    if changed {
        for mut t in &mut text { t.0 = settings.hole_placement.label().into(); }
    }
}

// === Difficulty stepper ===
pub fn gen_difficulty_interaction(
    dec: Query<&Interaction, (With<GenDiffDec>, Changed<Interaction>)>,
    inc: Query<&Interaction, (With<GenDiffInc>, Changed<Interaction>)>,
    mut settings: ResMut<GenSettings>,
    mut text: Query<&mut Text, With<GenDiffValue>>,
) {
    if dec.iter().any(|i| *i == Interaction::Pressed) && settings.difficulty >= 5 {
        settings.difficulty -= 5;
        for mut t in &mut text { t.0 = format!("{}", settings.difficulty); }
    }
    if inc.iter().any(|i| *i == Interaction::Pressed) && settings.difficulty <= 95 {
        settings.difficulty += 5;
        for mut t in &mut text { t.0 = format!("{}", settings.difficulty); }
    }
}

// === Inventory stepper ===
pub fn gen_inv_interaction(
    dec: Query<&Interaction, (With<GenInvDec>, Changed<Interaction>)>,
    inc: Query<&Interaction, (With<GenInvInc>, Changed<Interaction>)>,
    mut settings: ResMut<GenSettings>,
    mut text: Query<&mut Text, With<GenInvValue>>,
) {
    if dec.iter().any(|i| *i == Interaction::Pressed) && settings.inventory_target > 0 {
        settings.inventory_target -= 1;
        for mut t in &mut text { t.0 = inventory_label(settings.inventory_target); }
    }
    if inc.iter().any(|i| *i == Interaction::Pressed) && settings.inventory_target < 20 {
        settings.inventory_target += 1;
        for mut t in &mut text { t.0 = inventory_label(settings.inventory_target); }
    }
}

// === Weight interaction ===
pub fn gen_weight_interaction(
    decs: Query<(&Interaction, &GenWeightDec), Changed<Interaction>>,
    incs: Query<(&Interaction, &GenWeightInc), Changed<Interaction>>,
    mut settings: ResMut<GenSettings>,
    mut vals: Query<(&GenWeightVal, &mut Text, &mut BackgroundColor)>,
    mut pcts: Query<(&GenWeightPct, &mut Text), Without<GenWeightVal>>,
) {
    let mut changed = false;
    for (interaction, dec) in &decs {
        if *interaction == Interaction::Pressed && settings.weights[dec.0 as usize] > 0 {
            settings.weights[dec.0 as usize] -= 1;
            changed = true;
        }
    }
    for (interaction, inc) in &incs {
        if *interaction == Interaction::Pressed && settings.weights[inc.0 as usize] < GEN_MAX_WEIGHT {
            settings.weights[inc.0 as usize] += 1;
            changed = true;
        }
    }
    if changed { update_weight_displays(&settings.weights, &mut vals, &mut pcts); }
}

// === All Equal / Clear buttons ===
pub fn gen_all_equal_interaction(
    q: Query<&Interaction, (With<GenAllEqualBtn>, Changed<Interaction>)>,
    mut settings: ResMut<GenSettings>,
    mut vals: Query<(&GenWeightVal, &mut Text, &mut BackgroundColor)>,
    mut pcts: Query<(&GenWeightPct, &mut Text), Without<GenWeightVal>>,
) {
    if !q.iter().any(|i| *i == Interaction::Pressed) { return; }
    settings.weights = [GEN_DEFAULT_WEIGHT; GEN_NUM_WEIGHTS];
    update_weight_displays(&settings.weights, &mut vals, &mut pcts);
}

pub fn gen_clear_weights_interaction(
    q: Query<&Interaction, (With<GenClearWeightsBtn>, Changed<Interaction>)>,
    mut settings: ResMut<GenSettings>,
    mut vals: Query<(&GenWeightVal, &mut Text, &mut BackgroundColor)>,
    mut pcts: Query<(&GenWeightPct, &mut Text), Without<GenWeightVal>>,
) {
    if !q.iter().any(|i| *i == Interaction::Pressed) { return; }
    settings.weights = [0; GEN_NUM_WEIGHTS];
    update_weight_displays(&settings.weights, &mut vals, &mut pcts);
}

// === Unique toggle interaction ===
pub fn gen_toggle_interaction(
    q: Query<&Interaction, (With<GenUniqueToggle>, Changed<Interaction>)>,
    mut settings: ResMut<GenSettings>,
    mut bg: Query<&mut BackgroundColor, With<GenUniqueToggle>>,
    mut text: Query<&mut Text, With<GenUniqueCheck>>,
) {
    if !q.iter().any(|i| *i == Interaction::Pressed) { return; }
    settings.unique_solution = !settings.unique_solution;
    for mut b in &mut bg {
        b.0 = if settings.unique_solution { rgb(GEN_TOGGLE_ON) } else { rgb(GEN_TOGGLE_OFF) };
    }
    for mut t in &mut text {
        t.0 = if settings.unique_solution { "X".into() } else { String::new() };
    }
}

// === Door chain stepper ===
pub fn gen_chain_interaction(
    dec: Query<&Interaction, (With<GenChainDec>, Changed<Interaction>)>,
    inc: Query<&Interaction, (With<GenChainInc>, Changed<Interaction>)>,
    mut settings: ResMut<GenSettings>,
    mut text: Query<&mut Text, With<GenChainValue>>,
) {
    if dec.iter().any(|i| *i == Interaction::Pressed) && settings.door_chains > 0 {
        settings.door_chains -= 1;
        for mut t in &mut text { t.0 = format!("{}", settings.door_chains); }
    }
    if inc.iter().any(|i| *i == Interaction::Pressed) && settings.door_chains < 5 {
        settings.door_chains += 1;
        for mut t in &mut text { t.0 = format!("{}", settings.door_chains); }
    }
}

// === Path sharing toggle ===
pub fn gen_path_share_interaction(
    q: Query<&Interaction, (With<GenPathShareToggle>, Changed<Interaction>)>,
    mut settings: ResMut<GenSettings>,
    mut bg: Query<&mut BackgroundColor, (With<GenPathShareToggle>, Without<GenUniqueToggle>)>,
    mut text: Query<&mut Text, With<GenPathShareCheck>>,
) {
    if !q.iter().any(|i| *i == Interaction::Pressed) { return; }
    settings.path_sharing = !settings.path_sharing;
    for mut b in &mut bg {
        b.0 = if settings.path_sharing { rgb(GEN_TOGGLE_ON) } else { rgb(GEN_TOGGLE_OFF) };
    }
    for mut t in &mut text {
        t.0 = if settings.path_sharing { "X".into() } else { String::new() };
    }
}

// === Confusion toggle ===
pub fn gen_confusion_interaction(
    q: Query<&Interaction, (With<GenConfusionToggle>, Changed<Interaction>)>,
    mut settings: ResMut<GenSettings>,
    mut bg: Query<&mut BackgroundColor, (With<GenConfusionToggle>, Without<GenUniqueToggle>, Without<GenPathShareToggle>)>,
    mut text: Query<&mut Text, With<GenConfusionCheck>>,
) {
    if !q.iter().any(|i| *i == Interaction::Pressed) { return; }
    settings.confusion_tiles = !settings.confusion_tiles;
    for mut b in &mut bg {
        b.0 = if settings.confusion_tiles { rgb(GEN_TOGGLE_ON) } else { rgb(GEN_TOGGLE_OFF) };
    }
    for mut t in &mut text {
        t.0 = if settings.confusion_tiles { "X".into() } else { String::new() };
    }
}

// === Preset buttons ===
pub fn gen_preset_interaction(
    mut commands: Commands,
    q: Query<(&Interaction, &GenPresetBtn), Changed<Interaction>>,
    dialog: Query<Entity, With<GenDialog>>,
    mut settings: ResMut<GenSettings>,
    font: Res<GameFont>,
) {
    let mut preset_id = None;
    for (interaction, btn) in &q {
        if *interaction == Interaction::Pressed { preset_id = Some(btn.0); }
    }
    let Some(id) = preset_id else { return; };
    match id {
        0 => { // Easy
            settings.difficulty = 20; settings.door_chains = 0; settings.path_sharing = false;
            settings.unique_solution = false; settings.confusion_tiles = false;
            settings.inventory_target = 0;
            settings.weights = [5, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        }
        1 => { // Medium
            settings.difficulty = 50; settings.door_chains = 1; settings.path_sharing = true;
            settings.unique_solution = false; settings.confusion_tiles = false;
            settings.inventory_target = 0;
            settings.weights = [5, 0, 4, 0, 3, 0, 3, 0, 2, 0, 0, 0];
        }
        2 => { // Hard
            settings.difficulty = 75; settings.door_chains = 2; settings.path_sharing = true;
            settings.unique_solution = false; settings.confusion_tiles = true;
            settings.inventory_target = 3;
            settings.weights = [4, 2, 4, 2, 3, 1, 3, 1, 3, 2, 1, 2];
        }
        3 => { // Expert
            settings.difficulty = 90; settings.door_chains = 3; settings.path_sharing = true;
            settings.unique_solution = true; settings.confusion_tiles = true;
            settings.inventory_target = 5;
            settings.weights = [5, 3, 5, 3, 4, 2, 4, 2, 4, 3, 2, 3];
        }
        _ => { // Chaos
            settings.difficulty = 100; settings.door_chains = 5; settings.path_sharing = true;
            settings.unique_solution = false; settings.confusion_tiles = true;
            settings.inventory_target = 0;
            settings.weights = [GEN_MAX_WEIGHT; GEN_NUM_WEIGHTS];
        }
    }
    // Rebuild dialog to reflect new settings
    for entity in &dialog { commands.entity(entity).despawn(); }
    spawn_gen_dialog(&mut commands, &font.0, &settings);
}

// === Cancel ===
pub fn gen_cancel_interaction(
    mut commands: Commands,
    q: Query<&Interaction, (With<GenCancelBtn>, Changed<Interaction>)>,
    dialog: Query<Entity, With<GenDialog>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut gen_state: ResMut<GeneratorState>,
    hovered: Res<HoveredCell>, mut ghost_cell: ResMut<GhostCell>,
) {
    let cancel = q.iter().any(|i| *i == Interaction::Pressed) || keys.just_pressed(KeyCode::Escape);
    if !cancel || dialog.is_empty() { return; }
    gen_state.phase = GenPhase::Idle;
    suppress_ghost(&hovered, &mut ghost_cell);
    fade_out(&mut commands, &dialog);
}

// === Generate ===
pub fn gen_generate_interaction(
    q: Query<&Interaction, (With<GenGenerateBtn>, Changed<Interaction>)>,
    mut gen_state: ResMut<GeneratorState>,
    settings: Res<GenSettings>,
    board_size: Res<BoardSize>,
) {
    if !q.iter().any(|i| *i == Interaction::Pressed) { return; }
    if !matches!(gen_state.phase, GenPhase::Idle | GenPhase::Failed) { return; }
    let seed: u64 = rand::thread_rng().r#gen();
    gen_state.phase = GenPhase::Running {
        attempt: 0, best: None, seed,
        config: GenConfig {
            board_size: board_size.0,
            num_bots: settings.num_bots,
            hole_percent: settings.hole_percent,
            hole_placement: settings.hole_placement,
            difficulty: settings.difficulty,
            weights: settings.weights,
            unique_solution: settings.unique_solution,
            inventory_target: settings.inventory_target,
            door_chains: settings.door_chains,
            path_sharing: settings.path_sharing,
            confusion_tiles: settings.confusion_tiles,
            required_tile: None,
        },
    };
}

// === Progress bar update ===
pub fn gen_update_progress(
    gen_state: Res<GeneratorState>,
    mut fill: Query<&mut Node, With<GenProgressFill>>,
    mut text: Query<&mut Text, With<GenProgressText>>,
) {
    if !gen_state.is_changed() { return; }
    let (pct, msg) = match &gen_state.phase {
        GenPhase::Running { attempt, best, .. } => {
            let p = (*attempt as f32 / GEN_MAX_ATTEMPTS as f32 * 100.0).min(100.0);
            let suffix = if let Some((_, r)) = best { format!(" (best: {r})") } else { String::new() };
            (p, format!("Generating... {:.0}%{suffix}", p))
        }
        GenPhase::Done(tiles, rating, _) => (100.0, gen_stats_line(tiles, *rating)),
        GenPhase::Failed => (100.0, "Failed - try different settings".into()),
        _ => (0.0, String::new()),
    };
    for mut n in &mut fill { n.width = Val::Percent(pct); }
    for mut t in &mut text { if t.0 != msg { t.0 = msg.clone(); } }
}

// === Apply result to board ===
pub fn gen_apply_result(
    mut commands: Commands,
    mut gen_state: ResMut<GeneratorState>,
    dialog: Query<Entity, With<GenDialog>>,
    tiles: Query<Entity, With<Tile>>,
    assets: Res<GameAssets>,
    board_size: Res<BoardSize>,
    mut validated: ResMut<LevelValidated>,
    mut inv_state: ResMut<InventoryState>,
    mut selected_tool: ResMut<SelectedTool>,
    hovered: Res<HoveredCell>, mut ghost_cell: ResMut<GhostCell>,
    expansion: Query<(Entity, &Children), With<ExpansionContainer>>,
    mut loaded_name: ResMut<LoadedLevelName>,
) {
    if !matches!(&gen_state.phase, GenPhase::Done(_, _, _)) { return; }
    let GenPhase::Done(result, rating, seed) = std::mem::replace(&mut gen_state.phase, GenPhase::Idle)
        else { return; };

    for entity in &tiles { commands.entity(entity).despawn(); }
    populate_board(&mut commands, board_size.0, &result, &assets);

    validated.0 = false;
    *inv_state = InventoryState::default(); inv_state.level = 1;
    selected_tool.0 = Tool::Floor;
    loaded_name.name = None;
    loaded_name.gen_seed = Some(seed);
    loaded_name.gen_difficulty = Some(rating);
    suppress_ghost(&hovered, &mut ghost_cell);
    if let Ok((_, children)) = expansion.single() {
        for child in children.iter() { commands.entity(child).despawn(); }
    }
    fade_out(&mut commands, &dialog);
}

fn gen_stats_line(tiles: &[(u32, u32, TileKind, bool)], rating: u32) -> String {
    let mut c = [0u32; 7]; // Turn, Arrow, Teleport, Bounce, Door+Sw, ColorSw, Painter
    let mut inv = 0u32;
    for (_, _, k, sol) in tiles {
        if *sol { inv += 1; }
        match k {
            TileKind::Turn(..) | TileKind::TurnBut(..) => c[0] += 1,
            TileKind::Arrow(..) | TileKind::ArrowBut(..) => c[1] += 1,
            TileKind::Teleport(..) | TileKind::TeleportBut(..) => c[2] += 1,
            TileKind::Bounce(..) | TileKind::BounceBut(..) => c[3] += 1,
            TileKind::Switch | TileKind::Door(..) => c[4] += 1,
            TileKind::ColorSwitch(..) | TileKind::ColorSwitchBut(..) => c[5] += 1,
            TileKind::Painter(..) => c[6] += 1,
            _ => {}
        }
    }
    let labels = ["T", "Ar", "Tp", "Bo", "D", "CS", "Pa"];
    let parts: Vec<_> = labels.iter().zip(&c)
        .filter(|(_, n)| **n > 0).map(|(l, n)| format!("{n}{l}")).collect();
    let stats = if parts.is_empty() { String::new() } else { format!(" | {}", parts.join(" ")) };
    format!("Diff: {rating}/100{stats} | {inv} inv")
}

// === Animated generate button (pulse while running) ===
pub fn gen_btn_pulse(
    gen_state: Res<GeneratorState>,
    mut btn_q: Query<&mut BackgroundColor, With<GenGenerateBtn>>,
    time: Res<Time>,
) {
    let running = matches!(gen_state.phase, GenPhase::Running { .. });
    for mut bg in &mut btn_q {
        if running {
            let t = (time.elapsed_secs() * GEN_PULSE_SPEED).sin() * 0.5 + 0.5;
            let v = GEN_PULSE_MIN + (GEN_PULSE_MAX - GEN_PULSE_MIN) * t;
            bg.0 = Color::srgb(v * 0.3, v * 0.7, v * 0.4);
        } else {
            bg.0 = rgb(CONFIRM_BTN_BG);
        }
    }
}
