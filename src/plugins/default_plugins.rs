use bevy::{app::PluginGroupBuilder, prelude::*, window::WindowResolution};

use crate::settings::settings::{WINDOW_RESOLUTION, WINDOW_SIZE_FACTOR, WINDOW_TITLE};

pub fn get_defaults_plugins() -> PluginGroupBuilder {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            mode: bevy::window::WindowMode::Windowed,
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resolution: WindowResolution::new(
                WINDOW_SIZE_FACTOR * WINDOW_RESOLUTION.0,
                WINDOW_SIZE_FACTOR * WINDOW_RESOLUTION.1,
            ),
            title: WINDOW_TITLE.to_string(),
            resizable: false,
            ..default()
        }),
        ..default()
    };

    DefaultPlugins.set(window_plugin)
}
