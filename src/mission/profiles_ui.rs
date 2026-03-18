// SPDX-License-Identifier: GPL-3.0-or-later

//! Profile selection screen — UI layout and components.
//! Systems live in profiles_ui_systems.rs.

use bevy::prelude::*;
use super::constants::*;
use super::types::*;
use super::profiles::*;

// ── Constants (pub for systems module) ──────────────────────────────────

pub const PROFILE_TITLE_FONT: f32 = 36.0;
pub const PROFILE_TITLE_COLOR: (f32, f32, f32) = (0.85, 0.88, 0.95);
pub const PROFILE_CARD_WIDTH: f32 = 420.0;
pub const PROFILE_CARD_HEIGHT: f32 = 72.0;
pub const PROFILE_CARD_BG: (f32, f32, f32, f32) = (0.08, 0.09, 0.14, 0.85);
pub const PROFILE_CARD_HOVER_BG: (f32, f32, f32, f32) = (0.14, 0.16, 0.24, 0.95);
pub const PROFILE_CARD_BORDER: (f32, f32, f32, f32) = (0.22, 0.25, 0.35, 0.5);
pub const PROFILE_CARD_HOVER_BORDER: (f32, f32, f32, f32) = (0.5, 0.6, 0.85, 0.8);
pub const PROFILE_CARD_CORNER: f32 = 10.0;
pub const PROFILE_CARD_GAP: f32 = 10.0;
pub const PROFILE_NAME_FONT: f32 = 18.0;
pub const PROFILE_NAME_COLOR: (f32, f32, f32) = (0.9, 0.92, 1.0);
pub const PROFILE_DETAIL_FONT: f32 = 13.0;
pub const PROFILE_DETAIL_COLOR: (f32, f32, f32) = (0.55, 0.6, 0.7);
pub const PROFILE_EMPTY_COLOR: (f32, f32, f32) = (0.4, 0.45, 0.55);
pub const PROFILE_GLOW_COLOR: (f32, f32, f32, f32) = (0.4, 0.5, 0.8, 0.2);
pub const PROFILE_DELETE_FONT: f32 = 12.0;
pub const PROFILE_DELETE_COLOR: (f32, f32, f32) = (0.6, 0.35, 0.35);
pub const PROFILE_DELETE_HOVER_COLOR: (f32, f32, f32) = (0.95, 0.4, 0.4);
pub const PROFILE_FADE_DURATION: f32 = 0.8;
pub const PROFILE_FADE_OUT_DURATION: f32 = 0.6;

// ── Components ──────────────────────────────────────────────────────────

#[derive(Component)]
pub struct ProfileSelectRoot;

#[derive(Component)]
pub struct ProfileSlot(pub usize); // 1-5

#[derive(Component)]
pub struct ProfileNameText(pub usize);

#[derive(Component)]
pub struct ProfileDetailText(#[allow(dead_code)] pub usize);

#[derive(Component)]
pub struct ProfileDeleteBtn(pub usize);

#[derive(Component)]
pub struct ProfileFadeOverlay;

#[derive(Component)]
pub struct ProfileDeleteConfirm(#[allow(dead_code)] pub usize);

#[derive(Component, Clone, Copy)]
pub enum ProfileConfirmBtn {
    YesDelete(usize),
    NoCancel,
}

#[derive(Component)]
pub struct ProfileTitleText;

// ── Resources ───────────────────────────────────────────────────────────

/// State for the profile selection screen.
#[derive(Resource)]
pub struct ProfileSelectState {
    pub fade_in_timer: f32,
    pub fade_out_active: bool,
    pub fade_out_timer: f32,
    pub selected_profile: usize,
    pub rename_active: bool,
    pub rename_index: usize,
    pub rename_text: String,
}

impl Default for ProfileSelectState {
    fn default() -> Self {
        Self {
            fade_in_timer: 0.0,
            fade_out_active: false,
            fade_out_timer: 0.0,
            selected_profile: 0,
            rename_active: false,
            rename_index: 0,
            rename_text: String::new(),
        }
    }
}

// ── Spawn ───────────────────────────────────────────────────────────────

/// Spawn the profile selection screen UI.
pub fn enter_profile_select(mut commands: Commands, font: Res<MissionFont>) {
    let font = &font.0;
    let profiles = load_all_profiles();

    commands.insert_resource(ProfileSelectState::default());

    commands.spawn((
        ProfileSelectRoot,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        GlobalZIndex(10),
    )).with_children(|root| {
        root.spawn((
            ProfileTitleText,
            Text::new("Who\u{2019}s playing?"),
            TextFont {
                font: font.clone(),
                font_size: PROFILE_TITLE_FONT,
                ..default()
            },
            TextColor(Color::srgba(
                PROFILE_TITLE_COLOR.0, PROFILE_TITLE_COLOR.1,
                PROFILE_TITLE_COLOR.2, 0.0,
            )),
            Node { margin: UiRect::bottom(Val::Px(12.0)), ..default() },
        ));

        for (i, info) in profiles.iter().enumerate() {
            spawn_profile_card(root, font, i + 1, info);
        }
    });

    // Fade overlay
    commands.spawn((
        ProfileFadeOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        GlobalZIndex(50),
    ));
}

fn spawn_profile_card(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    index: usize,
    info: &ProfileInfo,
) {
    parent.spawn((
        Button,
        ProfileSlot(index),
        Node {
            width: Val::Px(PROFILE_CARD_WIDTH),
            height: Val::Px(PROFILE_CARD_HEIGHT),
            padding: UiRect::axes(Val::Px(20.0), Val::Px(12.0)),
            border: UiRect::all(Val::Px(1.5)),
            border_radius: BorderRadius::all(Val::Px(PROFILE_CARD_CORNER)),
            margin: UiRect::vertical(Val::Px(PROFILE_CARD_GAP / 2.0)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        BackgroundColor(Color::srgba(
            PROFILE_CARD_BG.0, PROFILE_CARD_BG.1,
            PROFILE_CARD_BG.2, 0.0,
        )),
        BorderColor::all(Color::srgba(
            PROFILE_CARD_BORDER.0, PROFILE_CARD_BORDER.1,
            PROFILE_CARD_BORDER.2, 0.0,
        )),
        BoxShadow::new(
            Color::srgba(
                PROFILE_GLOW_COLOR.0, PROFILE_GLOW_COLOR.1,
                PROFILE_GLOW_COLOR.2, 0.0,
            ),
            Val::ZERO, Val::ZERO,
            Val::Px(5.0), Val::Px(12.0),
        ),
    )).with_children(|card| {
        // Left column: name + details
        card.spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        }).with_children(|left| {
            let nc = if info.exists { PROFILE_NAME_COLOR } else { PROFILE_EMPTY_COLOR };
            left.spawn((
                ProfileNameText(index),
                Text::new(&info.name),
                TextFont { font: font.clone(), font_size: PROFILE_NAME_FONT, ..default() },
                TextColor(Color::srgba(nc.0, nc.1, nc.2, 0.0)),
            ));

            let detail = if info.exists {
                format!(
                    "Day {} \u{00b7} Level {} \u{00b7} {} \u{00b7} {} crew found",
                    info.day, info.bot_level, info.collapse_type, info.crew_discovered
                )
            } else {
                "New Player \u{2014} Click to begin".to_string()
            };
            left.spawn((
                ProfileDetailText(index),
                Text::new(&detail),
                TextFont { font: font.clone(), font_size: PROFILE_DETAIL_FONT, ..default() },
                TextColor(Color::srgba(
                    PROFILE_DETAIL_COLOR.0, PROFILE_DETAIL_COLOR.1,
                    PROFILE_DETAIL_COLOR.2, 0.0,
                )),
            ));
        });

        // Right: delete button (only for existing profiles)
        if info.exists {
            card.spawn((
                Button,
                ProfileDeleteBtn(index),
                Node {
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_child((
                Text::new("Delete"),
                TextFont { font: font.clone(), font_size: PROFILE_DELETE_FONT, ..default() },
                TextColor(Color::srgb(
                    PROFILE_DELETE_COLOR.0, PROFILE_DELETE_COLOR.1,
                    PROFILE_DELETE_COLOR.2,
                )),
            ));
        }
    });
}

/// Spawn delete confirmation dialog.
pub fn spawn_delete_confirm(
    commands: &mut Commands, font: &Handle<Font>, index: usize,
) {
    let info = load_profile_info(index);
    commands.spawn((
        ProfileDeleteConfirm(index),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        GlobalZIndex(60),
    )).with_children(|overlay| {
        overlay.spawn((
            Node {
                width: Val::Px(400.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(28.0)),
                row_gap: Val::Px(20.0),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(
                MENU_CONFIRM_BG.0, MENU_CONFIRM_BG.1,
                MENU_CONFIRM_BG.2, MENU_CONFIRM_BG.3,
            )),
            BorderColor::all(Color::srgba(
                MENU_CONFIRM_BORDER.0, MENU_CONFIRM_BORDER.1,
                MENU_CONFIRM_BORDER.2, MENU_CONFIRM_BORDER.3,
            )),
        )).with_children(|panel| {
            panel.spawn((
                Text::new(format!(
                    "Delete \"{}\"?\n\nDay {}, Level {}\nThis cannot be undone.",
                    info.name, info.day, info.bot_level
                )),
                TextFont { font: font.clone(), font_size: 16.0, ..default() },
                TextColor(Color::srgb(
                    MENU_CONFIRM_TEXT_COLOR.0, MENU_CONFIRM_TEXT_COLOR.1,
                    MENU_CONFIRM_TEXT_COLOR.2,
                )),
                Node { max_width: Val::Px(340.0), ..default() },
            ));

            panel.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                ..default()
            }).with_children(|row| {
                spawn_confirm_btn(row, font, "Yes, delete",
                    ProfileConfirmBtn::YesDelete(index));
                spawn_confirm_btn(row, font, "No, keep",
                    ProfileConfirmBtn::NoCancel);
            });
        });
    });
}

fn spawn_confirm_btn(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    action: ProfileConfirmBtn,
) {
    parent.spawn((
        Button, action,
        Node {
            padding: UiRect::axes(Val::Px(20.0), Val::Px(12.0)),
            border: UiRect::all(Val::Px(1.5)),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(
            MENU_BUTTON_BG.0, MENU_BUTTON_BG.1,
            MENU_BUTTON_BG.2, MENU_BUTTON_BG.3,
        )),
        BorderColor::all(Color::srgba(
            MENU_BUTTON_BORDER_COLOR.0, MENU_BUTTON_BORDER_COLOR.1,
            MENU_BUTTON_BORDER_COLOR.2, MENU_BUTTON_BORDER_COLOR.3,
        )),
    )).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size: MENU_CONFIRM_BTN_FONT, ..default() },
        TextColor(Color::srgb(
            MENU_BUTTON_TEXT_COLOR.0, MENU_BUTTON_TEXT_COLOR.1,
            MENU_BUTTON_TEXT_COLOR.2,
        )),
    ));
}

/// Respawn profile UI from a World context (used after delete).
pub fn spawn_profiles_from_world(
    world: &mut World,
    font: &Handle<Font>,
    profiles: &[ProfileInfo],
) {
    let font = font.clone();
    let profiles: Vec<ProfileInfo> = profiles.to_vec();
    world.commands().spawn((
        ProfileSelectRoot,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        GlobalZIndex(10),
    )).with_children(|root| {
        root.spawn((
            ProfileTitleText,
            Text::new("Who\u{2019}s playing?"),
            TextFont { font: font.clone(), font_size: PROFILE_TITLE_FONT, ..default() },
            TextColor(Color::srgb(
                PROFILE_TITLE_COLOR.0, PROFILE_TITLE_COLOR.1, PROFILE_TITLE_COLOR.2,
            )),
            Node { margin: UiRect::bottom(Val::Px(12.0)), ..default() },
        ));
        for (i, info) in profiles.iter().enumerate() {
            spawn_profile_card(root, &font, i + 1, info);
        }
    });

    world.commands().spawn((
        ProfileFadeOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        GlobalZIndex(50),
    ));
}
