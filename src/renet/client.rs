
use bevy::prelude::*;
use bevy_renet::renet::{ChannelConfig, SendType,transport::NetcodeTransportError};
use serde::Serialize;
use std::marker::PhantomData;
use std::time::Duration;
use bevy_egui::{EguiPlugin,EguiContexts};
use crate::game_asset::GameAssets;
use crate::renet::PlayerSpawner;
use crate::renet::server::{ServerChannel,ServerMessages};
use crate::gamestate::*;

pub enum ClientChannel {
    Input,
    Command,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Command => 0,
            ClientChannel::Input => 1,
        }
    }
}
impl ClientChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: Self::Input.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                },
            },
            ChannelConfig {
                channel_id: Self::Command.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                },
            },
        ]
    }
}

use crate::renet::{connection_config, PROTOCOL_ID};

use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use renet_visualizer::{RenetClientVisualizer, RenetVisualizerStyle};
use std::{collections::HashMap, net::UdpSocket, time::SystemTime};
use crate::input::controller::*;

#[derive(Component)]
pub struct ControlledPlayer;

#[derive(Default, Resource)]
pub struct NetworkMapping(pub HashMap<Entity, Entity>);

#[derive(Debug)]
pub struct PlayerInfo {
    pub client_entity: Entity,
    pub server_entity: Entity,
}

#[derive(Debug, Default, Resource)]
pub struct ClientLobby {
    pub players: HashMap<u64, PlayerInfo>,
}

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let client = RenetClient::new(connection_config());

    let addr = if let Ok(addr) = std::env::var("RENET_SERVER_ADDR") {
        println!("RENET_SERVER_ADDR: {}", &addr);
        addr
    } else {
        let default = "127.0.0.1:5000".to_string();
        println!("RENET_SERVER_ADDR not set, setting default: {}", &default);
        default
    };

    let server_addr = addr.parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    (client, transport)
}

fn update_visulizer_system(
    mut egui_contexts: EguiContexts,
    mut visualizer: ResMut<RenetClientVisualizer<200>>,
    client: Res<RenetClient>,
    mut show_visualizer: Local<bool>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    visualizer.add_network_info(client.network_info());
    if keyboard_input.just_pressed(KeyCode::F1) {
        *show_visualizer = !*show_visualizer;
    }
    if *show_visualizer {
        visualizer.show_window(egui_contexts.ctx_mut());
    }
}

pub trait PlayerCommand :  Event + Serialize {}

#[derive(Serialize,Event,Default)]
pub struct  NullPlayerCommand;
impl PlayerCommand for  NullPlayerCommand {
    
}

fn client_send_input(
    player_input: Res<PlayerInputState>,
    mut client: ResMut<RenetClient>) 
{
    let input_message = bincode::serialize(&*player_input).unwrap();
    client.send_message(ClientChannel::Input, input_message);
}

fn client_send_player_commands<C : PlayerCommand>(
    mut player_commands: EventReader<C>,
    mut client: ResMut<RenetClient>,
) {
    for command in player_commands.iter() {
        let command_message = bincode::serialize(command).unwrap();
        client.send_message(ClientChannel::Command, command_message);
    }
}

// If any error is found we just panic
fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
    for e in renet_error.iter() {
        panic!("{}", e);
    }
}


pub fn client_sync_players<PS : PlayerSpawner>(
    mut cmd: Commands,
    mut client: ResMut<RenetClient>,
    game_asset : Res<GameAssets>,
    transport: Res<NetcodeClientTransport>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkMapping>,
) 
{
    let client_id = transport.client_id();
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerCreate {id,translation,rotation,scale,entity,} =>
            {
                println!("Player {} connected.", id);                
                let is_player = client_id == id;

                // let transform: Transform = Transform::from_translation(translation);
                let transform: Transform = Transform::IDENTITY
                    .with_translation(Vec3::from_array(translation))
                    .with_rotation(Quat::from_array(rotation))
                    .with_scale(Vec3::from_array(scale));

                let archetype : u128 = 0;
                let (success,client_entity) = PS::spawn_proxy(&mut cmd,&game_asset, archetype, transform,is_player);
                if success
                {
                    let player_info = PlayerInfo {
                        server_entity: entity,
                        client_entity : client_entity,
                    };
                    if is_player
                    {
                        cmd.entity(client_entity).insert(ControlledPlayer);
                    }
                    lobby.players.insert(id, player_info);
                    network_mapping.0.insert(entity, client_entity);
                }
            }
            ServerMessages::PlayerRemove { id } => {
                println!("Player {} disconnected.", id);
                if let Some(PlayerInfo {server_entity,client_entity,}) = lobby.players.remove(&id)
                {
                    cmd.entity(client_entity).despawn();
                    network_mapping.0.remove(&server_entity);
                }
            }
        }
    }
}

use bevy::app::AppExit;
fn disconnent_on_app_exit(mut exit: EventReader<AppExit>,mut transport : ResMut<NetcodeClientTransport>) {
    for _ev in exit.iter() {
        transport.disconnect();
        break;
    }
}

pub struct NetClientPlugin<PS : PlayerSpawner,C : PlayerCommand>{
    c : PhantomData<C>,
    sp : PhantomData<PS>
}

impl<PS : PlayerSpawner,C : PlayerCommand> Default for NetClientPlugin<PS,C> {
    fn default() -> Self { 
        NetClientPlugin::<PS,C> {
            c : PhantomData,
            sp : PhantomData
        }
    }
}
impl<PS : PlayerSpawner,C : PlayerCommand> Plugin for NetClientPlugin<PS,C> {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RenetClientPlugin,
            NetcodeClientPlugin,
            EguiPlugin,
        ))
        .add_event::<NullPlayerCommand>()
        .insert_resource(ClientLobby::default())
        .insert_resource(NetworkMapping::default())
        .insert_resource(RenetClientVisualizer::<200>::new(
            RenetVisualizerStyle::default(),
        ))
        .add_systems(Update,(update_visulizer_system,panic_on_error_system))
        .add_systems(Update, (
            client_send_input,
            client_send_player_commands::<C>,
            client_sync_players::<PS>)
            .run_if(bevy_renet::transport::client_connected().and_then(in_state(GameState::Playing).or_else(in_state(GameState::Pause)))));
        
        let (client, transport) = new_renet_client();
        app.insert_resource(client)
           .insert_resource(transport);
        app.add_systems(PostUpdate,disconnent_on_app_exit);
    }
}