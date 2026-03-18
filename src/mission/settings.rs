// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings overlay — types, resources, spawn logic. Systems in `settings_systems.rs`.

use bevy::prelude::*;
use super::constants::*;
use super::settings_seed;
use crate::i18n::AVAILABLE_LANGUAGES;
use crate::save_state::GameState;

// ── Resources ──────────────────────────────────────────────────────────

/// Whether the settings overlay is open and its fade progress (0.0–1.0).
#[derive(Resource)]
pub struct SettingsOpen {
    pub open: bool,
    pub fade: f32,
}

impl Default for SettingsOpen {
    fn default() -> Self {
        Self { open: false, fade: 0.0 }
    }
}

/// Currently selected tab.
#[derive(Resource, Default)]
pub struct ActiveSettingsTab(pub SettingsTab);

// ── Components ─────────────────────────────────────────────────────────

#[derive(Component)]
pub struct SettingsOverlay;

#[derive(Component)]
pub struct SettingsContentArea;

#[derive(Component)]
pub struct SettingsTabBtn(pub SettingsTab);

#[derive(Component)]
pub struct SettingsLangBtn(pub usize);

#[derive(Component)]
pub struct SettingsResetBtn;

#[derive(Component)]
pub struct SettingsConfirmReset;

#[derive(Component)]
pub struct SettingsCancelReset;

#[derive(Component)]
pub struct ConfirmDialog;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsTab {
    #[default]
    Language,
    Display,
    Audio,
    Game,
}

impl SettingsTab {
    pub const ALL: [SettingsTab; 4] = [
        SettingsTab::Language,
        SettingsTab::Display,
        SettingsTab::Audio,
        SettingsTab::Game,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Language => "Language",
            Self::Display => "Display",
            Self::Audio => "Audio",
            Self::Game => "Game",
        }
    }
}

// ── Spawn / despawn ────────────────────────────────────────────────────

pub fn spawn_overlay(
    commands: &mut Commands, font: &Handle<Font>, tab: SettingsTab, gs: &GameState,
) {
    commands.spawn((
        SettingsOverlay,
        Button,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(
            SETTINGS_OVERLAY_BG.0, SETTINGS_OVERLAY_BG.1,
            SETTINGS_OVERLAY_BG.2, 0.0,
        )),
        GlobalZIndex(30),
    )).with_children(|overlay| {
        overlay.spawn((
            Node {
                max_width: Val::Px(SETTINGS_PANEL_MAX_W),
                max_height: Val::Px(SETTINGS_PANEL_MAX_H),
                width: Val::Percent(80.0),
                height: Val::Percent(70.0),
                flex_direction: FlexDirection::Row,
                border_radius: BorderRadius::all(Val::Px(SETTINGS_PANEL_CORNER)),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgba(
                SETTINGS_PANEL_BG.0, SETTINGS_PANEL_BG.1,
                SETTINGS_PANEL_BG.2, SETTINGS_PANEL_BG.3,
            )),
            BoxShadow::new(
                Color::srgba(
                    SETTINGS_GLOW_COLOR.0, SETTINGS_GLOW_COLOR.1,
                    SETTINGS_GLOW_COLOR.2, SETTINGS_GLOW_COLOR.3,
                ),
                Val::ZERO, Val::ZERO,
                Val::Px(SETTINGS_GLOW_SPREAD), Val::Px(SETTINGS_GLOW_BLUR),
            ),
            Button, Interaction::None,
        )).with_children(|panel| {
            spawn_tab_column(panel, font, tab);
            panel.spawn((
                Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(SETTINGS_PANEL_PAD)),
                    row_gap: Val::Px(14.0),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                SettingsContentArea,
            )).with_children(|content| {
                spawn_tab_content(content, font, tab, gs);
            });
        });
    });
}

fn spawn_tab_column(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>, active: SettingsTab,
) {
    parent.spawn(Node {
        width: Val::Px(SETTINGS_TAB_WIDTH),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(10.0)),
        row_gap: Val::Px(4.0),
        ..default()
    }).with_children(|col| {
        col.spawn((
            Text::new("Settings"),
            TextFont { font: font.clone(), font_size: SETTINGS_TITLE_FONT, ..default() },
            TextColor(Color::srgb(
                SETTINGS_TITLE_COLOR.0, SETTINGS_TITLE_COLOR.1, SETTINGS_TITLE_COLOR.2,
            )),
            Node { margin: UiRect::bottom(Val::Px(10.0)), ..default() },
        ));
        for tab in SettingsTab::ALL {
            let is_active = tab == active;
            let (bg, tc) = if is_active {
                (SETTINGS_TAB_ACTIVE_BG, SETTINGS_TAB_ACTIVE_COLOR)
            } else {
                (SETTINGS_TAB_BG, SETTINGS_TAB_COLOR)
            };
            col.spawn((
                Button,
                SettingsTabBtn(tab),
                Node {
                    padding: UiRect::axes(Val::Px(SETTINGS_TAB_PAD), Val::Px(8.0)),
                    border_radius: BorderRadius::all(Val::Px(SETTINGS_TAB_CORNER)),
                    ..default()
                },
                BackgroundColor(Color::srgba(bg.0, bg.1, bg.2, bg.3)),
            )).with_child((
                Text::new(tab.label()),
                TextFont { font: font.clone(), font_size: SETTINGS_TAB_FONT, ..default() },
                TextColor(Color::srgb(tc.0, tc.1, tc.2)),
            ));
        }
    });
}

fn spawn_tab_content(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    tab: SettingsTab, gs: &GameState,
) {
    match tab {
        SettingsTab::Language => spawn_language_content(parent, font, gs),
        SettingsTab::Display => spawn_display_content(parent, font),
        SettingsTab::Audio => spawn_audio_content(parent, font),
        SettingsTab::Game => spawn_game_content(parent, font, gs),
    }
}

fn spawn_language_content(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>, gs: &GameState,
) {
    heading(parent, font, "Language");
    for (i, &(code, name)) in AVAILABLE_LANGUAGES.iter().enumerate() {
        let selected = gs.language == code;
        let (bg, border) = if selected {
            (SETTINGS_LANG_SELECTED_BG, SETTINGS_LANG_SELECTED_BORDER)
        } else {
            (SETTINGS_BTN_BG, SETTINGS_BTN_BORDER)
        };
        parent.spawn((
            Button, SettingsLangBtn(i),
            Node {
                padding: UiRect::axes(Val::Px(SETTINGS_BTN_PAD_X), Val::Px(SETTINGS_BTN_PAD_Y)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(SETTINGS_BTN_CORNER)),
                ..default()
            },
            BackgroundColor(Color::srgba(bg.0, bg.1, bg.2, bg.3)),
            BorderColor::all(Color::srgba(border.0, border.1, border.2, border.3)),
        )).with_child((
            Text::new(name),
            TextFont { font: font.clone(), font_size: SETTINGS_BTN_FONT, ..default() },
            TextColor(Color::srgb(
                SETTINGS_BTN_COLOR.0, SETTINGS_BTN_COLOR.1, SETTINGS_BTN_COLOR.2,
            )),
        ));
    }
    note(parent, font, "Dialog translations are a work in progress.");
}

fn spawn_display_content(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    heading(parent, font, "Display");
    coming_soon(parent, font);
    body(parent, font, "Fullscreen toggle");
    body(parent, font, "Bloom intensity");
    body(parent, font, "Text size: Normal / Large");
}

fn spawn_audio_content(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    heading(parent, font, "Audio");
    coming_soon(parent, font);
    body(parent, font, "Music volume");
    body(parent, font, "SFX volume");
}

fn spawn_game_content(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>, gs: &GameState,
) {
    heading(parent, font, "Game");
    body(parent, font, &format!("World seed: {:016X}", gs.world_seed));
    note(parent, font, "This is your current world seed (read-only).");
    parent.spawn(Node { height: Val::Px(10.0), ..default() });
    settings_seed::spawn_seed_input(parent, font);
    parent.spawn(Node { height: Val::Px(12.0), ..default() });
    parent.spawn((
        Button, SettingsResetBtn,
        Node {
            padding: UiRect::axes(Val::Px(SETTINGS_BTN_PAD_X), Val::Px(SETTINGS_BTN_PAD_Y)),
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(SETTINGS_BTN_CORNER)),
            ..default()
        },
        BackgroundColor(Color::srgba(
            SETTINGS_DANGER_BG.0, SETTINGS_DANGER_BG.1,
            SETTINGS_DANGER_BG.2, SETTINGS_DANGER_BG.3,
        )),
        BorderColor::all(Color::srgba(
            SETTINGS_DANGER_BORDER.0, SETTINGS_DANGER_BORDER.1,
            SETTINGS_DANGER_BORDER.2, SETTINGS_DANGER_BORDER.3,
        )),
    )).with_child((
        Text::new("Reset All Progress"),
        TextFont { font: font.clone(), font_size: SETTINGS_BTN_FONT, ..default() },
        TextColor(Color::srgb(
            SETTINGS_DANGER_COLOR.0, SETTINGS_DANGER_COLOR.1, SETTINGS_DANGER_COLOR.2,
        )),
    ));
}

// ── Text helpers ───────────────────────────────────────────────────────

fn heading(parent: &mut ChildSpawnerCommands, font: &Handle<Font>, text: &str) {
    parent.spawn((
        Text::new(text),
        TextFont { font: font.clone(), font_size: SETTINGS_HEADING_FONT, ..default() },
        TextColor(Color::srgb(
            SETTINGS_HEADING_COLOR.0, SETTINGS_HEADING_COLOR.1, SETTINGS_HEADING_COLOR.2,
        )),
    ));
}

fn body(parent: &mut ChildSpawnerCommands, font: &Handle<Font>, text: &str) {
    parent.spawn((
        Text::new(text),
        TextFont { font: font.clone(), font_size: SETTINGS_BODY_FONT, ..default() },
        TextColor(Color::srgb(
            SETTINGS_BODY_COLOR.0, SETTINGS_BODY_COLOR.1, SETTINGS_BODY_COLOR.2,
        )),
    ));
}

fn note(parent: &mut ChildSpawnerCommands, font: &Handle<Font>, text: &str) {
    parent.spawn((
        Text::new(text),
        TextFont { font: font.clone(), font_size: SETTINGS_NOTE_FONT, ..default() },
        TextColor(Color::srgb(
            SETTINGS_NOTE_COLOR.0, SETTINGS_NOTE_COLOR.1, SETTINGS_NOTE_COLOR.2,
        )),
        Node { margin: UiRect::top(Val::Px(6.0)), ..default() },
    ));
}

fn coming_soon(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent.spawn((
        Text::new("Coming Soon"),
        TextFont { font: font.clone(), font_size: SETTINGS_NOTE_FONT, ..default() },
        TextColor(Color::srgb(
            SETTINGS_COMING_SOON_COLOR.0, SETTINGS_COMING_SOON_COLOR.1,
            SETTINGS_COMING_SOON_COLOR.2,
        )),
        Node { margin: UiRect::bottom(Val::Px(4.0)), ..default() },
    ));
}

/// Spawn the confirmation dialog for progress reset.
pub fn spawn_confirm_dialog(commands: &mut Commands, font: &Handle<Font>) {
    commands.spawn((
        ConfirmDialog,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        GlobalZIndex(50),
    )).with_children(|overlay| {
        overlay.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(28.0)),
                row_gap: Val::Px(16.0),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.10, 0.08, 0.08, 0.98)),
            BorderColor::all(Color::srgba(0.7, 0.2, 0.2, 0.5)),
            Button, Interaction::None,
        )).with_children(|panel| {
            panel.spawn((
                Text::new("This will delete ALL progress. Are you sure?"),
                TextFont { font: font.clone(), font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.95, 0.85, 0.8)),
            ));
            panel.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                ..default()
            }).with_children(|row| {
                row.spawn((
                    Button, SettingsCancelReset,
                    Node {
                        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.15, 0.16, 0.20, 0.9)),
                    BorderColor::all(Color::srgba(0.3, 0.32, 0.4, 0.6)),
                )).with_child((
                    Text::new("Cancel"),
                    TextFont { font: font.clone(), font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.7, 0.75, 0.85)),
                ));
                row.spawn((
                    Button, SettingsConfirmReset,
                    Node {
                        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.35, 0.08, 0.08, 0.9)),
                    BorderColor::all(Color::srgba(0.8, 0.2, 0.2, 0.7)),
                )).with_child((
                    Text::new("Delete Everything"),
                    TextFont { font: font.clone(), font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.95, 0.4, 0.4)),
                ));
            });
        });
    });
}
