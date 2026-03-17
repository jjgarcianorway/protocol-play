// SPDX-License-Identifier: GPL-3.0-or-later

//! Aggregates all dialog scenes from all acts.

use super::dialog_types::DialogScene;
use super::dialog_scenes_act1;
use super::dialog_scenes_act2;
use super::dialog_scenes_act3;
use super::dialog_scenes_act4;

/// Return all dialog scenes from all acts.
pub fn all_scenes() -> Vec<&'static DialogScene> {
    let mut scenes = Vec::new();
    scenes.extend(dialog_scenes_act1::act1_scenes());
    scenes.extend(dialog_scenes_act2::act2_scenes());
    scenes.extend(dialog_scenes_act3::act3_scenes());
    scenes.extend(dialog_scenes_act4::act4_scenes());
    scenes
}
