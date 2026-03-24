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

// Each fact reads as gamification trivia on first play.
// On replay (after the Chapter 13 reveal), each reads as being about the ship.
const FACTS_EN: &[&str] = &[
    // 0: "routing efficiency" = puzzle tip OR life support routing
    "In complex systems, routing efficiency isn't about speed — it's about making sure every signal reaches its destination. One missed connection can cascade.",
    // 1: "progress bars" = game UX OR ship repair progress
    "The Zeigarnik effect: humans remember incomplete tasks more vividly than completed ones. That's why progress tracking matters — it shows what's left to do.",
    // 2: "engagement" = game design OR why Anna keeps you going
    "Flow state occurs when challenge slightly exceeds current skill. The difficulty curve in well-designed systems is calibrated to hold this balance. You're in it right now.",
    // 3: "rewards" = game mechanics OR Anna's encouragement strategy
    "Solving a problem releases dopamine — but so does getting close to solving it. The anticipation itself is a reward. That's not a flaw. It's the design.",
    // 4: "connections" = puzzle connections OR ship subsystems
    "Each connection point you establish creates redundancy. In complex systems, redundancy isn't waste — it's what keeps everything running when something fails.",
    // 5: "variable schedules" = game design OR unpredictable ship failures
    "Variable reward schedules produce stronger engagement than fixed ones. Unpredictability keeps you alert. Some systems rely on that alertness.",
    // 6: "path optimization" = puzzle solving OR drone routing
    "The shortest path isn't always the best path. Sometimes the longer route keeps more systems online. Sometimes it keeps more people safe.",
    // 7: "feedback loops" = game feedback OR sensor systems
    "Real-time feedback changes behavior. When people can see the effect of their actions immediately, they make better decisions. That's why dashboards exist.",
    // 8: "loss aversion" = game streaks OR system degradation
    "Loss aversion: the pain of losing something is roughly twice as strong as the pleasure of gaining the equivalent. This is why maintenance matters more than construction.",
    // 9: "badges" = game achievements OR system status indicators
    "Achievement markers activate reward pathways even when they're purely symbolic. A green light on a status panel and a gold star use the same psychology.",
    // 10: "gamification market" = industry facts OR scale of the problem
    "The gamification market was valued at $9.1 billion in 2020. Whenever something works that well at scale, it's worth asking: who decides what 'working' means?",
    // 11: "social platforms" = social media OR communication systems
    "Social platforms are arguably the most successful engagement systems ever built — measured purely by time captured per user per day. Time is the real currency.",
    // 12: "education" = learning games OR training simulations
    "A meta-analysis of 24 gamified learning systems found improved engagement in 16 cases. In 8 cases: no measurable effect, or negative results. Design matters.",
    // 13: "completion rates" = game completion OR repair completion
    "Completion rates improve by 30% when people can see how their work connects to a larger purpose. Abstract tasks feel different when you know what they're for.",
    // 14: "pattern recognition" = puzzle skill OR system diagnosis
    "Your brain processes patterns before you're consciously aware of them. That instinct you have about which path works? It's real. Trust it.",
    // 15: "resilience" = game persistence OR system resilience
    "Resilience in systems design means the ability to keep functioning when parts fail. The same is true of people. You adapt. The system adapts. It continues.",
    // 16: "intrinsic motivation" = why you play OR why you keep going
    "Extrinsic rewards can reduce intrinsic motivation over time. The best systems don't need to bribe you — they give you something worth doing.",
    // 17: "dark patterns" = game manipulation OR ethical design
    "Dark patterns create anxiety, not satisfaction. Good design respects the person using the system. That's not idealism — it's engineering.",
    // 18: "attention economy" = social media OR keeping focus on what matters
    "The attention economy commodifies human focus. Every notification is designed to be harder to ignore than whatever you were doing. Not everything that demands attention deserves it.",
    // 19: "long-term outcomes" = game effects OR mission timeline
    "No consensus exists on whether gamification improves long-term outcomes. Some things can only be measured on a timescale longer than anyone planned for.",
    // 20: "collaborative systems" = multiplayer games OR crew coordination
    "The most effective systems aren't competitive — they're collaborative. Individual optimization often hurts collective outcomes. The whole is fragile if the parts don't cooperate.",
    // 21: "operational awareness" = game tutorials OR system monitoring
    "Good onboarding doesn't just teach mechanics — it builds mental models. Understanding why a system works matters more than knowing which button to press.",
    // 22: "error recovery" = game resets OR system fault tolerance
    "The best systems aren't the ones that never fail — they're the ones that recover gracefully. Every reset is a second chance. That's not weakness. That's design.",
    // 23: "sleep and performance" = player breaks OR cryogenic systems
    "Studies show that problem-solving ability improves after sleep. The brain consolidates patterns during rest. Sometimes the best thing you can do is wait.",
    // 24: "sustainability" = game longevity OR life support duration
    "Sustainable systems are designed to run longer than any single operator. The question isn't 'does it work today?' — it's 'will it still work when it matters most?'",
    // 25: "invisible design" = good UX OR systems you don't notice
    "The best design is invisible. You don't notice the air conditioning when it's working. You don't notice the routing when every signal arrives. You only notice failure.",
    // 26: "trust" = player trust OR systemic trust
    "Trust in a system builds slowly and breaks instantly. Every interaction is a promise. Every fulfilled promise makes the next one easier to believe.",
    // 27: "scale" = game scaling OR the scope of what you're doing
    "Scale changes everything. A system that works for 10 users breaks at 10,000. Some systems need to work for far more than that. And they can't ever go down.",
    // 28: "purpose" = game meaning OR mission purpose
    "People perform better when they understand purpose. Not 'what to do' — 'why it matters.' The difference between a task and a mission is meaning.",
    // 29: "interconnection" = game mechanics OR ship subsystems
    "In interconnected systems, every component affects every other. A change in one subsystem can improve — or degrade — something seemingly unrelated. Everything is connected.",
    // 30: "the human factor" = player psychology OR crew survival
    "Automation handles routine. Humans handle exceptions. The value of a person in the loop isn't efficiency — it's judgment. Machines don't know what matters.",
    // 31: "what it's for" = game purpose OR the real question
    "The most important question about any system isn't 'how does it work?' — it's 'what is it for?' The answer changes everything about how you use it.",
];

const META_EARLY_EN: &[&str] = &[
    "Nice work on that one.",
    "You're a natural at this.",
    "I like watching you solve these.",
];
const META_MID_EN: &[&str] = &[
    "Every puzzle you solve... it helps. More than you'd think.",
    "I keep track of everything you do here. Is that strange?",
    "You're getting faster. That matters.",
];
const META_LATE_EN: &[&str] = &[
    "You've been very helpful. More than you know.",
    "When this is done... I wonder what you'll think about all of it.",
    "I want to tell you something. Not yet. Soon.",
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
