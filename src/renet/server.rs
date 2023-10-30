use bevy::prelude::*;
use bincode::Error;
use serde::{Deserialize, Serialize};
use std::{time::Duration, marker::PhantomData};
use bevy_egui::EguiPlugin;

use crate::game_asset::GameAssets;
use crate::gamestate::GameState;
use crate::renet::{NetPlayer,PlayerSpawner};
use crate::renet::client::ClientChannel;

use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        RenetServer, ServerEvent,ChannelConfig, SendType,Bytes
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};

pub trait PlayerCommandProcessor : Send + Sync + 'static {
    fn process_command(client_id : u64,cmd: &mut Commands,message : &Bytes,lobby : &mut ResMut<ServerLobby>);
}  

pub struct NullPlayerCommandProcessor;
impl PlayerCommandProcessor for NullPlayerCommandProcessor{
    fn process_command(_client_id : u64,_cmd: &mut Commands,_message : &Bytes,_lobby : &mut ResMut<ServerLobby>){}
}

pub struct NullPlayerInputProcessor;
impl PlayerCommandProcessor for NullPlayerInputProcessor{
    fn process_command(_client_id : u64,_cmd: &mut Commands,_message : &Bytes,_lobby : &mut ResMut<ServerLobby>){}
}


pub enum ServerChannel {
    ServerMessages,
    NetworkedEntities,
}


#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerCreate {
        id: u64,
        entity: Entity,
        translation: [f32; 3],
        rotation:  [f32; 4],
        scale:[f32; 3],
    },
    PlayerRemove {
        id: u64,
    },
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::NetworkedEntities => 0,
            ServerChannel::ServerMessages => 1,
        }
    }
}

impl ServerChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: Self::NetworkedEntities.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::Unreliable,
            },
            ChannelConfig {
                channel_id: Self::ServerMessages.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(200),
                },
            },
        ]
    }
}
use crate::renet::{connection_config, PROTOCOL_ID};
use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use crate::input::controller::*;

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<u64, Entity>,
}

fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(connection_config());

    let addr = if let Ok(addr) = std::env::var("RENET_SERVER_SOCKET") {
        addr
    } else {
        let default = "127.0.0.1:5000".to_string();
        println!("RENET_SERVER_SOCKET not set, setting default: {}", &default);
        default
    };

    let public_addr = addr.parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time: std::time::Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

    (server, transport)
}

#[cfg(feature = "graphics")]
fn update_visulizer_system(
    mut egui_contexts: bevy_egui::EguiContexts,
    mut visualizer: ResMut<renet_visualizer::RenetServerVisualizer<200>>,
    server: Res<RenetServer>,
) {
    visualizer.update(&server);
    visualizer.show_window(egui_contexts.ctx_mut());
}

/*Form player create message*/
fn form_player_create_message(id : u64,entity : Entity,transform : Transform) -> Result<Vec<u8>,Error> {
    let translation: [f32; 3] = transform.translation.into();
    let rotation: [f32; 4] = transform.rotation.to_array();
    let scale: [f32; 3] = transform.scale.into();

    bincode::serialize(&ServerMessages::PlayerCreate {
        id: id,
        entity,
        translation,
        rotation,
        scale
    })
}

pub fn server_process_client_connections<PS : PlayerSpawner>(
    mut server_events: EventReader<ServerEvent>,
    mut cmd: Commands,
    game_asset : Res<GameAssets>,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    players: Query<(Entity, &NetPlayer, &Transform)>,
    #[cfg(feature = "graphics")] 
    mut visualizer: ResMut<renet_visualizer::RenetServerVisualizer<200>>,
) 
{
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);
                #[cfg(feature = "graphics")]
                visualizer.add_client(*client_id);

                for (entity, player, transform) in players.iter() {
                    let message = form_player_create_message(player.id,entity,*transform).unwrap();
                    server.send_message(*client_id, ServerChannel::ServerMessages, message);
                }

                let archetype : u128 = 0;
                let (success,entity,transform) = PS::spawn_authority(&mut cmd,&game_asset,archetype,&players);
                if success
                {
                    cmd.entity(entity)
                    .insert(NetPlayer { id: *client_id })
                    .insert(PlayerInputState::default());

                    lobby.players.insert(*client_id, entity);
                    let message = form_player_create_message(*client_id,entity,transform).unwrap();
                    server.broadcast_message(ServerChannel::ServerMessages, message);
                }
                else {
                    error!("Can not spawn player entity with Arch : {} , for Client {}",archetype,client_id);
                }
                
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
                #[cfg(feature = "graphics")]
                visualizer.remove_client(*client_id);
                if let Some(player_entity) = lobby.players.remove(client_id) {
                    cmd.entity(player_entity).despawn();
                }

                let message = bincode::serialize(&ServerMessages::PlayerRemove { id: *client_id }).unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
        }
    }
}

pub fn server_process_client_command<Processor : PlayerCommandProcessor> (
    mut cmd: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
) {
   
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
            Processor::process_command(client_id,&mut cmd,&message,&mut lobby);
        }
    }
}

pub fn server_process_client_input<Processor : PlayerCommandProcessor> (
    mut cmd: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
) {
   
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            Processor::process_command(client_id,&mut cmd,&message,&mut lobby);
        }
    }
}




pub struct NetServerPlugin<PS : PlayerSpawner,I : PlayerCommandProcessor,C : PlayerCommandProcessor>{
    _ps : PhantomData<PS>,
    _i : PhantomData<I>,
    _c : PhantomData<C>
}
impl<PS : PlayerSpawner,I : PlayerCommandProcessor,C : PlayerCommandProcessor> Default for NetServerPlugin<PS,I,C> {
    fn default() -> Self { 
        NetServerPlugin::<PS,I,C> {
            _ps : PhantomData,
            _i : PhantomData,
            _c : PhantomData
        }
    }
}

impl<PS : PlayerSpawner,I : PlayerCommandProcessor,C : PlayerCommandProcessor> Plugin for NetServerPlugin<PS,I,C> {
    fn build(&self, app: &mut App) {

        app.add_plugins((
            RenetServerPlugin,
            NetcodeServerPlugin,
            EguiPlugin,
        ))
        .insert_resource(ServerLobby::default())
        .add_systems(Update,
            (server_process_client_connections::<PS>,
                     server_process_client_command::<C>,
                     server_process_client_input::<I>).run_if(
                      in_state(GameState::Playing)
                                .or_else(in_state(GameState::Pause))));
                            
        #[cfg(feature = "graphics")]
        {
            app.add_systems(Update, update_visulizer_system)
            .insert_resource(renet_visualizer::RenetServerVisualizer::<200>::default());
        }

        let (server, transport) = new_renet_server();
        app.insert_resource(server).insert_resource(transport);
    }
}