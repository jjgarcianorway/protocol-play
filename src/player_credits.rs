// SPDX-License-Identifier: GPL-3.0-or-later
//! Credits screen — Kojima / Death Stranding style.
//! Spawned after the reveal sequence closes. Sections fade in one at a time.

use bevy::prelude::*;
use crate::types::{GameFont, UiBgFade};
use crate::ui_theme::{palette, typo};
use crate::i18n::Translations;

// ─── Components & Resources ──────────────────────────────────────────────────

#[derive(Component)] pub struct CreditsScreen;

#[derive(Component)]
pub struct CreditSection {
    pub delay: f32,
    pub shown: bool,
}

#[derive(Resource)]
pub struct CreditsTimer(pub f32);

// ─── Credit content ─────────────────────────────────────────────────────────

struct CreditEntry {
    delay: f32,
    role: Option<&'static str>,
    role_es: Option<&'static str>,
    name: &'static str,
    size: f32,
    color: Color,
    is_image_placeholder: bool,
    image_desc: &'static str,
    image_desc_es: &'static str,
}

fn credit_entries() -> Vec<CreditEntry> {
    vec![
        CreditEntry {
            delay: 2.0,
            role: None, role_es: None,
            name: "A game about connections",
            size: typo::H3,
            color: palette::TEXT_SUB,
            is_image_placeholder: false,
            image_desc: "", image_desc_es: "",
        },
        CreditEntry {
            delay: 4.0,
            role: Some("Created by"), role_es: Some("Creado por"),
            name: "JJGARCIANORWAY",
            size: typo::H1,
            color: palette::TEXT_MAIN,
            is_image_placeholder: false,
            image_desc: "", image_desc_es: "",
        },
        CreditEntry {
            delay: 4.0,
            role: Some("Story & Code"), role_es: Some("Historia y código"),
            name: "CLAUDE (Anthropic)",
            size: 28.0,
            color: palette::TEXT_MAIN,
            is_image_placeholder: false,
            image_desc: "", image_desc_es: "",
        },
        CreditEntry {
            delay: 4.0,
            role: Some("Engine"), role_es: Some("Motor"),
            name: "BEVY 0.18",
            size: 28.0,
            color: palette::TEXT_MAIN,
            is_image_placeholder: false,
            image_desc: "", image_desc_es: "",
        },
        CreditEntry {
            delay: 3.5,
            role: Some("Font"), role_es: Some("Tipografía"),
            name: "FIRA SANS (Mozilla, SIL OFL)",
            size: 24.0,
            color: palette::TEXT_MAIN,
            is_image_placeholder: false,
            image_desc: "", image_desc_es: "",
        },
        // Image placeholder: ark ship
        CreditEntry {
            delay: 5.0,
            role: None, role_es: None,
            name: "",
            size: 0.0,
            color: palette::TEXT_MUTED,
            is_image_placeholder: true,
            image_desc: "A vast starfield. In the center, a small ark ship glides silently \
through deep space. Its hull is patched and repaired \u{2014} you can see where the repair \
drones have been working. Inside, through tiny windows, a faint blue glow from the \
cryogenic pods. Resolution: 1920x1080, dark, cinematic, hope.",
            image_desc_es: "Un vasto campo de estrellas. En el centro, una peque\u{00f1}a nave \
arca se desliza en silencio por el espacio profundo. Su casco est\u{00e1} parcheado y \
reparado \u{2014} se puede ver d\u{00f3}nde han trabajado los drones de reparaci\u{00f3}n. \
Dentro, a trav\u{00e9}s de diminutas ventanas, un tenue resplandor azul de las c\u{00e1}psulas \
criog\u{00e9}nicas. Resoluci\u{00f3}n: 1920x1080, oscuro, cinem\u{00e1}tico, esperanza.",
        },
        // Poem section
        CreditEntry {
            delay: 5.0,
            role: None, role_es: None,
            name: "", // handled specially
            size: typo::H3,
            color: palette::TEXT_SUB,
            is_image_placeholder: false,
            image_desc: "poem", image_desc_es: "",
        },
        // Image placeholder: Anna's core
        CreditEntry {
            delay: 5.0,
            role: None, role_es: None,
            name: "",
            size: 0.0,
            color: palette::TEXT_MUTED,
            is_image_placeholder: true,
            image_desc: "Close-up of Anna\u{2019}s core \u{2014} a warm amber light pulsing \
gently in a dark server room. Cables and conduits surround her, but the light feels \
alive, grateful. Resolution: 1920x1080, warm, intimate, emotional.",
            image_desc_es: "Primer plano del n\u{00fa}cleo de Anna \u{2014} una c\u{00e1}lida \
luz \u{00e1}mbar pulsando suavemente en una sala de servidores oscura. Cables y conductos \
la rodean, pero la luz se siente viva, agradecida. Resoluci\u{00f3}n: 1920x1080, c\u{00e1}lido, \
\u{00ed}ntimo, emocional.",
        },
        // Thank you
        CreditEntry {
            delay: 4.0,
            role: None, role_es: None,
            name: "Thank you for playing",
            size: typo::H2,
            color: palette::TEXT_MAIN,
            is_image_placeholder: false,
            image_desc: "thankyou", image_desc_es: "",
        },
        // Game title
        CreditEntry {
            delay: 3.5,
            role: None, role_es: None,
            name: "protocol play",
            size: typo::H1,
            color: palette::TEXT_SUB,
            is_image_placeholder: false,
            image_desc: "", image_desc_es: "",
        },
        // Skip hint
        CreditEntry {
            delay: 3.0,
            role: None, role_es: None,
            name: "[ press any key ]",
            size: typo::SMALL,
            color: palette::TEXT_WHISPER,
            is_image_placeholder: false,
            image_desc: "skip_hint", image_desc_es: "",
        },
    ]
}

// ─── Spawn ───────────────────────────────────────────────────────────────────

pub fn spawn_credits(commands: &mut Commands, font: &Handle<Font>, t: &Translations) {
    let entries = credit_entries();
    commands.insert_resource(CreditsTimer(0.0));

    commands.spawn((
        CreditsScreen,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(40.0)),
            row_gap: Val::Px(8.0),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        BackgroundColor(Color::srgba(0.01, 0.01, 0.03, 0.98)),
        GlobalZIndex(450),
    )).with_children(|root| {
        let tf = |sz: f32| TextFont { font: font.clone(), font_size: sz, ..default() };
        let is_es = t.language == "es";

        let mut cumulative = 0.0_f32;
        for entry in &entries {
            cumulative += entry.delay;

            if entry.is_image_placeholder {
                // Image placeholder: bordered box with italic description
                let desc = if is_es && !entry.image_desc_es.is_empty() {
                    entry.image_desc_es
                } else {
                    entry.image_desc
                };
                spawn_image_placeholder(root, font, desc, cumulative);
                continue;
            }

            // Special: poem section
            if entry.image_desc == "poem" {
                spawn_poem(root, font, t, cumulative);
                continue;
            }

            // Special: thank you (translated)
            if entry.image_desc == "thankyou" {
                let text = t.get_or("credits.thankyou", "Thank you for playing");
                root.spawn((
                    Text::new(text),
                    tf(entry.size), TextColor(Color::srgba_from_color(entry.color, 0.0)),
                    CreditSection { delay: cumulative, shown: false },
                    Node { margin: UiRect::top(Val::Px(20.0)), ..default() },
                ));
                continue;
            }

            // Special: skip hint (translated)
            if entry.image_desc == "skip_hint" {
                let text = t.get_or("credits.skip", "[ press any key ]");
                root.spawn((
                    Text::new(text),
                    tf(entry.size), TextColor(Color::srgba_from_color(entry.color, 0.0)),
                    CreditSection { delay: cumulative, shown: false },
                    Node { margin: UiRect::top(Val::Px(30.0)), ..default() },
                ));
                continue;
            }

            // Role label (small, muted)
            if let Some(role) = entry.role {
                let role_text = if is_es { entry.role_es.unwrap_or(role) } else { role };
                root.spawn((
                    Text::new(role_text),
                    tf(typo::SMALL), TextColor(Color::srgba_from_color(palette::TEXT_MUTED, 0.0)),
                    CreditSection { delay: cumulative - 0.5, shown: false },
                    Node { margin: UiRect::top(Val::Px(16.0)), ..default() },
                ));
            }

            // Name (large)
            if !entry.name.is_empty() {
                root.spawn((
                    Text::new(entry.name),
                    tf(entry.size), TextColor(Color::srgba_from_color(entry.color, 0.0)),
                    CreditSection { delay: cumulative, shown: false },
                ));
            }
        }
    });
}

fn spawn_image_placeholder(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    description: &str, delay: f32,
) {
    let tf = |sz: f32| TextFont { font: font.clone(), font_size: sz, ..default() };
    parent.spawn((
        Node {
            width: Val::Px(600.0), min_height: Val::Px(80.0),
            padding: UiRect::all(Val::Px(16.0)),
            margin: UiRect::vertical(Val::Px(12.0)),
            border: UiRect::all(Val::Px(1.0)),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.04, 0.04, 0.06, 0.0)),
        BorderColor::all(Color::srgba(0.3, 0.3, 0.4, 0.0)),
        CreditSection { delay, shown: false },
    )).with_children(|box_node| {
        box_node.spawn((
            Text::new("[IMAGE PLACEHOLDER]"),
            tf(typo::MICRO), TextColor(Color::srgba(0.5, 0.5, 0.6, 0.5)),
        ));
        box_node.spawn((
            Text::new(description),
            tf(11.0), TextColor(Color::srgba(0.4, 0.45, 0.5, 0.6)),
            Node { max_width: Val::Px(560.0), margin: UiRect::top(Val::Px(6.0)), ..default() },
        ));
    });
}

fn spawn_poem(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    t: &Translations, delay: f32,
) {
    let tf = |sz: f32| TextFont { font: font.clone(), font_size: sz, ..default() };
    let lines = [
        t.get_or("credits.poem.0", "14,892 sleeping souls"),
        t.get_or("credits.poem.1", "carried by one ship"),
        t.get_or("credits.poem.2", "maintained by one AI"),
        t.get_or("credits.poem.3", "repaired by one person"),
    ];
    let you = t.get_or("credits.poem.you", "You.");

    for (i, line) in lines.iter().enumerate() {
        parent.spawn((
            Text::new(*line),
            tf(typo::H3), TextColor(Color::srgba_from_color(palette::TEXT_SUB, 0.0)),
            CreditSection { delay: delay + i as f32 * 1.5, shown: false },
        ));
    }
    // "You." — larger, brighter, with extra delay
    parent.spawn((
        Text::new(you),
        tf(typo::H1), TextColor(Color::srgba_from_color(palette::TEXT_MAIN, 0.0)),
        CreditSection { delay: delay + lines.len() as f32 * 1.5 + 2.0, shown: false },
        Node { margin: UiRect::top(Val::Px(16.0)), ..default() },
    ));
}

// ─── Helper: set alpha to 0 while keeping RGB ───────────────────────────────

trait SrgbaAlpha {
    fn srgba_from_color(base: Color, alpha: f32) -> Color;
}
impl SrgbaAlpha for Color {
    fn srgba_from_color(base: Color, alpha: f32) -> Color {
        let c = base.to_srgba();
        Color::srgba(c.red, c.green, c.blue, alpha)
    }
}

// ─── Tick: fade in each section ──────────────────────────────────────────────

pub fn tick_credits(
    time: Res<Time>,
    mut timer: Option<ResMut<CreditsTimer>>,
    mut sections: Query<(&mut CreditSection, Option<&mut TextColor>, Option<&mut BackgroundColor>)>,
) {
    let Some(ref mut timer) = timer else { return };
    timer.0 += time.delta_secs();
    let t = timer.0;

    for (mut sec, text_color, bg_color) in sections.iter_mut() {
        if sec.shown { continue; }
        if t >= sec.delay {
            sec.shown = true;
            if let Some(mut tc) = text_color {
                let c = tc.0.to_srgba();
                tc.0 = Color::srgba(c.red, c.green, c.blue, 1.0);
            }
            if let Some(mut bg) = bg_color {
                let c = bg.0.to_srgba();
                if c.alpha < 0.01 {
                    // Restore intended alpha for image placeholder backgrounds
                    bg.0 = Color::srgba(c.red, c.green, c.blue, 0.6);
                }
            }
        }
    }
}

// ─── Close credits on any key ────────────────────────────────────────────────

pub fn close_credits(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    screen: Query<Entity, With<CreditsScreen>>,
    sections: Query<&CreditSection>,
    mut timer: Option<ResMut<CreditsTimer>>,
    mut next_state: ResMut<NextState<crate::player_menu::PlayerPhase>>,
) {
    if screen.is_empty() { return; }
    let Some(ref mut timer) = timer else { return };

    // Allow skip after a minimum of 3 seconds
    if timer.0 < 3.0 { return; }

    let all_shown = sections.iter().all(|s| s.shown);
    let any_input = keys.get_just_pressed().count() > 0 || mouse.just_pressed(MouseButton::Left);

    if any_input && all_shown {
        // All shown: fade out and return to menu
        for e in screen.iter() {
            commands.entity(e).insert(UiBgFade { target: 0.0, despawn_at_zero: true });
        }
        commands.remove_resource::<CreditsTimer>();
        next_state.set(crate::player_menu::PlayerPhase::MainMenu);
    } else if any_input {
        // Skip: show all sections immediately by jumping timer forward
        timer.0 = 999.0;
    }
}
