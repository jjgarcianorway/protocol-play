// SPDX-License-Identifier: GPL-3.0-or-later

//! Aggregates all dialog scenes from all acts.

use super::dialog_types::DialogScene;
use super::dialog_scenes_act1;
use super::dialog_scenes_act2;
use super::dialog_scenes_act3;
use super::dialog_scenes_act4;
use super::dialog_scenes_crew;
use super::dialog_scenes_crew_ng;
use super::dialog_scenes_philosophy;
use super::dialog_scenes_philosophy2;
use super::dialog_scenes_hidden;
use super::dialog_scenes_earth;
use super::dialog_scenes_earth2;
use super::dialog_scenes_anna_personal;

/// Return all dialog scenes from all acts.
pub fn all_scenes() -> Vec<&'static DialogScene> {
    let mut scenes = Vec::new();
    scenes.extend(dialog_scenes_act1::act1_scenes());
    scenes.extend(dialog_scenes_act2::act2_scenes());
    scenes.extend(dialog_scenes_act3::act3_scenes());
    scenes.extend(dialog_scenes_act4::act4_scenes());
    scenes.extend(dialog_scenes_crew::crew_scenes());
    scenes.extend(dialog_scenes_crew_ng::crew_ng_scenes());
    scenes.extend(dialog_scenes_philosophy::philosophy_scenes_1());
    scenes.extend(dialog_scenes_philosophy2::philosophy_scenes_2());
    scenes.extend(dialog_scenes_hidden::hidden_scenes());
    scenes.extend(dialog_scenes_earth::earth_scenes_1());
    scenes.extend(dialog_scenes_earth2::earth_scenes_2());
    scenes.extend(dialog_scenes_anna_personal::anna_personal_scenes());
    scenes
}
