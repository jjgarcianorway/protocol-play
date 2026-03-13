// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
use crate::level_gen_algo::*;

// === Components ===
#[derive(Component)] pub struct GenDialog;
#[derive(Component)] pub struct GenButton;
#[derive(Component)] pub struct GenBotDec;
#[derive(Component)] pub struct GenBotInc;
#[derive(Component)] pub struct GenBotValue;
#[derive(Component)] pub struct GenHoleDec;
#[derive(Component)] pub struct GenHoleInc;
#[derive(Component)] pub struct GenHoleValue;
#[derive(Component)] pub struct GenHolePlaceDec;
#[derive(Component)] pub struct GenHolePlaceInc;
#[derive(Component)] pub struct GenHolePlaceValue;
#[derive(Component)] pub struct GenDiffDec;
#[derive(Component)] pub struct GenDiffInc;
#[derive(Component)] pub struct GenDiffValue;
#[derive(Component)] pub struct GenInvDec;
#[derive(Component)] pub struct GenInvInc;
#[derive(Component)] pub struct GenInvValue;
#[derive(Component)] pub struct GenWeightDec(pub u8);
#[derive(Component)] pub struct GenWeightInc(pub u8);
#[derive(Component)] pub struct GenWeightVal(pub u8);
#[derive(Component)] pub struct GenWeightPct(pub u8);
#[derive(Component)] pub struct GenAllEqualBtn;
#[derive(Component)] pub struct GenClearWeightsBtn;
#[derive(Component)] pub struct GenUniqueToggle;
#[derive(Component)] pub struct GenUniqueCheck;
#[derive(Component)] pub struct GenChainDec;
#[derive(Component)] pub struct GenChainInc;
#[derive(Component)] pub struct GenChainValue;
#[derive(Component)] pub struct GenPathShareToggle;
#[derive(Component)] pub struct GenPathShareCheck;
#[derive(Component)] pub struct GenConfusionToggle;
#[derive(Component)] pub struct GenConfusionCheck;
#[derive(Component)] pub struct GenPresetBtn(pub u8);
#[derive(Component)] pub struct GenGenerateBtn;
#[derive(Component)] pub struct GenCancelBtn;
#[derive(Component)] pub struct GenProgressFill;
#[derive(Component)] pub struct GenProgressText;

// === Settings resource ===
#[derive(Resource)]
pub struct GenSettings {
    pub num_bots: u32,
    pub hole_percent: u32,
    pub hole_placement: HolePlacement,
    pub difficulty: u32,
    pub unique_solution: bool,
    pub inventory_target: u32,
    pub weights: [u32; GEN_NUM_WEIGHTS],
    pub door_chains: u32,
    pub path_sharing: bool,
    pub confusion_tiles: bool,
}

impl Default for GenSettings {
    fn default() -> Self {
        Self {
            num_bots: 1, hole_percent: 0, hole_placement: HolePlacement::Both,
            difficulty: 50, unique_solution: false, inventory_target: 0,
            weights: [5, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            door_chains: 1, path_sharing: false, confusion_tiles: false,
        }
    }
}

pub fn inventory_label(v: u32) -> String {
    if v == 0 { "All".into() } else { format!("{v}") }
}

// Weight layout: (label, normal_index, but_index)
const WEIGHT_ROWS: [(&str, u8, Option<u8>); 7] = [
    ("Turn", 0, Some(1)),
    ("Arrow", 2, Some(3)),
    ("Teleport", 4, Some(5)),
    ("Bounce", 6, Some(7)),
    ("Door+Sw", 8, None),
    ("ColorSw", 9, Some(10)),
    ("Painter", 11, None),
];

// === Open dialog ===
pub fn gen_button_interaction(
    q: Query<&Interaction, (With<GenButton>, Changed<Interaction>)>,
    play_mode: Res<PlayMode>,
    mut commands: Commands,
    existing: Query<Entity, Or<(With<SaveDialog>, With<LoadDialog>, With<ValidationErrorDialog>,
        With<OverwriteDialog>, With<DeleteLevelDialog>, With<GenDialog>)>>,
    font: Res<GameFont>,
    settings: Res<GenSettings>,
    hovered: Res<HoveredCell>, mut ghost_cell: ResMut<GhostCell>,
) {
    if *play_mode != PlayMode::Editing { return; }
    if !q.iter().any(|i| *i == Interaction::Pressed) { return; }
    if !existing.is_empty() { return; }
    suppress_ghost(&hovered, &mut ghost_cell);
    spawn_gen_dialog(&mut commands, &font.0, &settings);
}

pub fn spawn_gen_dialog(commands: &mut Commands, f: &Handle<Font>, s: &GenSettings) {
    let tf = gf(DIALOG_TITLE_FONT, f);
    let lf = gf(GEN_LABEL_FONT, f);
    let tc = TextColor(Color::WHITE);
    let hc = TextColor(rgb(GEN_HINT_COLOR));
    let hf = gf(GEN_HINT_FONT, f);
    let mut pn = dialog_panel_node(DIALOG_ROW_GAP);
    pn.min_width = Val::Px(GEN_DIALOG_WIDTH);
    pn.max_height = Val::Vh(GEN_DIALOG_MAX_H);
    pn.overflow.y = OverflowAxis::Scroll;

    spawn_dialog(commands, GenDialog, pn, |panel| {
        panel.spawn((Text::new("Level Generator"), tf.clone(), tc));

        // === Presets ===
        section_label(panel, "PRESETS", f);
        panel.spawn((Text::new("One-click configurations"), hf.clone(), hc));
        panel.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(4.0),
            flex_wrap: FlexWrap::Wrap, ..default() })
        .with_children(|row| {
            for (i, name) in ["Easy", "Medium", "Hard", "Expert", "Chaos"].iter().enumerate() {
                let (r, g, b) = GEN_PRESET_COLORS[i];
                row.spawn((Button, GenPresetBtn(i as u8), preset_btn_node(),
                    BackgroundColor(rgb((r, g, b)))))
                    .with_child((Text::new(*name), gf(GEN_PRESET_FONT, f), tc));
            }
        });

        // === Board ===
        section_label(panel, "BOARD", f);
        stepper_row(panel, "Bots", &format!("{}", s.num_bots), f, GenBotDec, GenBotInc, GenBotValue);
        stepper_row(panel, "Holes", &format!("{}%", s.hole_percent), f, GenHoleDec, GenHoleInc, GenHoleValue);
        stepper_row(panel, "Hole pos", s.hole_placement.label(), f, GenHolePlaceDec, GenHolePlaceInc, GenHolePlaceValue);

        // === Complexity ===
        section_label(panel, "COMPLEXITY", f);
        panel.spawn((Text::new("Path length, mechanic density, interactions"), hf.clone(), hc));
        stepper_row(panel, "Difficulty", &format!("{}", s.difficulty), f, GenDiffDec, GenDiffInc, GenDiffValue);
        stepper_row(panel, "Inventory", &inventory_label(s.inventory_target), f, GenInvDec, GenInvInc, GenInvValue);
        stepper_row(panel, "Door chains", &format!("{}", s.door_chains), f, GenChainDec, GenChainInc, GenChainValue);
        panel.spawn((Text::new("Switch/door pairs for cross-bot puzzles"), hf.clone(), hc));
        toggle_row(panel, "Path sharing", s.path_sharing, f, GenPathShareToggle, GenPathShareCheck);
        panel.spawn((Text::new("Bots reuse each other's floor corridors"), hf.clone(), hc));
        toggle_row(panel, "Unique solution", s.unique_solution, f, GenUniqueToggle, GenUniqueCheck);
        toggle_row(panel, "Confusion tiles", s.confusion_tiles, f, GenConfusionToggle, GenConfusionCheck);
        panel.spawn((Text::new("Add red herring tiles to inventory"), hf.clone(), hc));

        // === Tile weights ===
        section_label(panel, "TILE WEIGHTS", f);
        panel.spawn((Text::new("Relative chance per tile type (0=off)"), hf.clone(), hc));
        let hdr_c = TextColor(rgb(DIALOG_EMPTY_TEXT));
        let total: u32 = s.weights.iter().sum();
        panel.spawn(Node { flex_direction: FlexDirection::Column, row_gap: Val::Px(3.0),
            width: Val::Percent(100.0), ..default() })
        .with_children(|grid| {
            grid.spawn(Node { flex_direction: FlexDirection::Row, align_items: AlignItems::Center,
                width: Val::Percent(100.0), ..default() })
            .with_children(|row| {
                row.spawn(Node { width: Val::Px(62.0), ..default() })
                    .with_child((Text::new("Type"), lf.clone(), hdr_c));
                row.spawn(Node { width: Val::Px(GEN_WEIGHT_CELL_W), justify_content: JustifyContent::Center, ..default() })
                    .with_child((Text::new("Normal"), gf(COUNT_FONT, f), hdr_c));
                row.spawn(Node { width: Val::Px(GEN_WEIGHT_CELL_W), justify_content: JustifyContent::Center, ..default() })
                    .with_child((Text::new("But"), gf(COUNT_FONT, f), hdr_c));
            });
            for &(label, ni, bi) in &WEIGHT_ROWS {
                weight_grid_row(grid, label, ni, bi, &s.weights, total, f);
            }
        });

        panel.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(8.0), ..default() })
        .with_children(|row| {
            row.spawn((Button, GenAllEqualBtn, small_btn_node(), BackgroundColor(btn_bg())))
                .with_child((Text::new("All Equal"), gf(COUNT_FONT, f), tc));
            row.spawn((Button, GenClearWeightsBtn, small_btn_node(), BackgroundColor(btn_bg())))
                .with_child((Text::new("Clear All"), gf(COUNT_FONT, f), tc));
        });

        // === Actions ===
        panel.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(GEN_BTN_GAP),
            margin: UiRect::top(Val::Px(4.0)), ..default() })
        .with_children(|row| {
            row.spawn((Button, GenCancelBtn, dialog_btn_node(), BackgroundColor(btn_bg())))
                .with_child((Text::new("Cancel"), lf.clone(), tc));
            row.spawn((Button, GenGenerateBtn, dialog_btn_node(), BackgroundColor(rgb(CONFIRM_BTN_BG))))
                .with_child((Text::new("Generate"), lf.clone(), tc));
        });

        panel.spawn(Node { flex_direction: FlexDirection::Column, width: Val::Percent(100.0),
            row_gap: Val::Px(4.0), ..default() })
        .with_children(|area| {
            area.spawn((Text::new(""), gf(COUNT_FONT, f), tc, GenProgressText));
            area.spawn((Node { width: Val::Percent(100.0), height: Val::Px(GEN_PROGRESS_H),
                border_radius: BorderRadius::all(Val::Px(2.0)), overflow: Overflow::clip(), ..default() },
                BackgroundColor(rgb(GEN_PROGRESS_BG)),
            )).with_children(|track| {
                track.spawn((Node { width: Val::Percent(0.0), height: Val::Percent(100.0), ..default() },
                    BackgroundColor(rgb(GEN_PROGRESS_FILL)), GenProgressFill));
            });
        });
    });
}

fn section_label(parent: &mut ChildSpawnerCommands, label: &str, f: &Handle<Font>) {
    parent.spawn(Node { margin: UiRect::top(Val::Px(4.0)), ..default() })
        .with_child((Text::new(label), gf(GEN_SECTION_FONT, f), TextColor(rgb(GEN_SECTION_COLOR))));
}

fn preset_btn_node() -> Node {
    Node {
        padding: UiRect::axes(Val::Px(GEN_PRESET_PAD.0), Val::Px(GEN_PRESET_PAD.1)),
        border_radius: BorderRadius::all(Val::Px(4.0)),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center,
        ..default()
    }
}

// === Widget helpers ===
fn small_btn_node() -> Node {
    Node {
        padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
        border_radius: BorderRadius::all(Val::Px(4.0)),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center,
        ..default()
    }
}

fn stepper_row<D: Component, I: Component, V: Component>(
    parent: &mut ChildSpawnerCommands, label: &str, value: &str, f: &Handle<Font>,
    dec: D, inc: I, val: V,
) {
    let lf = gf(GEN_LABEL_FONT, f);
    let sf = gf(GEN_STEPPER_FONT, f);
    let tc = TextColor(Color::WHITE);
    let btn = Node { width: Val::Px(GEN_STEPPER_BTN), height: Val::Px(GEN_STEPPER_BTN),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center,
        border_radius: BorderRadius::all(Val::Px(4.0)), ..default() };
    parent.spawn(Node { flex_direction: FlexDirection::Row, align_items: AlignItems::Center,
        column_gap: Val::Px(10.0), justify_content: JustifyContent::SpaceBetween,
        width: Val::Percent(100.0), ..default() })
    .with_children(|row| {
        row.spawn((Text::new(label), lf, tc));
        row.spawn(Node { flex_direction: FlexDirection::Row, align_items: AlignItems::Center,
            column_gap: Val::Px(4.0), ..default() })
        .with_children(|stepper| {
            stepper.spawn((Button, dec, btn.clone(), BackgroundColor(btn_bg())))
                .with_child((Text::new("-"), sf.clone(), tc));
            stepper.spawn(Node { width: Val::Px(GEN_VALUE_WIDTH),
                justify_content: JustifyContent::Center, ..default() })
                .with_child((Text::new(value), sf.clone(), tc, val));
            stepper.spawn((Button, inc, btn, BackgroundColor(btn_bg())))
                .with_child((Text::new("+"), sf, tc));
        });
    });
}

fn weight_grid_row(
    parent: &mut ChildSpawnerCommands, label: &str,
    norm_idx: u8, but_idx: Option<u8>,
    weights: &[u32; GEN_NUM_WEIGHTS], total: u32, f: &Handle<Font>,
) {
    let tc = TextColor(Color::WHITE);
    parent.spawn(Node { flex_direction: FlexDirection::Row, align_items: AlignItems::Center,
        width: Val::Percent(100.0), ..default() })
    .with_children(|row| {
        row.spawn(Node { width: Val::Px(62.0), ..default() })
            .with_child((Text::new(label), gf(GEN_LABEL_FONT, f), tc));
        weight_cell(row, norm_idx, weights[norm_idx as usize], total, f);
        if let Some(idx) = but_idx {
            weight_cell(row, idx, weights[idx as usize], total, f);
        } else {
            row.spawn(Node { width: Val::Px(GEN_WEIGHT_CELL_W), ..default() });
        }
    });
}

fn weight_cell(parent: &mut ChildSpawnerCommands, idx: u8, weight: u32, total: u32, f: &Handle<Font>) {
    let pct = if total > 0 { weight * 100 / total } else { 0 };
    let wf = gf(GEN_WEIGHT_FONT, f);
    let tc = TextColor(Color::WHITE);
    let btn = Node { width: Val::Px(GEN_WEIGHT_BTN), height: Val::Px(GEN_WEIGHT_BTN),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center,
        border_radius: BorderRadius::all(Val::Px(3.0)), ..default() };
    let bg = weight_bg(weight);
    parent.spawn(Node { width: Val::Px(GEN_WEIGHT_CELL_W), flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center, column_gap: Val::Px(2.0),
        justify_content: JustifyContent::Center, ..default() })
    .with_children(|cell| {
        cell.spawn((Button, GenWeightDec(idx), btn.clone(), BackgroundColor(btn_bg())))
            .with_child((Text::new("-"), wf.clone(), tc));
        cell.spawn((Node { width: Val::Px(GEN_WEIGHT_VAL_W),
            justify_content: JustifyContent::Center, ..default() },
            BackgroundColor(bg),
        )).with_child((Text::new(format!("{weight}")), wf.clone(), tc, GenWeightVal(idx)));
        cell.spawn((Button, GenWeightInc(idx), btn, BackgroundColor(btn_bg())))
            .with_child((Text::new("+"), wf, tc));
        cell.spawn(Node { width: Val::Px(30.0), ..default() })
            .with_child((Text::new(format!("{pct}%")), gf(GEN_PCT_FONT, f),
                TextColor(rgb(DIALOG_EMPTY_TEXT)), GenWeightPct(idx)));
    });
}

fn weight_bg(w: u32) -> Color {
    let t = w as f32 / GEN_MAX_WEIGHT as f32;
    let (r0, g0, b0) = GEN_TOGGLE_OFF;
    let (r1, g1, b1) = GEN_TOGGLE_ON;
    rgb((r0 + (r1 - r0) * t, g0 + (g1 - g0) * t, b0 + (b1 - b0) * t))
}

pub fn toggle_row<T: Component, C: Component>(
    parent: &mut ChildSpawnerCommands, label: &str, on: bool, f: &Handle<Font>, toggle: T, check: C,
) {
    let sf = gf(GEN_STEPPER_FONT, f);
    let tc = TextColor(Color::WHITE);
    let bg = if on { rgb(GEN_TOGGLE_ON) } else { rgb(GEN_TOGGLE_OFF) };
    parent.spawn(Node { flex_direction: FlexDirection::Row, align_items: AlignItems::Center,
        column_gap: Val::Px(10.0), ..default() })
    .with_children(|row| {
        row.spawn((Button, toggle,
            Node { width: Val::Px(GEN_TOGGLE_SIZE), height: Val::Px(GEN_TOGGLE_SIZE),
                justify_content: JustifyContent::Center, align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(4.0)), ..default() },
            BackgroundColor(bg),
        )).with_child((Text::new(if on { "X" } else { "" }), sf, tc, check));
        row.spawn((Text::new(label), gf(GEN_LABEL_FONT, f), tc));
    });
}

pub fn update_weight_displays(
    weights: &[u32; GEN_NUM_WEIGHTS],
    vals: &mut Query<(&GenWeightVal, &mut Text, &mut BackgroundColor)>,
    pcts: &mut Query<(&GenWeightPct, &mut Text), Without<GenWeightVal>>,
) {
    let total: u32 = weights.iter().sum();
    for (v, mut t, mut bg) in vals.iter_mut() {
        let w = weights[v.0 as usize];
        t.0 = format!("{w}");
        bg.0 = weight_bg(w);
    }
    for (p, mut t) in pcts.iter_mut() {
        let pct = if total > 0 { weights[p.0 as usize] * 100 / total } else { 0 };
        t.0 = format!("{pct}%");
    }
}
