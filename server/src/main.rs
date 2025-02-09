use bevy::prelude::*;
use bevy_renet::{
    netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent},
    RenetServerPlugin,
};
use bincode;
use serde::{Deserialize, Serialize};
use std::{net::UdpSocket, time::SystemTime};

#[derive(Component, Default, Debug, Clone, Serialize, Deserialize)]
struct PlayerState {
    position: Vec2,
    rotation: f32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
struct BulletState {
    position: Vec2,
    angle: f32,
}

#[derive(Resource, Default, Serialize, Deserialize)]
struct GameState {
    player_blue: PlayerState,
    player_red: PlayerState,
    //bullets: Vec<BulletState>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .insert_resource(new_server())
        .insert_resource(new_transport())
        .insert_resource(GameState::default())
        .add_systems(
            Update,
            (
                handle_events_system,
                receive_message_system,
                broadcast_state_system,
            ),
        )
        .run();
}

fn new_server() -> RenetServer {
    RenetServer::new(ConnectionConfig::default())
}

fn new_transport() -> NetcodeServerTransport {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    socket.set_nonblocking(true).unwrap();

    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 2,
        protocol_id: 12345,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    NetcodeServerTransport::new(server_config, socket).unwrap()
}

// Handle client connections
fn handle_events_system(
    mut server_events: EventReader<ServerEvent>,
    mut game_state: ResMut<GameState>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {} connected", client_id);
                game_state.players.insert(
                    *client_id,
                    PlayerState {
                        position: Vec2::ZERO,
                        rotation: 0.0,
                    },
                );
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {} disconnected: {:?}", client_id, reason);
                game_state.players.remove(client_id);
            }
        }
    }
}

// Receive movement and shooting actions from clients
fn receive_message_system(mut server: ResMut<RenetServer>, mut game_state: ResMut<GameState>) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            if let Ok(update) = bincode::deserialize::<PlayerState>(&message) {
                if let Some(player) = game_state.players.get_mut(&client_id) {
                    println!(
                        "Received movement from {}: {:?}",
                        client_id, update.position
                    );
                    player.position = update.position;
                    player.rotation = update.rotation;
                }
            }
        }
    }
}

// Send game state to all clients
fn broadcast_state_system(mut server: ResMut<RenetServer>, game_state: Res<GameState>) {
    let state_data = bincode::serialize(&*game_state).unwrap(); // Extract the inner struct
    server.broadcast_message(DefaultChannel::ReliableOrdered, state_data);
}
