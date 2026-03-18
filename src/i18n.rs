// SPDX-License-Identifier: GPL-3.0-or-later

//! Internationalization (i18n) system.
//! Translation keys are auto-derived from `scene_id.node_index` for dialog
//! and `ui.key_name` for UI labels.
//! Falls back to English (the hardcoded `&'static str`) if no translation found.

use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;

/// Resource holding all translations for the active language.
#[derive(Resource, Debug)]
pub struct Translations {
    /// Current language code ("en", "es", etc.)
    pub language: String,
    /// Map of translation keys to translated strings.
    /// Key format: "scene_id.node_index" for dialog, "ui.label_name" for UI.
    translations: HashMap<String, String>,
}

impl Default for Translations {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            translations: HashMap::new(),
        }
    }
}

impl Translations {
    /// Look up a dialog node's translated text.
    /// Key = `"{scene_id}.{node_index}"`.
    /// Returns the translation, or `None` if not found (caller uses English
    /// fallback).
    pub fn dialog(&self, scene_id: &str, node_index: usize) -> Option<&str> {
        if self.language == "en" { return None; }
        let key = format!("{}.{}", scene_id, node_index);
        self.translations.get(&key).map(|s| s.as_str())
    }

    /// Look up a dialog choice's translated text.
    /// Key = `"{scene_id}.{node_index}.choice.{choice_index}"`.
    pub fn choice(
        &self, scene_id: &str, node_index: usize, choice_index: usize,
    ) -> Option<&str> {
        if self.language == "en" { return None; }
        let key = format!("{}.{}.choice.{}", scene_id, node_index, choice_index);
        self.translations.get(&key).map(|s| s.as_str())
    }

    /// Look up a UI label translation.
    /// Key = `"ui.{label_name}"`.
    pub fn ui(&self, label: &str) -> Option<&str> {
        if self.language == "en" { return None; }
        let key = format!("ui.{}", label);
        self.translations.get(&key).map(|s| s.as_str())
    }

    /// Translate a UI label with English fallback.
    pub fn ui_or<'a>(&'a self, label: &str, fallback: &'a str) -> &'a str {
        self.ui(label).unwrap_or(fallback)
    }
}

/// Load translations for the given language code.
/// Looks for `assets/i18n/{lang}.json` next to the executable,
/// then falls back to embedded translations.
pub fn load_translations(lang: &str) -> Translations {
    if lang == "en" {
        return Translations::default();
    }

    // Try loading from file
    let exe_dir = crate::save_state::exe_dir();
    let path = exe_dir.join(format!("assets/i18n/{}.json", lang));

    let translations = if let Ok(contents) = fs::read_to_string(&path) {
        parse_translation_json(&contents)
    } else {
        // Try embedded translations
        load_embedded(lang)
    };

    Translations {
        language: lang.to_string(),
        translations,
    }
}

/// Parse a flat JSON object `{ "key": "value", ... }` into a HashMap.
fn parse_translation_json(json: &str) -> HashMap<String, String> {
    serde_json::from_str(json).unwrap_or_default()
}

/// Embedded translations compiled into the binary.
/// For now, only Spanish is embedded as a starter set.
fn load_embedded(lang: &str) -> HashMap<String, String> {
    match lang {
        "es" => embedded_es(),
        _ => HashMap::new(),
    }
}

/// Spanish (Spain) UI translations (embedded baseline).
/// Uses Castilian Spanish (vosotros, tú), NOT Latin American Spanish.
fn embedded_es() -> HashMap<String, String> {
    let mut m = HashMap::new();
    // Mission Control UI
    m.insert("ui.power".into(), "Energía".into());
    m.insert("ui.life_support".into(), "Soporte Vital".into());
    m.insert("ui.cryo".into(), "Criogenia".into());
    m.insert("ui.shields".into(), "Escudos".into());
    m.insert("ui.repair".into(), "Reparación".into());
    m.insert("ui.crew".into(), "Tripulación".into());
    m.insert("ui.day".into(), "Día".into());
    m.insert("ui.distance".into(), "Distancia".into());
    m.insert("ui.crystals".into(), "Cristales".into());
    m.insert("ui.final_voyage".into(), "Viaje Final".into());
    m.insert("ui.click_continue".into(), "Clic para continuar".into());
    m.insert("ui.click_skip".into(), "Clic para saltar".into());
    m.insert("ui.the_repairing".into(), "La Reparación".into());
    m.insert("ui.the_gathering".into(), "La Recolección".into());
    m.insert("ui.the_converter".into(), "El Conversor".into());
    m.insert("ui.the_delivery".into(), "La Entrega".into());
    m.insert("ui.orben".into(), "Orben".into());
    m.insert("ui.crew_manifest".into(), "Registro de Tripulación".into());
    m.insert("ui.new_journey".into(), "Nuevo Viaje".into());
    m.insert("ui.same_world".into(), "Mismo Mundo".into());
    m.insert("ui.new_world".into(), "Nuevo Mundo".into());
    m.insert("ui.language".into(), "Idioma".into());
    m.insert("ui.anna".into(), "Anna".into());
    m.insert("ui.narrator".into(), "Narrador".into());
    m.insert("ui.you".into(), "Tú".into());
    m.insert("ui.system".into(), "[SISTEMA]".into());
    m
}

/// Available languages (code, display name).
/// Available languages (code, display name).
/// "en" = US English, "es" = Castilian Spanish (Spain).
pub const AVAILABLE_LANGUAGES: &[(&str, &str)] = &[
    ("en", "English (US)"),
    ("es", "Español (España)"),
];
