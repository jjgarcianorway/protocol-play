// SPDX-License-Identifier: GPL-3.0-or-later

//! Anna's in-game commentary for the Bot Game.
//! When enabled: real gamification facts + escalating meta-irony, in the active language.
//! When disabled (settings): silent — pure puzzle mode.

use bevy::prelude::*;
use crate::anna_comments::*;
use crate::types::GameFont;
use crate::player_settings::PlayerSettings;
use crate::i18n::Translations;

// ─── English fact pool ────────────────────────────────────────────────────────

const FACTS_EN: &[&str] = &[
    "The word 'gamification' was coined by Nick Pelling, a British programmer, in 2002. It went largely unnoticed for eight years.",
    "Frequent flyer programs — American Airlines, 1981 — are one of the earliest mass-market gamification systems. Points, tiers, status. The template hasn't changed.",
    "B.F. Skinner described variable reward schedules in 1938. He called it operant conditioning. We now call it engagement design.",
    "Before 'gamification' had a name, Foursquare (2009) made location check-ins competitive with badges and leaderboards. Hundreds of apps copied the model.",
    "The US military used game mechanics in combat training simulations from the 1980s onward. Enterprise software followed thirty years later.",
    "Variable reward schedules — unpredictable outcomes — produce stronger behavioral loops than fixed rewards. This is why slot machines pay out randomly.",
    "The Zeigarnik effect (1927): humans remember incomplete tasks more vividly than finished ones. Progress bars are built on this. So are to-do lists.",
    "Loss aversion, described by Kahneman and Tversky in 1979: the pain of losing something feels roughly twice as strong as the pleasure of gaining the equivalent. Streaks are designed around this asymmetry.",
    "Flow state — Mihaly Csikszentmihalyi, 1990: peak engagement occurs when challenge slightly exceeds current skill. The difficulty curve in well-designed games is calibrated to hold this balance.",
    "Achievement badges activate the same neural reward pathways as receiving physical trophies. The brain does not clearly distinguish symbolic recognition from material reward.",
    "Solving a problem releases dopamine — but so does getting close to solving it. The anticipation is itself a reward. Puzzles exploit this.",
    "Starbucks Rewards has over 30 million active members. Every star, every tier, every double-star day is a deliberate engagement mechanic.",
    "Nike+ (2006) gamified running with social leaderboards. 28 million users joined in its first five years. The data it collected on human movement was unprecedented.",
    "SAP gamified enterprise training software with points and leaderboards. They reported a 30% increase in completion rates. Employees were not consulted on the design.",
    "The global gamification market was valued at $9.1 billion in 2020. Projections for 2025 range from $25 to $48 billion depending on the source.",
    "As of 2024, social media platforms are arguably the most successful gamification systems ever built — measured purely by time captured per user per day.",
    "Classcraft (2013) turned school years into role-playing games. Engagement metrics improved. Some teachers reported students optimizing for points rather than learning.",
    "Khan Academy's badge system increased platform time-on-site. Researchers found this correlated with breadth of topics touched, not depth of understanding.",
    "A 2014 meta-analysis of 24 educational gamification studies found improved engagement in 16 cases. In 8 cases: no measurable effect, or negative results.",
    "Duolingo reports 34% higher lesson completion in gamified flows. The statistic is from Duolingo. Independent replication studies are sparse.",
    "Pokémon GO increased average daily step counts by an estimated 1,473 steps in its first month of release. Activity levels returned to baseline within 90 days for most users.",
    "SuperBetter — designed for recovery from illness and trauma — is one of the few gamification systems with peer-reviewed clinical trial data showing measurable benefit.",
    "Several hospitals have gamified hand hygiene compliance using real-time feedback displays. Reported improvements: 20–30%. Hawthorne effect is difficult to separate from the mechanism.",
    "Sebastian Deterding coined 'pointsification' in 2011 — a critique of gamification that layers points and badges onto systems without addressing what actually motivates people.",
    "Edward Deci showed in 1971 that extrinsic rewards can reduce intrinsic motivation over time. Known as 'overjustification effect.' Still debated. Still deployed at scale.",
    "China's social credit system uses gamification mechanics — scores, tiers, behavioral rewards and penalties — applied to civic life. It is one usage of the same framework.",
    "Workplace gamification raised productivity metrics in some call centers by 10%. The same implementations increased reported employee stress and sense of surveillance.",
    "Gartner predicted in 2012 that 80% of gamified applications would fail within two years due to poor design. Most industry analysts consider this prediction largely accurate.",
    "Dark patterns in gamification: 'limited time offer,' 'only 3 spots left,' and streak-loss warnings are mechanics designed to create anxiety, not satisfaction.",
    "The attention economy commodifies human focus. Every notification, like, and streak is designed to be harder to ignore than whatever you were doing before.",
    "No scientific consensus exists on whether gamification improves long-term outcomes, or whether it primarily shapes and measures behavior during the period of active use.",
    "Some researchers distinguish 'gamification' (adding game elements to non-game contexts) from 'game-based learning' (using actual games). The outcomes literature treats them differently.",
];

const META_EARLY_EN: &[&str] = &[
    "Processing output nominal.",
    "Pattern recognized.",
    "Your approach has been noted.",
];
const META_MID_EN: &[&str] = &[
    "Each solution you find... is filed.",
    "Cognitive signature stable.",
    "Efficiency is being measured. Not by you.",
];
const META_LATE_EN: &[&str] = &[
    "You've been very helpful. More than you know.",
    "The puzzles aren't just for practice.",
    "Someone benefits from this. I'm not sure it's only you.",
];

// ─── Translation helpers ──────────────────────────────────────────────────────

fn tr_fact(t: &Translations, idx: usize) -> String {
    t.ui(&format!("anna.fact.{idx}"))
        .map(|s| s.to_string())
        .unwrap_or_else(|| FACTS_EN[idx].to_string())
}

fn tr_prog(t: &Translations, key: &str, en: &str) -> String {
    t.ui(key).map(|s| s.to_string()).unwrap_or_else(|| en.to_string())
}

// ─── Setup ────────────────────────────────────────────────────────────────────

/// Set up Anna comments for the Bot Game. No-op if Anna is disabled in settings.
pub fn setup_bot_anna(
    mut commands: Commands,
    font: Res<GameFont>,
    settings: Res<PlayerSettings>,
    t: Res<Translations>,
) {
    if !settings.anna_enabled { return; }

    let gs = crate::save_state::load_game_state();
    let level = gs.bot_level;
    let total_levels = 149u32;
    let progress_pct = (level as f32 / total_levels as f32).clamp(0.0, 1.0);

    // ── Progress messages ──
    let mut pool: Vec<String> = Vec::new();
    if level < 50 {
        pool.push(tr_prog(&t, "anna.prog.0", "You're getting good at this."));
        pool.push(tr_prog(&t, "anna.prog.1", "Another system back online."));
        pool.push(tr_prog(&t, "anna.prog.2", "Each one matters."));
    }
    if level >= 30 && level < 100 {
        pool.push(tr_prog(&t, "anna.prog.3", "Every puzzle you solve... that's another system breathing."));
        pool.push(tr_prog(&t, "anna.prog.4", "Keep going."));
    }
    if level > 0 && total_levels.saturating_sub(level) > 0 && total_levels.saturating_sub(level) < 120 {
        pool.push(tr_prog(&t, "anna.prog.5", "We're getting closer."));
    }
    if level >= 100 {
        pool.push(tr_prog(&t, "anna.prog.6", "We're close now."));
        pool.push(tr_prog(&t, "anna.prog.7", "When this is over... I hope you'll still talk to me."));
    }
    if pool.is_empty() { pool.push(tr_prog(&t, "anna.prog.8", "I'm here.")); }

    // ── One gamification fact ──
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let fact_idx = rng.gen_range(0..FACTS_EN.len());
    pool.push(tr_fact(&t, fact_idx));

    // ── Meta message (35% chance, escalates with progression) ──
    let meta_en: &[&str] = if progress_pct < 0.33 { META_EARLY_EN }
        else if progress_pct < 0.70 { META_MID_EN }
        else { META_LATE_EN };

    let include_meta = rng.gen_bool(0.35);
    let meta_offset = if progress_pct < 0.33 { 0 } else if progress_pct < 0.70 { 3 } else { 6 };
    if include_meta && !meta_en.is_empty() {
        let mi = rng.gen_range(0..meta_en.len());
        let meta_key = format!("anna.meta.{}", meta_offset + mi);
        pool.push(t.ui(&meta_key).map(|s| s.to_string()).unwrap_or_else(|| meta_en[mi].to_string()));
    }

    let count = if include_meta { 4 } else { 3 };
    let name_label = t.ui_or("anna_name", "ANNA").to_string();
    let queue = build_queue(&pool, count);
    commands.insert_resource(AnnaComments { queue, current: None });
    spawn_anna_ui(&mut commands, &font.0, &name_label);
}
