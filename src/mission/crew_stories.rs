// SPDX-License-Identifier: GPL-3.0-or-later

//! Crew member generation — deterministic from world seed.
//! Generates 20-30 unique crew members with names, backstories, and perspectives.

use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Serialize, Deserialize};

/// A single crew member in cryogenic suspension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewMember {
    pub name: String,
    pub age: u32,
    pub nationality: String,
    pub profession: String,
    pub pod_number: u32,
    pub backstory: String,
    pub perspective: String,
    pub secret: Option<String>,
    pub connection: Option<usize>,
    pub alive: bool,
}

// === Name pools ===

const FIRST_NAMES: &[&str] = &[
    "Amara", "Yuki", "Dmitri", "Fatima", "Carlos", "Mei", "Olumide",
    "Priya", "Luisa", "Kenji", "Aaliya", "Sven", "Zainab", "Hiroshi",
    "Elena", "Kofi", "Ananya", "Mateo", "Linh", "Ibrahim",
    "Saoirse", "Tariq", "Ingrid", "Chidi", "Esperanza", "Rin",
    "Mikhail", "Adaeze", "Raj", "Freya", "Hassan", "Yara",
    "Tomoko", "Eduardo", "Nkechi", "Arjun", "Sigrid", "Wei",
    "Amina", "Nikolai", "Sakura", "Omar", "Leila", "Bjorn",
    "Chioma", "Ravi", "Astrid", "Jun", "Mariana", "Kwame",
    "Aiko", "Viktor", "Naomi", "Diego", "Suki",
];

const LAST_NAMES: &[&str] = &[
    "Okafor", "Tanaka", "Volkov", "Al-Rashid", "Herrera", "Chen",
    "Adeyemi", "Sharma", "Ferreira", "Nakamura", "Hassan", "Eriksen",
    "Osei", "Sato", "Petrova", "Mensah", "Gupta", "Silva",
    "Nguyen", "Khoury", "O'Brien", "Johansson", "Eze", "Watanabe",
    "Rodriguez", "Kumar", "Lindberg", "Zhang", "Diallo", "Petrov",
    "Yoshida", "Ramirez", "Okonkwo", "Patel", "Andersson", "Li",
    "Abara", "Ivanov", "Kimura", "Reyes", "Nwosu", "Bhat",
    "Holm", "Wang", "Abubakar", "Sokolov", "Mori", "Torres",
    "Achebe", "Krishnan", "Berg", "Huang", "Bello", "Kozlov",
];

const NATIONALITIES: &[&str] = &[
    "Nigerian", "Japanese", "Russian", "Egyptian", "Brazilian",
    "Chinese", "Indian", "Mexican", "Vietnamese", "Lebanese",
    "Irish", "Swedish", "Ghanaian", "Korean", "Ukrainian",
    "Kenyan", "Chilean", "Indonesian", "Norwegian", "Argentinian",
];

const PROFESSIONS: &[&str] = &[
    "Hydrologist", "Nuclear Engineer", "Geneticist", "Surgeon",
    "Teacher", "Farmer", "Architect", "Psychologist", "Botanist",
    "Geologist", "Astrophysicist", "Carpenter", "Electrician",
    "Chef", "Musician", "Poet", "Historian", "Linguist",
    "Philosopher", "Marine Biologist", "Epidemiologist",
    "Civil Engineer", "Data Scientist", "Social Worker", "Midwife",
    "Veterinarian", "Metallurgist", "Atmospheric Chemist",
    "Soil Scientist", "Pediatrician", "Trauma Surgeon",
    "Water Treatment Specialist", "Solar Engineer",
    "Structural Engineer", "Agronomist", "Mycologist",
    "Anthropologist", "Ethicist", "Mediator", "Firefighter",
];

const PERSPECTIVES: &[&str] = &[
    "Democracy failed Earth. Maybe we need something new.",
    "Democracy was the only thing worth saving.",
    "Everything should be shared equally.",
    "People should earn their share.",
    "The children should know the truth about Earth.",
    "Let the children have innocence.",
    "Technology saved us.",
    "Technology destroyed us first.",
    "Anna is a tool, nothing more.",
    "Anna is the best of us.",
    "We should look back. Learn from what happened.",
    "Looking back serves nothing. Only forward matters.",
    "We owe it to the ones left behind to build something better.",
    "We owe nothing. Survival is enough.",
    "Humanity deserved what happened.",
    "Nobody deserved what happened. Not even the ones who caused it.",
];

const SECRETS: &[&str] = &[
    "Was not selected through the official program. Bribed their way aboard.",
    "Carries a genetic marker for a rare condition. Selection committee didn't know.",
    "Left a twin sibling behind. Chose themselves.",
    "Was part of the team that designed the cryo systems. Knows the failure rates.",
    "Smuggled seeds from a now-extinct species.",
    "Has a photographic memory of Earth's last broadcast.",
    "Was originally assigned to a different ark. Switched at the last moment.",
    "Knows the real reason one of the other arks was destroyed.",
    "Wrote the selection algorithm. Knows it was biased.",
    "Is carrying encrypted data from the last government.",
    "Their profession was falsified. They were actually a soldier.",
    "Made a deal: their seat on the ark in exchange for information.",
];

const BACKSTORY_DISASTERS: &[&str] = &[
    "the floods", "the drought", "the collapse", "the fires",
    "the uprising", "the quarantine", "the blackouts", "the famine",
];

const BACKSTORY_ACTIONS: &[&str] = &[
    "watching the last sunset from their rooftop",
    "holding a stranger's hand at the launch site",
    "burning everything they couldn't carry",
    "recording a message for someone who would never hear it",
    "planting a tree that would never grow tall enough",
    "teaching a child to read by candlelight",
    "cooking a meal from the last real ingredients",
    "walking through empty streets, memorizing every detail",
];

const BACKSTORY_OBJECTS: &[&str] = &[
    "a folded photograph", "a seed packet", "a child's drawing",
    "a broken watch", "a handwritten recipe", "a river stone",
    "a music box", "a pressed flower", "a wooden chess piece",
    "a letter never sent",
];

const BACKSTORY_PEOPLE: &[&str] = &[
    "their mother", "their daughter", "a friend they'll never see again",
    "someone whose name they promised to remember",
    "the last patient they treated", "a student who believed in them",
];

const FAMILY_MEMBERS: &[&str] = &[
    "partner", "sister", "brother", "daughter", "son",
    "mother", "father", "best friend", "mentor",
];

/// Generate crew members from a seed. Returns 20-30 unique members.
pub fn generate_crew(seed: u64) -> Vec<CrewMember> {
    let mut rng = StdRng::seed_from_u64(seed.wrapping_add(0xC4E5_5EED));
    let count = rng.gen_range(20..=30);
    let mut crew = Vec::with_capacity(count);
    let mut used_names: Vec<String> = Vec::new();
    let mut used_pods: Vec<u32> = Vec::new();

    for i in 0..count {
        let member = gen_one_member(&mut rng, i, &mut used_names, &mut used_pods);
        crew.push(member);
    }

    // Create connections between some crew members
    let crew_len = crew.len();
    let connection_count = rng.gen_range(3..=6).min(crew_len / 2);
    for _ in 0..connection_count {
        let a = rng.gen_range(0..crew_len);
        let b = rng.gen_range(0..crew_len);
        if a != b && crew[a].connection.is_none() && crew[b].connection.is_none() {
            crew[a].connection = Some(b);
            crew[b].connection = Some(a);
        }
    }

    crew
}

fn gen_one_member(
    rng: &mut StdRng,
    _index: usize,
    used_names: &mut Vec<String>,
    used_pods: &mut Vec<u32>,
) -> CrewMember {
    // Generate unique name
    let name = loop {
        let first = FIRST_NAMES[rng.gen_range(0..FIRST_NAMES.len())];
        let last = LAST_NAMES[rng.gen_range(0..LAST_NAMES.len())];
        let full = format!("{} {}", first, last);
        if !used_names.contains(&full) {
            used_names.push(full.clone());
            break full;
        }
    };

    // Generate unique pod number
    let pod_number = loop {
        let pod = rng.gen_range(1..=14892);
        if !used_pods.contains(&pod) {
            used_pods.push(pod);
            break pod;
        }
    };

    let age = rng.gen_range(22..=65);
    let nationality = NATIONALITIES[rng.gen_range(0..NATIONALITIES.len())].to_string();
    let profession = PROFESSIONS[rng.gen_range(0..PROFESSIONS.len())].to_string();
    let perspective = PERSPECTIVES[rng.gen_range(0..PERSPECTIVES.len())].to_string();

    let backstory = gen_backstory(rng, &name, &profession, &nationality);

    let secret = if rng.gen_bool(0.35) {
        Some(SECRETS[rng.gen_range(0..SECRETS.len())].to_string())
    } else {
        None
    };

    CrewMember {
        name,
        age,
        nationality,
        profession,
        pod_number,
        backstory,
        perspective,
        secret,
        connection: None,
        alive: true,
    }
}

fn gen_backstory(rng: &mut StdRng, name: &str, profession: &str, nationality: &str) -> String {
    let first_name = name.split_whitespace().next().unwrap_or(name);
    let template = rng.gen_range(0..4);
    match template {
        0 => {
            let disaster = BACKSTORY_DISASTERS[rng.gen_range(0..BACKSTORY_DISASTERS.len())];
            format!(
                "{} watched {} take everything they knew. They believe {}",
                first_name, disaster,
                PERSPECTIVES[rng.gen_range(0..PERSPECTIVES.len())],
            )
        }
        1 => {
            let action = BACKSTORY_ACTIONS[rng.gen_range(0..BACKSTORY_ACTIONS.len())];
            format!(
                "Before the ark, {} was a {} from {}. They spent their last day on Earth {}.",
                first_name, profession, nationality, action,
            )
        }
        2 => {
            let family = FAMILY_MEMBERS[rng.gen_range(0..FAMILY_MEMBERS.len())];
            format!(
                "{} volunteered for the ark program. Their {} didn't make the selection.",
                first_name, family,
            )
        }
        _ => {
            let object = BACKSTORY_OBJECTS[rng.gen_range(0..BACKSTORY_OBJECTS.len())];
            let person = BACKSTORY_PEOPLE[rng.gen_range(0..BACKSTORY_PEOPLE.len())];
            format!(
                "{} was one of the last to board. They carried nothing but {} — a gift from {}.",
                first_name, object, person,
            )
        }
    }
}

/// Get a crew member by pod number.
#[allow(dead_code)]
pub fn find_by_pod(crew: &[CrewMember], pod: u32) -> Option<&CrewMember> {
    crew.iter().find(|m| m.pod_number == pod)
}

/// Get crew members connected to a specific member.
#[allow(dead_code)]
pub fn get_connected(crew: &[CrewMember], index: usize) -> Option<&CrewMember> {
    crew.get(index)
        .and_then(|m| m.connection)
        .and_then(|conn_idx| crew.get(conn_idx))
}

/// Format a crew member introduction for Anna to read.
#[allow(dead_code)]
pub fn anna_introduces(member: &CrewMember) -> String {
    format!(
        "I was monitoring Pod {} today. {}, age {}. {}",
        member.pod_number, member.name, member.age, member.backstory,
    )
}

/// Format a crew death report for Anna.
#[allow(dead_code)]
pub fn anna_reports_death(member: &CrewMember) -> String {
    format!(
        "{} is gone. They were a {} from {}. Pod {}.",
        member.name, member.profession, member.nationality, member.pod_number,
    )
}
