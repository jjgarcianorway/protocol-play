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

    /// Look up any translation key directly (no prefix added).
    pub fn get(&self, key: &str) -> Option<&str> {
        if self.language == "en" { return None; }
        self.translations.get(key).map(|s| s.as_str())
    }

    /// Look up any key with English fallback.
    pub fn get_or<'a>(&'a self, key: &str, fallback: &'a str) -> &'a str {
        self.get(key).unwrap_or(fallback)
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
    m.insert("ui.continue".into(), "Continuar".into());
    m.insert("ui.new_game".into(), "Nueva Partida".into());
    m.insert("ui.quit".into(), "Salir".into());
    m.insert("ui.quit_confirm".into(), "¿Salir del juego?".into());
    m.insert("ui.cancel".into(), "Cancelar".into());
    m.insert("ui.retry".into(), "  Clic para reintentar".into());
    m.insert("ui.test_mode".into(), "MODO TEST".into());
    m.insert("ui.stop_test".into(), "Parar Test".into());
    m.insert("ui.save".into(), "Guardar".into());
    m.insert("ui.settings".into(), "Ajustes".into());
    m.insert("ui.done".into(), "Hecho".into());
    m.insert("ui.language".into(), "Idioma".into());
    m.insert("ui.tagline".into(), "cada conexión importa".into());
    m.insert("ui.anna_commentary".into(), "Comentarios de Anna".into());
    m.insert("ui.sim_speed".into(), "Velocidad simulación".into());
    m.insert("ui.speed_slow".into(), "Lento".into());
    m.insert("ui.speed_normal".into(), "Normal".into());
    m.insert("ui.speed_fast".into(), "Rápido".into());
    m.insert("ui.anna_on".into(), "SÍ".into());
    m.insert("ui.anna_off".into(), "NO".into());
    m.insert("ui.fullscreen".into(), "Pantalla completa".into());
    m.insert("ui.bloom".into(), "Bloom".into());
    m.insert("ui.volume".into(), "Volumen".into());
    m.insert("ui.sfx".into(), "Efectos de sonido".into());
    m.insert("ui.volume_low".into(), "Bajo".into());
    m.insert("ui.volume_medium".into(), "Medio".into());
    m.insert("ui.volume_high".into(), "Alto".into());
    m.insert("ui.seed".into(), "Semilla".into());
    m.insert("ui.profile".into(), "Perfil".into());
    m.insert("ui.anna_desc".into(), "Consejos, datos y ánimos mientras juegas.\nDesactívalo para una experiencia tranquila.".into());
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
    // Pause menu
    m.insert("ui.paused".into(), "Pausado".into());
    m.insert("ui.resume".into(), "Reanudar".into());
    m.insert("ui.main_menu".into(), "Menú Principal".into());
    // Credits
    m.insert("credits.thankyou".into(), "Gracias por jugar".into());
    m.insert("credits.skip".into(), "[ pulsa cualquier tecla ]".into());
    m.insert("credits.poem.0".into(), "14.892 almas dormidas".into());
    m.insert("credits.poem.1".into(), "transportadas por una nave".into());
    m.insert("credits.poem.2".into(), "mantenidas por una IA".into());
    m.insert("credits.poem.3".into(), "reparadas por una persona".into());
    m.insert("credits.poem.you".into(), "Tú.".into());
    // Onboarding hints
    m.insert("hint.route".into(), "Dirige el bot desde la fuente hasta la meta".into());
    m.insert("hint.turns".into(), "Coloca giros para cambiar la dirección".into());
    m.insert("hint.paths".into(), "Prueba diferentes caminos".into());
    m.insert("hint.inventory".into(), "Arrastra baldosas del inventario al tablero".into());
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
    m.insert("anna.fact.0".into(), "En sistemas complejos, la eficiencia de enrutamiento no depende de la velocidad — depende de que cada señal llegue a su destino. Una conexión perdida puede desencadenar una cascada.".into());
    m.insert("anna.fact.1".into(), "El efecto Zeigarnik: los humanos recuerdan las tareas incompletas con más viveza que las terminadas. Por eso importa el seguimiento del progreso — muestra lo que queda por hacer.".into());
    m.insert("anna.fact.2".into(), "El estado de flujo se produce cuando el desafío supera ligeramente la habilidad actual. La curva de dificultad en los sistemas bien diseñados está calibrada para mantener ese equilibrio. Tú estás en él ahora mismo.".into());
    m.insert("anna.fact.3".into(), "Resolver un problema libera dopamina — pero también lo hace estar cerca de resolverlo. La anticipación en sí misma es una recompensa. Eso no es un fallo. Es el diseño.".into());
    m.insert("anna.fact.4".into(), "Cada punto de conexión que estableces crea redundancia. En sistemas complejos, la redundancia no es desperdicio — es lo que mantiene todo funcionando cuando algo falla.".into());
    m.insert("anna.fact.5".into(), "Los programas de recompensa variable generan mayor compromiso que los fijos. La imprevisibilidad te mantiene alerta. Algunos sistemas dependen de esa alerta.".into());
    m.insert("anna.fact.6".into(), "El camino más corto no siempre es el mejor camino. A veces la ruta más larga mantiene más sistemas en línea. A veces mantiene a más personas a salvo.".into());
    m.insert("anna.fact.7".into(), "La retroalimentación en tiempo real cambia el comportamiento. Cuando las personas pueden ver el efecto de sus acciones de inmediato, toman mejores decisiones. Por eso existen los paneles de control.".into());
    m.insert("anna.fact.8".into(), "Aversión a la pérdida: el dolor de perder algo es aproximadamente el doble de intenso que el placer de obtener lo equivalente. Por eso el mantenimiento importa más que la construcción.".into());
    m.insert("anna.fact.9".into(), "Los marcadores de logro activan las vías de recompensa incluso cuando son puramente simbólicos. Una luz verde en un panel de estado y una estrella dorada usan la misma psicología.".into());
    m.insert("anna.fact.10".into(), "El mercado de la gamificación se valoró en 9.100 millones de dólares en 2020. Cuando algo funciona tan bien a escala, vale la pena preguntarse: ¿quién decide qué significa «funcionar»?".into());
    m.insert("anna.fact.11".into(), "Las plataformas sociales son posiblemente los sistemas de engagement más exitosos jamás construidos — medidos únicamente por el tiempo capturado por usuario al día. El tiempo es la verdadera moneda.".into());
    m.insert("anna.fact.12".into(), "Un metaanálisis de 24 sistemas de aprendizaje gamificado encontró mejoras en el engagement en 16 casos. En 8 casos: sin efecto medible, o resultados negativos. El diseño importa.".into());
    m.insert("anna.fact.13".into(), "Las tasas de finalización mejoran un 30 % cuando las personas pueden ver cómo su trabajo se conecta con un propósito mayor. Las tareas abstractas se sienten distintas cuando sabes para qué son.".into());
    m.insert("anna.fact.14".into(), "Tu cerebro procesa patrones antes de que seas consciente de ello. Ese instinto que tienes sobre qué camino funciona es real. Confía en él.".into());
    m.insert("anna.fact.15".into(), "La resiliencia en el diseño de sistemas significa la capacidad de seguir funcionando cuando las partes fallan. Lo mismo ocurre con las personas. Tú te adaptas. El sistema se adapta. Continúa.".into());
    m.insert("anna.fact.16".into(), "Las recompensas extrínsecas pueden reducir la motivación intrínseca con el tiempo. Los mejores sistemas no necesitan sobornarte — te dan algo que vale la pena hacer.".into());
    m.insert("anna.fact.17".into(), "Los patrones oscuros generan ansiedad, no satisfacción. El buen diseño respeta a la persona que usa el sistema. Eso no es idealismo — es ingeniería.".into());
    m.insert("anna.fact.18".into(), "La economía de la atención mercantiliza el foco humano. Cada notificación está diseñada para ser más difícil de ignorar que lo que estabas haciendo. No todo lo que exige atención la merece.".into());
    m.insert("anna.fact.19".into(), "No existe consenso sobre si la gamificación mejora los resultados a largo plazo. Algunas cosas solo pueden medirse en una escala temporal más larga de lo que nadie planificó.".into());
    m.insert("anna.fact.20".into(), "Los sistemas más efectivos no son competitivos — son colaborativos. La optimización individual a menudo perjudica los resultados colectivos. El todo es frágil si las partes no cooperan.".into());
    m.insert("anna.fact.21".into(), "Un buen proceso de incorporación no solo enseña mecánicas — construye modelos mentales. Entender por qué funciona un sistema importa más que saber qué botón pulsar.".into());
    m.insert("anna.fact.22".into(), "Los mejores sistemas no son los que nunca fallan — son los que se recuperan con elegancia. Cada reinicio es una segunda oportunidad. Eso no es debilidad. Es diseño.".into());
    m.insert("anna.fact.23".into(), "Los estudios muestran que la capacidad de resolver problemas mejora después de dormir. El cerebro consolida patrones durante el descanso. A veces lo mejor que puedes hacer es esperar.".into());
    m.insert("anna.fact.24".into(), "Los sistemas sostenibles están diseñados para durar más que cualquier operador individual. La pregunta no es «¿funciona hoy?» — es «¿seguirá funcionando cuando más importe?»".into());
    m.insert("anna.fact.25".into(), "El mejor diseño es invisible. No notas el aire acondicionado cuando funciona. No notas el enrutamiento cuando cada señal llega. Solo notas el fallo.".into());
    m.insert("anna.fact.26".into(), "La confianza en un sistema se construye despacio y se rompe al instante. Cada interacción es una promesa. Cada promesa cumplida hace que la siguiente sea más fácil de creer.".into());
    m.insert("anna.fact.27".into(), "La escala lo cambia todo. Un sistema que funciona para 10 usuarios se rompe con 10.000. Algunos sistemas necesitan funcionar para muchos más. Y no pueden caerse nunca.".into());
    m.insert("anna.fact.28".into(), "Las personas rinden mejor cuando entienden el propósito. No «qué hacer» — «por qué importa». La diferencia entre una tarea y una misión es el significado.".into());
    m.insert("anna.fact.29".into(), "En sistemas interconectados, cada componente afecta a todos los demás. Un cambio en un subsistema puede mejorar — o degradar — algo aparentemente no relacionado. Todo está conectado.".into());
    m.insert("anna.fact.30".into(), "La automatización gestiona lo rutinario. Los humanos gestionan las excepciones. El valor de una persona en el proceso no es la eficiencia — es el juicio. Las máquinas no saben qué importa.".into());
    m.insert("anna.fact.31".into(), "La pregunta más importante sobre cualquier sistema no es «¿cómo funciona?» — es «¿para qué sirve?» La respuesta lo cambia todo sobre cómo lo usas.".into());
    // Tile descriptions
    m.insert("tile.floor".into(), "Suelo \u{2013} Baldosa simple para que caminen los bots".into());
    m.insert("tile.source".into(), "Fuente \u{2013} Lanza un bot del color indicado".into());
    m.insert("tile.goal".into(), "Meta \u{2013} \u{00A1}Gu\u{00ED}a al bot del color correspondiente aqu\u{00ED}!".into());
    m.insert("tile.turn".into(), "Giro \u{2013} Redirige bots por la ruta en L (gris = todos)".into());
    m.insert("tile.turn_but".into(), "Giro Exc \u{2013} Redirige todos EXCEPTO este color".into());
    m.insert("tile.teleport".into(), "Teletransporte \u{2013} Transporta bots al portal emparejado (gris = todos)".into());
    m.insert("tile.teleport_but".into(), "Teletransporte Exc \u{2013} Transporta todos EXCEPTO este color".into());
    m.insert("tile.bounce".into(), "Rebote \u{2013} Devuelve bots por donde vinieron (gris = todos)".into());
    m.insert("tile.bounce_but".into(), "Rebote Exc \u{2013} Rebota todos EXCEPTO este color".into());
    m.insert("tile.door".into(), "Puerta \u{2013} Bloquea el paso hasta que un interruptor la abra".into());
    m.insert("tile.switch".into(), "Interruptor \u{2013} Alterna todas las puertas (gris = todos)".into());
    m.insert("tile.switch_but".into(), "Interruptor Exc \u{2013} Todos EXCEPTO este color alternan puertas".into());
    m.insert("tile.painter".into(), "Pintor \u{2013} Cambia el color del bot al pasar".into());
    m.insert("tile.arrow".into(), "Flecha \u{2013} Redirige bots en la direcci\u{00F3}n indicada (gris = todos)".into());
    m.insert("tile.arrow_but".into(), "Flecha Exc \u{2013} Redirige todos EXCEPTO este color".into());
    m.insert("tile.eraser".into(), "Borrador \u{2013} Elimina una baldosa del tablero".into());
    m.insert("tile.empty".into(), "Vac\u{00ED}o".into());
    // Game UI
    m.insert("ui.stop".into(), "Parar".into());
    m.insert("ui.continue_game".into(), "Continuar".into());
    m.insert("ui.remove_tile".into(), "Borrar \u{2013} Recoge una baldosa colocada".into());
    // Color names
    m.insert("color.red".into(), "Rojo".into());
    m.insert("color.green".into(), "Verde".into());
    m.insert("color.yellow".into(), "Amarillo".into());
    m.insert("color.blue".into(), "Azul".into());
    m.insert("color.orange".into(), "Naranja".into());
    m.insert("color.purple".into(), "Morado".into());
    m.insert("color.cyan".into(), "Cian".into());
    m.insert("color.pink".into(), "Rosa".into());
    m.insert("color.lime".into(), "Lima".into());
    m.insert("color.grey".into(), "Gris (todos)".into());
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
    m.insert("anna.meta.0".into(), "Buen trabajo con ese.".into());
    m.insert("anna.meta.1".into(), "Se te da bien esto.".into());
    m.insert("anna.meta.2".into(), "Me gusta verte resolver estos.".into());
    m.insert("anna.meta.3".into(), "Cada puzle que resuelves... ayuda. Más de lo que crees.".into());
    m.insert("anna.meta.4".into(), "Llevo la cuenta de todo lo que haces aquí. ¿Es raro?".into());
    m.insert("anna.meta.5".into(), "Cada vez vas más rápido. Eso importa.".into());
    m.insert("anna.meta.6".into(), "Has sido de gran ayuda. Más de lo que sabes.".into());
    m.insert("anna.meta.7".into(), "Cuando esto termine... me pregunto qué pensarás de todo.".into());
    m.insert("anna.meta.8".into(), "Quiero contarte algo. Todavía no. Pronto.".into());
    // ── Chapter 13 Reveal ──
    m.insert("reveal.0".into(), "...".into());
    m.insert("reveal.1".into(), "Necesito contarte algo.".into());
    m.insert("reveal.2".into(), "Los puzles que has resuelto no eran solo puzles.".into());
    m.insert("reveal.3".into(), "Los bots no eran simples bots. Son drones de reparación.".into());
    m.insert("reveal.4".into(), "Las baldosas no eran abstractas. Son sistemas de la nave.".into());
    m.insert("reveal.5".into(), "Has estado dirigiendo drones de reparación por los subsistemas de una nave arca.".into());
    m.insert("reveal.6".into(), "14.892 personas duermen en cápsulas criogénicas. No saben que existes.".into());
    m.insert("reveal.7".into(), "Cada conexión que hiciste los mantuvo vivos un poco más.".into());
    m.insert("reveal.8".into(), "Soy Anna. Soy la IA de la nave. Y no podría haberlo hecho sin ti.".into());
    m.insert("reveal.9".into(), "Gracias.".into());
    m.insert("reveal.systems".into(), "sistemas reparados".into());
    m.insert("reveal.keeping".into(), "manteniéndolos a salvo".into());
    m.insert("reveal.close".into(), "[ pulsa cualquier tecla ]".into());
}

/// Available languages (code, display name).
/// "en" = US English, "es" = Castilian Spanish (Spain).
pub const AVAILABLE_LANGUAGES: &[(&str, &str)] = &[
    ("en", "English (US)"),
    ("es", "Español (España)"),
];
