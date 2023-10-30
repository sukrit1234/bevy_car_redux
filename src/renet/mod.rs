use bevy::prelude::*;
use bevy_renet::renet::{ConnectionConfig,transport::NETCODE_KEY_BYTES};

pub mod server;
pub mod client;

use server::*;
use client::*;

use crate::game_asset::GameAssets;

pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes
pub const PROTOCOL_ID: u64 = 7;

#[derive(PartialEq)]
pub enum NetworkMode
{
    Standalone,
    Client,
    Server
}

#[derive(Debug, Component)]
pub struct NetPlayer {
    pub id: u64,
}

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: ClientChannel::channels_config(),
        server_channels_config: ServerChannel::channels_config(),
    }
}


pub trait PlayerSpawner : Send + Sync + 'static {
    fn spawn_authority(cmd: &mut Commands,game_asset : &Res<GameAssets>,archetype : u128,players : &Query<(Entity, &NetPlayer, &Transform)>) -> (bool,Entity,Transform);
    fn spawn_proxy(cmd: &mut Commands,game_asset : &Res<GameAssets>,archetype : u128,transform : Transform,local_player : bool) -> (bool,Entity);
}