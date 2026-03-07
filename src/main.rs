// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "protocol: play".into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
