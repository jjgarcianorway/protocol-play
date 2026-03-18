// SPDX-License-Identifier: GPL-3.0-or-later
//! Main menu UI layout — title, buttons, quote, overlays.
use bevy::prelude::*;
use super::constants::*;
use super::main_menu::*;
use crate::save_state::load_game_state;

pub fn spawn_menu_ui(commands: &mut Commands, font: &Handle<Font>) {
    let has_save = save_exists();

    commands.spawn((
        MainMenuRoot,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(8.0),
            ..default()
        },
        GlobalZIndex(10),
    )).with_children(|root| {
        root.spawn((
            MenuTitleText,
            MenuFadeIn {
                start_time: MENU_TITLE_FADE_START,
                duration: MENU_TITLE_FADE_END - MENU_TITLE_FADE_START,
            },
            Text::new("protocol: play"),
            TextFont {
                font: font.clone(),
                font_size: MENU_TITLE_FONT,
                ..default()
            },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
        ));

        root.spawn((
            MenuFadeIn {
                start_time: MENU_TITLE_FADE_START + 0.5,
                duration: 1.5,
            },
            Text::new("A story about carrying each other"),
            TextFont {
                font: font.clone(),
                font_size: MENU_SUBTITLE_FONT,
                ..default()
            },
            TextColor(Color::srgba(
                MENU_SUBTITLE_COLOR.0, MENU_SUBTITLE_COLOR.1,
                MENU_SUBTITLE_COLOR.2, 0.0,
            )),
        ));

        root.spawn(Node { height: Val::Px(32.0), ..default() });
        let mut bi: usize = 0;
        if has_save {
            spawn_menu_button(root, font, "Continue", MenuButton::Continue, bi); bi += 1;
            spawn_menu_button(root, font, "New Game", MenuButton::NewGame, bi); bi += 1;
        } else {
            spawn_menu_button(root, font, "Begin", MenuButton::Begin, bi); bi += 1;
        }
        spawn_menu_button(root, font, "Settings", MenuButton::Settings, bi); bi += 1;
        if has_codex_content() {
            spawn_menu_button(root, font, "Crew Manifest", MenuButton::CrewManifest, bi); bi += 1;
        }
        // "Your Story" and "Journey Map" — only if the player has made progress
        let gs_check = load_game_state();
        if gs_check.bot_level > 0 || !gs_check.decisions.is_empty() {
            spawn_menu_button(root, font, "Your Story", MenuButton::YourStory, bi); bi += 1;
            spawn_menu_button(root, font, "Journey Map", MenuButton::JourneyMap, bi); bi += 1;
        }
        spawn_menu_button(root, font, "Credits", MenuButton::Credits, bi); bi += 1;
        spawn_menu_button(root, font, "Quit", MenuButton::Quit, bi);
        root.spawn(Node { height: Val::Px(40.0), ..default() });
        let quote = random_quote();
        root.spawn((
            MenuQuoteText,
            MenuFadeIn {
                start_time: MENU_QUOTE_FADE_START,
                duration: MENU_QUOTE_FADE_END - MENU_QUOTE_FADE_START,
            },
            Text::new(format!("\"{quote}\"")),
            TextFont {
                font: font.clone(),
                font_size: MENU_QUOTE_FONT,
                ..default()
            },
            TextColor(Color::srgba(
                MENU_QUOTE_COLOR.0, MENU_QUOTE_COLOR.1,
                MENU_QUOTE_COLOR.2, 0.0,
            )),
            Node {
                max_width: Val::Px(500.0),
                ..default()
            },
        ));
    });

    // --- Version label (bottom-right) ---
    commands.spawn((
        MenuVersionLabel,
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(6.0),
            bottom: Val::Px(4.0),
            ..default()
        },
        GlobalZIndex(10),
    )).with_child((
        Text::new(format!("Mission Control · v{}", env!("CARGO_PKG_VERSION"))),
        TextFont { font: font.clone(), font_size: VERSION_FONT_M, ..default() },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
    ));

    // --- Fade-out overlay (starts transparent) ---
    commands.spawn((
        MenuFadeOut,
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

fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    action: MenuButton,
    index: usize,
) {
    let fade_start = MENU_BUTTON_FADE_START + index as f32 * MENU_BUTTON_FADE_STAGGER;
    parent.spawn((
        Button,
        action,
        MenuFadeIn {
            start_time: fade_start,
            duration: 0.6,
        },
        Node {
            width: Val::Px(MENU_BUTTON_WIDTH),
            height: Val::Px(MENU_BUTTON_HEIGHT),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(MENU_BUTTON_BORDER)),
            border_radius: BorderRadius::all(Val::Px(MENU_BUTTON_CORNER)),
            margin: UiRect::vertical(Val::Px(MENU_BUTTON_GAP / 2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(
            MENU_BUTTON_BG.0, MENU_BUTTON_BG.1,
            MENU_BUTTON_BG.2, 0.0,
        )),
        BorderColor::all(Color::srgba(
            MENU_BUTTON_BORDER_COLOR.0, MENU_BUTTON_BORDER_COLOR.1,
            MENU_BUTTON_BORDER_COLOR.2, 0.0,
        )),
        BoxShadow::new(
            Color::srgba(
                MENU_BUTTON_HOVER_GLOW.0, MENU_BUTTON_HOVER_GLOW.1,
                MENU_BUTTON_HOVER_GLOW.2, 0.0,
            ),
            Val::ZERO, Val::ZERO,
            Val::Px(5.0), Val::Px(12.0),
        ),
    )).with_child((
        Text::new(label),
        TextFont {
            font: font.clone(),
            font_size: MENU_BUTTON_FONT,
            ..default()
        },
        TextColor(Color::srgba(
            MENU_BUTTON_TEXT_COLOR.0, MENU_BUTTON_TEXT_COLOR.1,
            MENU_BUTTON_TEXT_COLOR.2, 1.0,
        )),
    ));
}

pub fn spawn_new_game_submenu(commands: &mut Commands, font: &Handle<Font>) {
    commands.spawn((
        NewGameSubMenu,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        GlobalZIndex(55),
    )).with_children(|overlay| {
        overlay.spawn((
            Node {
                width: Val::Px(380.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                row_gap: Val::Px(14.0),
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
                Text::new("Start a new game"),
                TextFont { font: font.clone(), font_size: MENU_CONFIRM_FONT, ..default() },
                TextColor(Color::srgb(
                    MENU_CONFIRM_TEXT_COLOR.0, MENU_CONFIRM_TEXT_COLOR.1,
                    MENU_CONFIRM_TEXT_COLOR.2,
                )),
            ));
            spawn_submenu_btn(
                panel, font, "New Journey", "Same world, fresh start",
                MenuButton::NewJourney,
            );
            spawn_submenu_btn(
                panel, font, "New World", "New world seed, everything resets",
                MenuButton::NewWorld,
            );
        });
    });
}

fn spawn_submenu_btn(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    title: &str,
    subtitle: &str,
    action: MenuButton,
) {
    parent.spawn((
        Button,
        action,
        Node {
            width: Val::Px(300.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(20.0), Val::Px(12.0)),
            border: UiRect::all(Val::Px(MENU_BUTTON_BORDER)),
            border_radius: BorderRadius::all(Val::Px(MENU_BUTTON_CORNER)),
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
    )).with_children(|btn| {
        btn.spawn((
            Text::new(title),
            TextFont { font: font.clone(), font_size: MENU_BUTTON_FONT, ..default() },
            TextColor(Color::srgb(
                MENU_BUTTON_TEXT_COLOR.0, MENU_BUTTON_TEXT_COLOR.1,
                MENU_BUTTON_TEXT_COLOR.2,
            )),
        ));
        btn.spawn((
            Text::new(subtitle),
            TextFont { font: font.clone(), font_size: 12.0, ..default() },
            TextColor(Color::srgba(0.5, 0.55, 0.65, 0.7)),
        ));
    });
}

pub fn spawn_confirm_dialog(commands: &mut Commands, font: &Handle<Font>, new_world: bool) {
    let msg = if new_world {
        "This will erase ALL progress and generate\nan entirely new world. Your current journey\nwill be lost forever. Are you sure?"
    } else {
        "This will reset your journey but keep\nthe same world. Are you sure?"
    };
    let confirm_btn = if new_world {
        ConfirmButton::YesWorld
    } else {
        ConfirmButton::YesJourney
    };
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
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        GlobalZIndex(60),
    )).with_children(|overlay| {
        overlay.spawn((
            Node {
                width: Val::Px(420.0),
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
            // Warning text
            panel.spawn((
                Text::new(msg),
                TextFont {
                    font: font.clone(),
                    font_size: MENU_CONFIRM_FONT,
                    ..default()
                },
                TextColor(Color::srgb(
                    MENU_CONFIRM_TEXT_COLOR.0, MENU_CONFIRM_TEXT_COLOR.1,
                    MENU_CONFIRM_TEXT_COLOR.2,
                )),
                Node {
                    max_width: Val::Px(350.0),
                    ..default()
                },
            ));

            // Button row
            panel.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                ..default()
            }).with_children(|row| {
                spawn_confirm_btn(row, font, "Yes, start fresh", confirm_btn);
                spawn_confirm_btn(row, font, "No, go back", ConfirmButton::No);
            });
        });
    });
}

fn spawn_confirm_btn(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    action: ConfirmButton,
) {
    parent.spawn((
        Button,
        action,
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
        TextColor(Color::srgb(MENU_BUTTON_TEXT_COLOR.0, MENU_BUTTON_TEXT_COLOR.1, MENU_BUTTON_TEXT_COLOR.2)),
    ));
}

