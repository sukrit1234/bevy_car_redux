
use crate::GameState;
pub mod physics_settings;
use bevy::app::*;
use bevy::ecs::schedule::*;
use bevy_rapier3d::prelude::*;
use physics_settings::{rapier_config_start_system,PhysicsParams};
pub struct PhysicPlugin(pub PhysicsParams);

impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin {
            enabled: false,
            style: DebugRenderStyle {
                rigid_body_axes_length: 0.5,
                ..DebugRenderStyle::default()
            },
            mode: DebugRenderMode::COLLIDER_SHAPES
                | DebugRenderMode::RIGID_BODY_AXES
                | DebugRenderMode::JOINTS
                | DebugRenderMode::CONTACTS
                | DebugRenderMode::SOLVER_CONTACTS,
            ..RapierDebugRenderPlugin::default()
        })
        .add_systems(OnEnter(GameState::Playing),rapier_config_start_system)
        .insert_resource(self.0)
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Variable {
                max_dt: 1. / 60.,
                time_scale: 1.,
                substeps: self.0.substeps,
            },
            ..RapierConfiguration::default()
        });
    }
}