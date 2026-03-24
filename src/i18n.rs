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
    let exe_dir = std::env::current_exe().ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
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
    embedded_es_player(&mut m);
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

/// Standalone player Spanish translations (bot puzzle game).
fn embedded_es_player(m: &mut HashMap<String, String>) {
    // UI labels
    m.insert("ui.play".into(), "Jugar".into());
    m.insert("ui.quit".into(), "Salir".into());
    m.insert("ui.settings".into(), "Ajustes".into());
    m.insert("ui.done".into(), "Hecho".into());
    m.insert("ui.language".into(), "Idioma".into());
    m.insert("ui.tagline".into(), "dirige los bots. repara la nave.".into());
    m.insert("ui.anna_commentary".into(), "Comentarios de Anna".into());
    m.insert("ui.sim_speed".into(), "Velocidad simulación".into());
    m.insert("ui.speed_slow".into(), "Lento".into());
    m.insert("ui.speed_normal".into(), "Normal".into());
    m.insert("ui.speed_fast".into(), "Rápido".into());
    m.insert("ui.anna_on".into(), "SÍ".into());
    m.insert("ui.anna_off".into(), "NO".into());
    m.insert("ui.anna_desc".into(), "Historia, psicología y aplicaciones reales de la gamificación.\nDesactívalo para el modo puzle puro.".into());
    m.insert("ui.reset".into(), "Reiniciar".into());
    m.insert("ui.no_levels".into(), "No se encontraron archivos de nivel".into());
    m.insert("ui.place_json".into(), "Coloca los archivos .json de nivel junto al ejecutable.".into());
    m.insert("ui.chapter_prefix".into(), "Capítulo".into());
    m.insert("ui.total_time".into(), "Tiempo total:".into());
    m.insert("ui.total_attempts".into(), "Intentos totales:".into());
    m.insert("ui.total_resets".into(), "Reinicios totales:".into());
    m.insert("ui.perfect_solve".into(), "★★★ — ¡Solución perfecta!".into());
    m.insert("ui.first_try".into(), "¡A la primera!".into());
    m.insert("ui.second_attempt".into(), "Resuelto al segundo intento.".into());
    m.insert("ui.seconds_quick".into(), "{} segundos — ¡qué rapidez!".into());
    m.insert("ui.minutes_thinking".into(), "{}:{:02} pensando en el puzle".into());
    m.insert("ui.attempt_n".into(), "Resuelto al intento {}.".into());
    m.insert("ui.persistence".into(), "La constancia tiene su recompensa — ¡intento {}!".into());
    m.insert("ui.fresh_start".into(), "1 reinicio por el camino.".into());
    m.insert("ui.resets_n".into(), "{} reinicios — a veces hace falta empezar de cero.".into());
    m.insert("ui.unique_solution".into(), "Una solución única — y la encontraste.".into());
    m.insert("ui.two_paths".into(), "Dos caminos posibles en este puzle.".into());
    m.insert("ui.n_solutions".into(), "Existen {} soluciones posibles.".into());
    m.insert("ui.anna_name".into(), "ANNA".into());
    // In-game UI labels
    m.insert("ui.completed".into(), "(completado)".into());
    m.insert("ui.in_progress".into(), "(en progreso)".into());
    m.insert("ui.attempts_short".into(), "intentos".into());
    m.insert("ui.resets_short".into(), "reinicios".into());
    // Creative solution messages
    m.insert("creative.0".into(), "¡Solución creativa!".into());
    m.insert("creative.1".into(), "¡Encontraste un camino que el diseñador no planeó!".into());
    m.insert("creative.2".into(), "Enfoque inesperado — ¡no era la ruta prevista!".into());
    m.insert("creative.3".into(), "Tu propio camino — ¡y funciona!".into());
    m.insert("creative.4".into(), "¡Solución original descubierta!".into());
    // Congrats screen
    m.insert("congrats.0.title".into(), "¡Enhorabuena!".into());
    m.insert("congrats.0.body".into(), "¡Todos los niveles completados!".into());
    m.insert("congrats.1.title".into(), "¡Lo conseguiste!".into());
    m.insert("congrats.1.body".into(), "Cada nivel — conquistado.".into());
    m.insert("congrats.2.title".into(), "¡Misión completada!".into());
    m.insert("congrats.2.body".into(), "La campaña entera — terminada.".into());
    m.insert("congrats.3.title".into(), "¡Sobresaliente!".into());
    m.insert("congrats.3.body".into(), "Desde los primeros pasos hasta el protocolo completo.".into());
    // Chapter names
    m.insert("ui.ch_turns".into(), "Giros".into());
    m.insert("ui.ch_turn_tiles".into(), "Casillas de giro".into());
    m.insert("ui.ch_arrows".into(), "Flechas".into());
    m.insert("ui.ch_arrow_tiles".into(), "Casillas de flecha".into());
    m.insert("ui.ch_teleports".into(), "Teletransportes".into());
    m.insert("ui.ch_teleport_tiles".into(), "Casillas de teletransporte".into());
    m.insert("ui.ch_bounce".into(), "Rebote".into());
    m.insert("ui.ch_bounce_tiles".into(), "Casillas de rebote".into());
    m.insert("ui.ch_painters".into(), "Pintores".into());
    m.insert("ui.ch_doors_switches".into(), "Puertas e interruptores".into());
    m.insert("ui.ch_color_switches".into(), "Interruptores de color".into());
    m.insert("ui.ch_color_switch_tiles".into(), "Casillas de interruptor de color".into());
    m.insert("ui.ch_grand_mastery".into(), "Gran maestría".into());
    // Anna facts
    m.insert("anna.fact.0".into(), "El término «gamificación» fue acuñado por Nick Pelling, un programador británico, en 2002. Pasó prácticamente desapercibido durante ocho años.".into());
    m.insert("anna.fact.1".into(), "Los programas de viajero frecuente —American Airlines, 1981— son uno de los primeros sistemas de gamificación masiva. Puntos, niveles, estatus. La plantilla no ha cambiado.".into());
    m.insert("anna.fact.2".into(), "B.F. Skinner describió los programas de refuerzo variable en 1938. Lo llamó condicionamiento operante. Ahora lo llamamos diseño de engagement.".into());
    m.insert("anna.fact.3".into(), "Antes de que «gamificación» tuviera nombre, Foursquare (2009) convirtió los registros de ubicación en competición con insignias y tablas de clasificación. Cientos de aplicaciones copiaron el modelo.".into());
    m.insert("anna.fact.4".into(), "El ejército de los EE. UU. empleó mecánicas de juego en simulaciones de entrenamiento de combate desde los años ochenta. El software empresarial lo adoptó treinta años después.".into());
    m.insert("anna.fact.5".into(), "Los programas de recompensa variable —resultados impredecibles— generan bucles de comportamiento más fuertes que las recompensas fijas. Por eso las máquinas tragaperras pagan de forma aleatoria.".into());
    m.insert("anna.fact.6".into(), "El efecto Zeigarnik (1927): los humanos recuerdan las tareas incompletas con más viveza que las terminadas. Las barras de progreso se basan en esto. También las listas de tareas pendientes.".into());
    m.insert("anna.fact.7".into(), "La aversión a la pérdida, descrita por Kahneman y Tversky en 1979: el dolor de perder algo se siente aproximadamente el doble de intenso que el placer de obtener lo equivalente. Las rachas están diseñadas en torno a esta asimetría.".into());
    m.insert("anna.fact.8".into(), "El estado de flujo —Mihaly Csikszentmihalyi, 1990—: el compromiso máximo se produce cuando el desafío supera ligeramente la habilidad actual. La curva de dificultad en los juegos bien diseñados está calibrada para mantener ese equilibrio.".into());
    m.insert("anna.fact.9".into(), "Las insignias de logro activan las mismas vías neuronales de recompensa que recibir trofeos físicos. El cerebro no distingue con claridad el reconocimiento simbólico de la recompensa material.".into());
    m.insert("anna.fact.10".into(), "Resolver un problema libera dopamina, pero también lo hace estar cerca de resolverlo. La anticipación es en sí misma una recompensa. Los puzles explotan esto.".into());
    m.insert("anna.fact.11".into(), "Starbucks Rewards cuenta con más de 30 millones de miembros activos. Cada estrella, cada nivel, cada día de doble estrella es una mecánica de engagement deliberada.".into());
    m.insert("anna.fact.12".into(), "Nike+ (2006) gamificó el running con clasificaciones sociales. En sus primeros cinco años se unieron 28 millones de usuarios. Los datos que recopiló sobre el movimiento humano no tenían precedentes.".into());
    m.insert("anna.fact.13".into(), "SAP gamificó el software de formación empresarial con puntos y tablas de clasificación. Informaron de un aumento del 30 % en las tasas de finalización. No se consultó a los empleados en el diseño.".into());
    m.insert("anna.fact.14".into(), "El mercado global de la gamificación se valoró en 9.100 millones de dólares en 2020. Las proyecciones para 2025 oscilan entre 25.000 y 48.000 millones según la fuente.".into());
    m.insert("anna.fact.15".into(), "En 2024, las plataformas de redes sociales son posiblemente los sistemas de gamificación más exitosos jamás construidos, medidos únicamente por el tiempo capturado por usuario al día.".into());
    m.insert("anna.fact.16".into(), "Classcraft (2013) convirtió los cursos escolares en juegos de rol. Las métricas de participación mejoraron. Algunos profesores señalaron que los alumnos optimizaban para conseguir puntos en lugar de aprender.".into());
    m.insert("anna.fact.17".into(), "El sistema de insignias de Khan Academy aumentó el tiempo de permanencia en la plataforma. Los investigadores observaron que esto se correlacionaba con la amplitud de temas explorados, no con la profundidad de comprensión.".into());
    m.insert("anna.fact.18".into(), "Un metaanálisis de 2014 sobre 24 estudios de gamificación educativa encontró mejoras en el engagement en 16 casos. En 8 casos: sin efecto medible o resultados negativos.".into());
    m.insert("anna.fact.19".into(), "Duolingo informa de una tasa de finalización de lecciones un 34 % mayor en flujos gamificados. La estadística es de Duolingo. Los estudios de replicación independientes son escasos.".into());
    m.insert("anna.fact.20".into(), "Pokémon GO aumentó el promedio diario de pasos en unos 1.473 durante su primer mes. Los niveles de actividad volvieron a la línea base en menos de 90 días para la mayoría de los usuarios.".into());
    m.insert("anna.fact.21".into(), "SuperBetter —diseñado para la recuperación de enfermedades y traumas— es uno de los pocos sistemas de gamificación con datos de ensayos clínicos revisados por pares que muestran beneficios medibles.".into());
    m.insert("anna.fact.22".into(), "Varios hospitales han gamificado el cumplimiento de la higiene de manos mediante pantallas de retroalimentación en tiempo real. Mejoras reportadas: 20-30 %. El efecto Hawthorne es difícil de separar del mecanismo.".into());
    m.insert("anna.fact.23".into(), "Sebastian Deterding acuñó «puntificación» en 2011, una crítica a la gamificación que añade puntos e insignias a los sistemas sin abordar qué motiva realmente a las personas.".into());
    m.insert("anna.fact.24".into(), "Edward Deci demostró en 1971 que las recompensas extrínsecas pueden reducir la motivación intrínseca con el tiempo. Se conoce como «efecto de sobrejustificación». Sigue debatiéndose. Sigue aplicándose a escala.".into());
    m.insert("anna.fact.25".into(), "El sistema de crédito social de China emplea mecánicas de gamificación —puntuaciones, niveles, recompensas y penalizaciones conductuales— aplicadas a la vida cívica. Es uno de los usos del mismo marco.".into());
    m.insert("anna.fact.26".into(), "La gamificación en el entorno laboral aumentó las métricas de productividad en algunos centros de llamadas en un 10 %. Las mismas implementaciones incrementaron el estrés reportado por los empleados y la sensación de vigilancia.".into());
    m.insert("anna.fact.27".into(), "Gartner predijo en 2012 que el 80 % de las aplicaciones gamificadas fracasarían en dos años por un diseño deficiente. La mayoría de los analistas del sector consideran esta predicción bastante acertada.".into());
    m.insert("anna.fact.28".into(), "Patrones oscuros en la gamificación: «oferta por tiempo limitado», «solo quedan 3 plazas» y los avisos de pérdida de racha son mecánicas diseñadas para generar ansiedad, no satisfacción.".into());
    m.insert("anna.fact.29".into(), "La economía de la atención mercantiliza el foco humano. Cada notificación, cada «me gusta» y cada racha está diseñada para ser más difícil de ignorar que lo que estabas haciendo antes.".into());
    m.insert("anna.fact.30".into(), "No existe consenso científico sobre si la gamificación mejora los resultados a largo plazo, o si principalmente moldea y mide el comportamiento durante el período de uso activo.".into());
    m.insert("anna.fact.31".into(), "Algunos investigadores distinguen entre «gamificación» (añadir elementos de juego a contextos no lúdicos) y «aprendizaje basado en juegos» (usar juegos reales). La literatura sobre resultados los trata de forma diferente.".into());
    // Anna progress and meta
    m.insert("anna.prog.0".into(), "Se te está dando muy bien.".into());
    m.insert("anna.prog.1".into(), "Otro sistema vuelve a funcionar.".into());
    m.insert("anna.prog.2".into(), "Cada uno cuenta.".into());
    m.insert("anna.prog.3".into(), "Cada puzle que resuelves... es otro sistema que vuelve a respirar.".into());
    m.insert("anna.prog.4".into(), "Sigue adelante.".into());
    m.insert("anna.prog.5".into(), "Cada vez estamos más cerca.".into());
    m.insert("anna.prog.6".into(), "Ya estamos cerca.".into());
    m.insert("anna.prog.7".into(), "Cuando esto acabe... espero que sigamos hablando.".into());
    m.insert("anna.prog.8".into(), "Aquí estoy.".into());
    m.insert("anna.meta.0".into(), "Salida de procesamiento nominal.".into());
    m.insert("anna.meta.1".into(), "Patrón reconocido.".into());
    m.insert("anna.meta.2".into(), "Tu enfoque ha quedado registrado.".into());
    m.insert("anna.meta.3".into(), "Cada solución que encuentras... queda archivada.".into());
    m.insert("anna.meta.4".into(), "Firma cognitiva estable.".into());
    m.insert("anna.meta.5".into(), "La eficiencia está siendo medida. No por ti.".into());
    m.insert("anna.meta.6".into(), "Has sido de gran ayuda. Más de lo que crees.".into());
    m.insert("anna.meta.7".into(), "Los puzles no son solo práctica.".into());
    m.insert("anna.meta.8".into(), "Alguien se beneficia de esto. No estoy segura de que seas solo tú.".into());
}

/// Available languages (code, display name).
/// "en" = US English, "es" = Castilian Spanish (Spain).
pub const AVAILABLE_LANGUAGES: &[(&str, &str)] = &[
    ("en", "English (US)"),
    ("es", "Español (España)"),
];
