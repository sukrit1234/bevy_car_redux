use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::input::controller::PlayerInputState;
use crate::renet::client::*;
use crate::renet::server::*;
use bevy_renet::renet::{RenetClient,RenetServer,Bytes};
use crate::car::*;
use crate::renet::server::PlayerCommandProcessor;
use crate::renet::{PlayerSpawner,NetPlayer};
use crate::game_asset::GameAssets;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NetworkedEntities {
    pub entities: Vec<Entity>,
    pub translations: Vec<[f32; 3]>,
    pub rotations: Vec<[f32; 4]>,
    pub wheels_translations: Vec<[[f32; 3]; 4]>,
    pub wheels_rotations: Vec<[[f32; 4]; 4]>,
}

pub fn client_sync_entities(
    mut cmd: Commands,
    mut client: ResMut<RenetClient>,
    network_mapping: Res<NetworkMapping>,
    car_wheels: Query<&CarWheels>,
    mut wheel_query: Query<&mut Transform, With<Wheel>>,
) 
{
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();
        for i in 0..networked_entities.entities.len() {

            if let Some(entity) = network_mapping.0.get(&networked_entities.entities[i]) {
                let translation = networked_entities.translations[i].into();
                let rotation: Quat = Quat::from_array(networked_entities.rotations[i]);
                let transform = Transform {
                    translation,
                    rotation,
                    ..Default::default()
                };
                cmd.entity(*entity).insert(transform);

                let translations = networked_entities.wheels_translations[i];
                let rotations = networked_entities.wheels_rotations[i];

                let car_wheels = car_wheels.get(*entity);
                if let Ok(car_wheels) = car_wheels {
                    for (i, e) in car_wheels.entities.iter().enumerate() {
                        let mut wheel_transform = wheel_query.get_mut(*e).unwrap();
                        wheel_transform.translation = translations[i].into();
                        wheel_transform.rotation = Quat::from_array(rotations[i]);
                    }
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn server_network_sync(
    mut server: ResMut<RenetServer>,
    mut tr_set: ParamSet<(
        Query<(Entity, &Transform, &CarWheels), With<NetPlayer>>,
        Query<&Transform, With<Wheel>>,
    )>,
) {
    let mut networked_entities = NetworkedEntities::default();
    let mut wheels_all: Vec<[Entity; 4]> = vec![];
    
    for (entity, transform, wheels) in tr_set.p0().iter() {
        networked_entities.entities.push(entity);
        networked_entities
            .translations
            .push(transform.translation.into());
        networked_entities.rotations.push(transform.rotation.into());

        wheels_all.push(wheels.entities);
    }
    for wheels in wheels_all {
        networked_entities.wheels_translations.push([
            tr_set.p1().get(wheels[0]).unwrap().translation.into(),
            tr_set.p1().get(wheels[1]).unwrap().translation.into(),
            tr_set.p1().get(wheels[2]).unwrap().translation.into(),
            tr_set.p1().get(wheels[3]).unwrap().translation.into(),
        ]);
        networked_entities.wheels_rotations.push([
            tr_set.p1().get(wheels[0]).unwrap().rotation.into(),
            tr_set.p1().get(wheels[1]).unwrap().rotation.into(),
            tr_set.p1().get(wheels[2]).unwrap().rotation.into(),
            tr_set.p1().get(wheels[3]).unwrap().rotation.into(),
        ]);
    }

    
    let sync_message = bincode::serialize(&networked_entities).unwrap();
    server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
}

pub struct PlayerCarCommandProcessor;
impl PlayerCommandProcessor for PlayerCarCommandProcessor
{
    fn process_command(_client_id : u64,_cmd: &mut Commands,_message : &Bytes,_lobby : &mut ResMut<ServerLobby>)
    {

    }
}

pub struct PlayerCarInputProcessor;
impl PlayerCommandProcessor for PlayerCarInputProcessor
{
    fn process_command(client_id : u64,cmd: &mut Commands,message : &Bytes,lobby : &mut ResMut<ServerLobby>)
    {
        let input: PlayerInputState = bincode::deserialize(&message).unwrap();
        if let Some(player_entity) = lobby.players.get(&client_id) {
            cmd.entity(*player_entity).insert(input);
        }
    }
}

pub struct PlayerCarSpawner;
impl PlayerSpawner for PlayerCarSpawner
{
    fn spawn_authority(cmd: &mut Commands,game_asset : &Res<GameAssets>,_archetype : u128,_players : &Query<(Entity, &NetPlayer, &Transform)>) -> (bool,Entity,Transform)
    {
        let transform = Transform::from_xyz(
            (fastrand::f32() - 0.5) * 40.,
            0.51,
            (fastrand::f32() - 0.5) * 40.,
        );
        let player_entity = spawn_car(
            cmd,
            #[cfg(feature = "graphics")]
            &game_asset.car_body,
            #[cfg(feature = "graphics")]
            &game_asset.wheel_body,
            false,
            transform,
        );
        (true,player_entity,transform)
    }
    fn spawn_proxy(cmd: &mut Commands,game_asset : &Res<GameAssets>,_archetype : u128,transform : Transform,local_player : bool) -> (bool,Entity)
    {
        let client_entity = car::spawn_car(
            cmd,
            &game_asset.car_body,
            &game_asset.wheel_body,
            local_player,
            transform,
        );
        (true,client_entity)
    }
}