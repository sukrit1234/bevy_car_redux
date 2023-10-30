use bevy::prelude::*;
use crate::{track::{spawn_car_on_track, SpawnCarOnTrackEvent, TrackConfig}, game_asset::{self, GameAssets}};

pub fn spawn_car_start_system(mut car_spawn_events: EventWriter<SpawnCarOnTrackEvent>) {
    car_spawn_events.send(SpawnCarOnTrackEvent {
        player: true,
        index: 0,
        position: Some(0.),
    });
}

pub fn spawn_car_system(
    mut events: EventReader<SpawnCarOnTrackEvent>,
    mut cmd: Commands,
    track_config: ResMut<TrackConfig>,
    game_asset: ResMut<GameAssets>,
) {
    for spawn_event in events.iter() {
        dbg!(spawn_event);

        let (transform, init_meters) = if let Some(init_meters) = spawn_event.position {
            let (translate, quat) = track_config.get_transform_by_meter(init_meters);
            let transform = Transform::from_translation(translate).with_rotation(quat);
            (transform, init_meters)
        } else {
            track_config.get_transform_random()
        };

        spawn_car_on_track(
            &mut cmd,
            &game_asset.car_body,
            &game_asset.wheel_body,
            spawn_event.player,
            transform,
            spawn_event.index,
            init_meters,
        );
    }
}
