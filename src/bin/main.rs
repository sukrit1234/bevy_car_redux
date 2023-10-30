// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::*, window::WindowResolution};
use bevy_racing_redux::car_app;
use bevy_racing_redux::renet::NetworkMode;

fn main() {
        let mut app = App::new();
        #[cfg(not(target_arch = "wasm32"))]
        let res = WindowResolution::default();
        #[cfg(target_arch = "wasm32")]
        let res = WindowResolution::new(720., 360.);
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Racing Redux".to_string(),
                    resolution: res,
                    canvas: Some("#bevy-racing-redux".to_string()),
                    ..default()
                }),
                ..default()
            }),
        ));
        car_app(&mut app,NetworkMode::Standalone).run();
}
