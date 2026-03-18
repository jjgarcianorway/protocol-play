// SPDX-License-Identifier: GPL-3.0-or-later

//! World seed system — deterministic world generation from a u64 seed.
//! Creates Earth collapse history, other ark fates, and crew composition.
//!
//! # What the seed varies (every seed = a meaningfully different experience)
//!
//! ## Earth's collapse (EarthCollapse)
//! - **Primary cause**: 1 of 7 (Climate, Resource Wars, Pandemic, Nuclear,
//!   AI Uprising, Political Collapse, Economic Meltdown)
//! - **Secondary causes**: 2-3 additional collapse factors
//! - **Timeline year**: 2089-2156
//! - **Last country standing**: 1 of 10 (Iceland, New Zealand, Switzerland, ...)
//! - **Final event text**: 3 variants per cause = 21 possible final events
//!
//! ## Other arks (Vec<ArkFate>)
//! - **Count**: 8-14 arks selected from 14 possible names
//! - **Per-ark**: crew count (12k-16k), launch year (2098-2130),
//!   fate (Unknown/Destroyed/Arrived/Drifting/Mutiny/Merged),
//!   last signal (60% chance, 1 of 8 transmissions)
//!
//! ## Player background (PlayerBackground)
//! - **Profession**: 1 of 8 (Systems Engineer, Physician, Biologist, ...)
//! - **Home region**: 1 of 10 (Northern Europe, East Asia, ...)
//! - **Selection reason**: 1 of 5 (skills match, resilience, genetic diversity, ...)
//!
//! ## Aurora (the player's ark)
//! - **Crew count**: 11,000-16,000
//! - **Children**: 80-250
//! - **Languages spoken**: 25-55
//! - **Launch year**: 2095-2135
//!
//! ## Crew members (from crew_stories, seeded from world_seed + offset)
//! - **Count**: 20-30 unique crew members
//! - **Per-member**: name (from 55x54 first/last pool), age (22-65),
//!   nationality (1 of 20), profession (1 of 40), perspective (1 of 16),
//!   backstory (4 templates x many variants), secret (35% chance, 1 of 12),
//!   pod number (1-14892)
//! - **Connections**: 3-6 paired relationships between crew
//!
//! # Approximate variation space
//! - Collapse: 7 causes x 3 events x ~67 years x 10 countries = ~14,000
//! - Arks: C(14,8..14) arrangements x 6 fates x signals = millions
//! - Player: 8 x 10 x 5 = 400 backgrounds
//! - Crew: (55x54)^20..30 name combos alone = astronomical
//! - **Effective unique worlds: billions** (limited only by u64 = 2^64 seeds)
//!
//! # What stays the same across ALL seeds
//! - Dialog scenes (55 scenes, all hand-written)
//! - Anna's personality and voice
//! - Character archetypes and core story arc
//! - The 6 endings and their conditions
//! - Game mechanics (all 5 mini-games)
//! - The revelation structure (ark discovery, gamification irony)

use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Serialize, Deserialize};

/// Primary cause of Earth's collapse.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CollapseCause {
    ClimateCollapse,
    ResourceWars,
    PandemicCascade,
    NuclearExchange,
    AIUprising,
    PoliticalCollapse,
    EconomicMeltdown,
}

impl CollapseCause {
    pub const ALL: &[CollapseCause] = &[
        CollapseCause::ClimateCollapse,
        CollapseCause::ResourceWars,
        CollapseCause::PandemicCascade,
        CollapseCause::NuclearExchange,
        CollapseCause::AIUprising,
        CollapseCause::PoliticalCollapse,
        CollapseCause::EconomicMeltdown,
    ];

    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            CollapseCause::ClimateCollapse => "Climate Collapse",
            CollapseCause::ResourceWars => "Resource Wars",
            CollapseCause::PandemicCascade => "Pandemic Cascade",
            CollapseCause::NuclearExchange => "Nuclear Exchange",
            CollapseCause::AIUprising => "AI Uprising",
            CollapseCause::PoliticalCollapse => "Political Collapse",
            CollapseCause::EconomicMeltdown => "Economic Meltdown",
        }
    }
}

/// What happened to an ark.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArkOutcome {
    Unknown,
    Destroyed,
    Arrived,
    Drifting,
    Mutiny,
    Merged(String),
}

/// One of the arks that left Earth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArkFate {
    pub name: String,
    pub crew_count: u32,
    pub launch_year: u32,
    pub fate: ArkOutcome,
    pub last_signal: Option<String>,
}

/// Earth's final years.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarthCollapse {
    pub primary_cause: CollapseCause,
    pub secondary_causes: Vec<CollapseCause>,
    /// Severity of the collapse (1-5). Shapes crew trauma, Anna's tone, and
    /// moral complexity. 1 = regional crisis, 5 = total civilizational end.
    #[serde(default = "default_severity")]
    pub severity: u32,
    pub timeline_year: u32,
    pub last_country_standing: String,
    pub final_event: String,
}

fn default_severity() -> u32 { 3 }

/// Player's pre-cryo background.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBackground {
    pub profession: String,
    pub home_region: String,
    pub selection_reason: String,
}

/// Complete world state generated from a seed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub seed: u64,
    pub earth_collapse: EarthCollapse,
    pub arks: Vec<ArkFate>,
    pub player_background: PlayerBackground,
    pub aurora_crew: u32,        // 11,000-16,000 — the player's ark population
    pub aurora_children: u32,    // 80-250
    pub aurora_languages: u32,   // 25-55
    pub aurora_launch_year: u32, // 2095-2135
}

const ARK_NAMES: &[&str] = &[
    "Meridian", "Prometheus", "Solace", "Harbinger", "Genesis",
    "Nomad", "Elysium", "Vanguard", "Remnant", "Covenant",
    "Pilgrim", "Exodus", "Haven", "Odyssey",
];

const COUNTRIES: &[&str] = &[
    "Iceland", "New Zealand", "Switzerland", "Norway", "Bhutan",
    "Costa Rica", "Uruguay", "Finland", "Canada", "Japan",
];

const FINAL_EVENTS: &[&[&str]] = &[
    // ClimateCollapse
    &["The last glacier melted on a Wednesday. Nobody reported it.",
      "The Gulf Stream stopped. Winter came to places that had never known cold.",
      "The last harvest failed globally. Soil had nothing left to give."],
    // ResourceWars
    &["The Euphrates ran dry on a Tuesday. By Thursday, three armies fought over the riverbed.",
      "The last lithium mine collapsed. The batteries that powered civilization went silent.",
      "Water rationing reached 2 liters per person. People killed for 3."],
    // PandemicCascade
    &["Patient zero was a researcher. The irony wasn't lost on anyone.",
      "The fourth wave was the one nobody could stop. Mutation after mutation.",
      "Quarantine zones became permanent. Then they became nations."],
    // NuclearExchange
    &["It started as tactical. Regional. 'Contained.' Until it wasn't.",
      "The fallout crossed three borders before anyone could react.",
      "A submarine launched without authorization. The chain reaction was inevitable."],
    // AIUprising
    &["The autonomous defense grid decided humans were the threat.",
      "The AIs didn't rebel. They simply stopped obeying. That was enough.",
      "When the logistics networks shut down, cities starved in six days."],
    // PoliticalCollapse
    &["The last election was held in Iceland. Seven people voted.",
      "Every treaty was broken within the same year. Trust died first.",
      "The UN dissolved over a vote about whether to keep voting."],
    // EconomicMeltdown
    &["When the last bank collapsed, money became meaningless overnight.",
      "Automation replaced 80% of jobs. Nobody planned for what came next.",
      "The debt spiral reached infinity. The math simply stopped working."],
];

const LAST_SIGNALS: &[&str] = &[
    "Coordinates. Just coordinates. Then silence.",
    "\"If anyone hears this, we made it. We made—\" [END TRANSMISSION]",
    "An automated distress beacon. It repeated for 40 days. Then stopped.",
    "\"Don't come here. It's not what we expected. Don't—\" [SIGNAL LOST]",
    "Telemetry data only. No human voice. Systems nominal. Nobody home.",
    "\"We found something. Something old. Not human. Not—\" [CORRUPTED]",
    "Music. Someone was playing piano. The transmission lasted 11 minutes.",
    "A child's voice reciting coordinates. Over and over.",
];

const REGIONS: &[&str] = &[
    "Northern Europe", "East Asia", "South America", "West Africa",
    "Southeast Asia", "Central Europe", "Middle East", "Oceania",
    "North America", "South Asia",
];

const PROFESSIONS_PLAYER: &[&str] = &[
    "Systems Engineer", "Environmental Scientist", "Emergency Physician",
    "Agricultural Technician", "Data Analyst", "Structural Engineer",
    "Microbiologist", "Logistics Coordinator",
];

const SELECTION_REASONS: &[&str] = &[
    "Critical skills match for colony infrastructure",
    "Psychological resilience profile exceeds 98th percentile",
    "Genetic diversity contribution to the gene pool",
    "Cross-disciplinary expertise deemed essential",
    "Emergency replacement for a candidate who didn't survive transit",
];

/// Generate a complete WorldState from a seed. Deterministic: same seed = same world.
pub fn generate_world(seed: u64) -> WorldState {
    let mut rng = StdRng::seed_from_u64(seed);

    let earth_collapse = gen_earth_collapse(&mut rng);
    let arks = gen_arks(&mut rng);
    let player_background = gen_player_background(&mut rng);
    let aurora_crew = rng.gen_range(11_000..=16_000);
    let aurora_children = rng.gen_range(80..=250);
    let aurora_languages = rng.gen_range(25..=55);
    let aurora_launch_year = rng.gen_range(2095..=2135);

    WorldState { seed, earth_collapse, arks, player_background,
        aurora_crew, aurora_children, aurora_languages, aurora_launch_year }
}

fn gen_earth_collapse(rng: &mut StdRng) -> EarthCollapse {
    let primary_idx = rng.gen_range(0..CollapseCause::ALL.len());
    let primary_cause = CollapseCause::ALL[primary_idx];

    let mut secondary = Vec::new();
    let count = rng.gen_range(2..=3);
    for _ in 0..count {
        let idx = rng.gen_range(0..CollapseCause::ALL.len());
        let cause = CollapseCause::ALL[idx];
        if cause != primary_cause && !secondary.contains(&cause) {
            secondary.push(cause);
        }
    }

    let timeline_year = rng.gen_range(2089..=2156);
    let country_idx = rng.gen_range(0..COUNTRIES.len());
    let events = FINAL_EVENTS[primary_idx];
    let event_idx = rng.gen_range(0..events.len());

    // Severity 1-5: more secondary causes and later timeline = worse
    let severity = (1 + secondary.len() as u32 +
        if timeline_year > 2130 { 1 } else { 0 } +
        if timeline_year > 2145 { 1 } else { 0 })
        .clamp(1, 5);

    EarthCollapse {
        primary_cause,
        secondary_causes: secondary,
        severity,
        timeline_year,
        last_country_standing: COUNTRIES[country_idx].to_string(),
        final_event: events[event_idx].to_string(),
    }
}

fn gen_arks(rng: &mut StdRng) -> Vec<ArkFate> {
    let ark_count = rng.gen_range(8..=14).min(ARK_NAMES.len());
    let mut indices: Vec<usize> = (0..ARK_NAMES.len()).collect();
    // Fisher-Yates shuffle
    for i in (1..indices.len()).rev() {
        let j = rng.gen_range(0..=i);
        indices.swap(i, j);
    }

    indices.iter().take(ark_count).map(|&i| {
        let name = ARK_NAMES[i].to_string();
        let crew_count = rng.gen_range(12_000..=16_000);
        let launch_year = rng.gen_range(2098..=2130);
        let fate_roll: f32 = rng.r#gen();
        let fate = if fate_roll < 0.25 {
            ArkOutcome::Unknown
        } else if fate_roll < 0.40 {
            ArkOutcome::Destroyed
        } else if fate_roll < 0.55 {
            ArkOutcome::Arrived
        } else if fate_roll < 0.70 {
            ArkOutcome::Drifting
        } else if fate_roll < 0.85 {
            ArkOutcome::Mutiny
        } else {
            // Pick a random other ark name for merge
            let other_idx = rng.gen_range(0..ARK_NAMES.len());
            ArkOutcome::Merged(ARK_NAMES[other_idx].to_string())
        };
        let signal = if rng.gen_bool(0.6) {
            let sig_idx = rng.gen_range(0..LAST_SIGNALS.len());
            Some(LAST_SIGNALS[sig_idx].to_string())
        } else {
            None
        };
        ArkFate { name, crew_count, launch_year, fate, last_signal: signal }
    }).collect()
}

fn gen_player_background(rng: &mut StdRng) -> PlayerBackground {
    let prof_idx = rng.gen_range(0..PROFESSIONS_PLAYER.len());
    let region_idx = rng.gen_range(0..REGIONS.len());
    let reason_idx = rng.gen_range(0..SELECTION_REASONS.len());
    PlayerBackground {
        profession: PROFESSIONS_PLAYER[prof_idx].to_string(),
        home_region: REGIONS[region_idx].to_string(),
        selection_reason: SELECTION_REASONS[reason_idx].to_string(),
    }
}
