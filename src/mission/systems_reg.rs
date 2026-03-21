// SPDX-License-Identifier: GPL-3.0-or-later
// System registration helpers — split from mod.rs to keep it under 400 lines.

use bevy::prelude::*;
use super::types::{AppPhase, GameScene};
use super::{
    credits, credits_systems, dashboard, dialog_system, dialog_types, dialog_ui,
    endings, endings_anim, games, anna, loading_screen, main_menu, profiles_ui,
    profiles_ui_systems, questions, resources, settings_systems, settings_seed,
    stats_screen, decision_tree, twinkle_stars,
};

/// Run condition: AppPhase::Playing AND GameScene::Dashboard.
/// In non-full builds, GameScene sub-state may not exist, so we check optionally.
fn playing_dashboard(
    phase: Option<Res<State<AppPhase>>>,
    scene: Option<Res<State<GameScene>>>,
) -> bool {
    let playing = phase.is_some_and(|p| *p.get() == AppPhase::Playing);
    if !playing { return false; }
    // If GameScene state exists (full mode), only run on Dashboard
    // If it doesn't exist (standalone mission), always run
    scene.map_or(true, |s| *s.get() == GameScene::Dashboard)
}

pub fn register_profile_systems(app: &mut App) {
    app.add_systems(OnEnter(AppPhase::ProfileSelect), profiles_ui::enter_profile_select)
    .add_systems(Update, (
        profiles_ui_systems::animate_profile_fade_in,
        profiles_ui_systems::profile_slot_hover,
        profiles_ui_systems::profile_slot_click,
        profiles_ui_systems::profile_fade_out,
        profiles_ui_systems::profile_delete_hover,
        profiles_ui_systems::profile_delete_click,
        profiles_ui_systems::profile_confirm_click,
        profiles_ui_systems::profile_confirm_hover,
        profiles_ui_systems::profile_name_click,
        profiles_ui_systems::profile_rename_keyboard,
        twinkle_stars,
        main_menu::drift_menu_stars,
    ).run_if(in_state(AppPhase::ProfileSelect)))
    .add_systems(OnExit(AppPhase::ProfileSelect),
        profiles_ui_systems::cleanup_profile_select);
}

pub fn register_menu_systems(app: &mut App) {
    app.add_systems(OnEnter(AppPhase::MainMenu), super::enter_main_menu)
    .add_systems(Update, (
        main_menu::tick_menu_timer,
        main_menu::animate_menu_fade_in,
        main_menu::menu_button_hover,
        main_menu::menu_button_click,
        main_menu::confirm_button_click,
        main_menu::confirm_button_hover,
        main_menu::menu_fade_out,
        main_menu::drift_menu_stars,
    ).run_if(in_state(AppPhase::MainMenu)))
    .add_systems(Update, twinkle_stars.run_if(in_state(AppPhase::MainMenu)))
    .add_systems(Update, (
        stats_screen::stats_dismiss, stats_screen::animate_stats_glow,
        decision_tree::decision_tree_dismiss, decision_tree::animate_decision_tree_glow,
        decision_tree::parallax_system, decision_tree::decision_node_hover,
    ).run_if(in_state(AppPhase::MainMenu)));
}

pub fn register_loading_systems(app: &mut App) {
    app.add_systems(OnEnter(AppPhase::Loading), loading_screen::enter_loading)
    .add_systems(Update, (
        loading_screen::generate_world_during_loading,
        loading_screen::tick_loading_progress,
        twinkle_stars,
        main_menu::drift_menu_stars,
    ).run_if(in_state(AppPhase::Loading)))
    .add_systems(OnExit(AppPhase::Loading), loading_screen::cleanup_loading);
}

pub fn register_playing_systems(app: &mut App) {
    app.add_systems(OnEnter(AppPhase::Playing), super::enter_playing)
    // Dashboard-specific systems — only when on the Dashboard scene (or standalone)
    .add_systems(Update, (
        dashboard::animate_resource_bars,
        dashboard::update_status_texts,
        games::card_hover_interaction,
        games::card_click_interaction,
        games::poll_running_game,
        games::manage_game_overlay,
        resources::drain_resources,
        anna::update_anna_messages,
        anna::anna_click_dismiss,
        anna::update_anna_glow,
        twinkle_stars,
    ).run_if(playing_dashboard))
    .add_systems(Update, (
        questions::check_pending_question,
        questions::question_option_hover,
        questions::question_option_click,
        questions::update_reaction_overlay,
        super::final_voyage_click,
    ).run_if(playing_dashboard))
    .add_systems(Update, (
        dialog_system::check_dialog_triggers,
        dialog_system::start_next_dialog,
        dialog_system::update_typewriter,
        dialog_system::dialog_click_advance,
        dialog_system::dialog_choice_click,
        dialog_system::spawn_choices_when_ready,
        dialog_ui::dialog_choice_hover,
        dialog_ui::animate_dialog_glow,
        dialog_ui::animate_dialog_circle,
    ).run_if(playing_dashboard))
    .add_systems(Update, (
        endings_anim::animate_ending_screen,
        endings_anim::animate_ending_stats,
        endings_anim::animate_ending_glow,
    ).run_if(playing_dashboard
        .and(resource_exists::<endings::EndingState>)))
    .add_systems(Update, (
        endings_anim::new_journey_hover,
        endings_anim::new_journey_click,
    ).run_if(playing_dashboard
        .and(resource_exists::<endings::EndingState>)));
}

pub fn register_settings_systems(app: &mut App) {
    app.add_systems(Update, (
        settings_systems::toggle_settings,
        settings_systems::animate_settings_fade,
        settings_systems::dismiss_on_bg_click,
        settings_systems::tab_click,
        settings_systems::tab_hover,
        settings_systems::language_click,
        settings_systems::lang_btn_hover,
        settings_systems::reset_click,
        settings_systems::confirm_reset_click,
        settings_systems::reset_btn_hover,
        settings_seed::seed_input_click,
        settings_seed::seed_input_deactivate,
        settings_seed::seed_keyboard_input,
        settings_seed::seed_apply_click,
        settings_seed::seed_apply_hover,
    ));
}

pub fn register_credits_systems(app: &mut App) {
    app.add_systems(Update, (
        credits_systems::update_credits,
        credits_systems::credits_keyboard,
        credits_systems::cleanup_credits,
    ).run_if(in_state(AppPhase::MainMenu)
        .and(resource_exists::<credits::CreditsState>)));
}

/// Register integrated minigame systems (only in full mode).
#[cfg(feature = "full")]
pub fn register_integrated_game_systems(app: &mut App) {
    crate::gathering::integrated::register_integrated_systems(app);
    crate::converter::integrated::register_integrated_systems(app);
    crate::delivery::integrated::register_integrated_systems(app);
    crate::orben::integrated::register_integrated_systems(app);

    // Exit confirmation modal (shared across all minigames)
    app.insert_resource(super::exit_confirm::ExitConfirmOpen::default())
        .add_systems(Update, (
            super::exit_confirm::esc_open_confirm,
            super::exit_confirm::manage_confirm_overlay,
            super::exit_confirm::confirm_btn_click,
            super::exit_confirm::confirm_btn_hover,
        ).run_if(in_state(AppPhase::Playing)));

    // When returning from a game to dashboard, reload state.
    // Note: OnEnter(GameScene::Dashboard) also fires on initial Playing entry,
    // but enter_playing already handles that via OnEnter(AppPhase::Playing).
    // We use a resource flag to skip the first OnEnter(Dashboard).
    app.insert_resource(DashboardFirstEntry(true))
        .add_systems(OnEnter(GameScene::Dashboard), reenter_dashboard);
}

/// Flag to skip the first OnEnter(Dashboard) which fires alongside OnEnter(Playing).
#[cfg(feature = "full")]
#[derive(Resource)]
struct DashboardFirstEntry(bool);

/// When returning to Dashboard from a minigame, reload state from disk.
#[cfg(feature = "full")]
fn reenter_dashboard(
    mut first: ResMut<DashboardFirstEntry>,
    mut ship: ResMut<super::types::ShipStatus>,
    mut gs: ResMut<crate::save_state::GameState>,
    mut qs: ResMut<super::questions::QuestionState>,
    mut ds: ResMut<super::dialog_types::DialogState>,
) {
    // Skip the first time (enter_playing handles initial spawn)
    if first.0 {
        first.0 = false;
        return;
    }

    // Reload game state (minigame may have changed it)
    let fresh = crate::save_state::load_game_state();
    ship.power = fresh.power;
    ship.life_support = fresh.life_support;
    ship.cryo = fresh.cryo;
    ship.shields = fresh.shields;
    ship.repair = fresh.repair;
    ship.crystals = fresh.total_crystals();
    ship.crew_count = fresh.crew_count;
    ship.day = fresh.day;
    ship.distance_au = fresh.distance_au;
    ship.bot_level = fresh.bot_level;
    *gs = fresh;

    // Reset question and dialog checks
    super::questions::reset_question_check(&mut qs);
    super::dialog_system::reset_dialog_check(&mut ds);
}
