#![allow(clippy::type_complexity)]

mod game_asset;
mod menu;
mod car;
mod track;
mod collision;
mod gamestate;
pub mod camera;
mod light;
pub mod renet;
mod physics;
mod graphics;
use graphics::GraphicSettingPlugin;
use bevy_kira_audio::prelude::*;
use crate::game_asset::LoadingPlugin;
use crate::menu::MenuPlugin;
use bevy::app::App;
use crate::physics::{PhysicPlugin,physics_settings::PhysicsParams};
mod config;
mod input;

use crate::car::{aero_system,do_input_from_state, esp_system,dash_start_system,dash_fps_system,dash_speed_update_system,spawn_car_start_system,spawn_car_system};
use crate::light::{animate_light_direction, light_start_system};
use crate::track::{SpawnCarOnTrackEvent, TrackPlugin};
use config::*;
use input::*;
use car::control::do_input;
use gamestate::GameState;
use crate::camera::CarCameraPlugin;
use crate::renet::NetworkMode;


#[cfg(debug_assertions)]
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, diagnostic::LogDiagnosticsPlugin};
use bevy::prelude::*;
use car::{PlayerCarSpawner, CarSet};
use crate::renet::server::NetServerPlugin;
use car::{PlayerCarCommandProcessor,server_network_sync, PlayerCarInputProcessor};
use crate::renet::client::{NetClientPlugin,NullPlayerCommand};
use crate::car::client_sync_entities;

#[cfg(feature = "graphics")]
pub fn setup_simple_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-20.5, 30.0, 20.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
use bevy::app::AppExit;
pub fn close_on_esc_ex(
    mut exit: EventWriter<AppExit>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}



fn bypass_menu_state(mut state: ResMut<NextState<GameState>>) {
    info!("Bypass to playing because of it's server");
    state.set(GameState::Playing);
}
pub fn car_app(app: &mut App,network_mode : NetworkMode) -> &mut App {
    //#[cfg(feature = "nn")]
    //let esp_run_after: CarSet = CarSet::NeuralNetwork;
    #[cfg(not(feature = "nn"))]
    let esp_run_after: CarSet = CarSet::Input;

    app.add_state::<GameState>()
        .add_plugins((
            LoadingPlugin,
            PhysicPlugin(PhysicsParams::make_default())
        ));
        if network_mode == NetworkMode::Server
        {
            app.add_plugins(NetServerPlugin::<PlayerCarSpawner,PlayerCarInputProcessor,PlayerCarCommandProcessor>::default());
            app.add_systems(OnEnter(GameState::Playing),setup_simple_camera)
            .add_systems(Update,(server_network_sync,do_input_from_state).run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Menu),bypass_menu_state);
        }
        else {
            app.add_plugins((InputPlugin::<6>,
                GraphicSettingPlugin,
                AudioPlugin,MenuPlugin,
                CarCameraPlugin))
               .add_systems(
                Update,
                (
                    do_input::<6>.in_set(CarSet::Input),
                ).run_if(in_state(GameState::Playing)))
                .add_systems(
                    OnEnter(GameState::Playing),
                    (
                        light_start_system,
                        dash_start_system,
                    ),
                )
                .add_systems(
                    Update,
                    (
                        animate_light_direction,
                        dash_fps_system,
                        dash_speed_update_system,
                    ).run_if(in_state(GameState::Playing)),
                );

        }

        if network_mode == NetworkMode::Client
        {
            app.add_plugins(NetClientPlugin::<PlayerCarSpawner,NullPlayerCommand>::default());
            app.add_systems(Update,client_sync_entities.run_if(bevy_renet::transport::client_connected()));
        }


        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
        
        app
        .insert_resource(Config::default())
        .add_plugins((
            TrackPlugin,
        ))
       .add_event::<SpawnCarOnTrackEvent>()
       .add_systems(
            Update,
            (
                aero_system.in_set(CarSet::Input),
                esp_system.in_set(CarSet::Esp).after(esp_run_after),
            ).run_if(in_state(GameState::Playing)),
        );


        if network_mode == NetworkMode::Standalone {
            app.add_systems(
                OnEnter(GameState::Playing),
                (
                    spawn_car_start_system,
                ),
            )
            .add_systems(
                Update,
                (
                    spawn_car_system,
                ).run_if(in_state(GameState::Playing)),
            );
        }
       
    #[cfg(feature = "nn")]
    {
        app.add_plugins(bevy_garage_nn::NeuralNetworkPlugin);
    }
    app.add_systems(Update,close_on_esc_ex);
    app
}
