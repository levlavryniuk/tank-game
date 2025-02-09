use std::{
    net::{Ipv4Addr, UdpSocket},
    time::SystemTime,
};

use crate::constants::SERVER_ADDR;
use bevy::prelude::*;
use bevy_renet::{
    netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport},
    renet::{ConnectionConfig, DefaultChannel, RenetClient},
    RenetClientPlugin,
};
use bincode;
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
struct PlayerState {
    linvel: Vec2,
    position: Vec2,
    rotation: f32,
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
struct BulletState {
    position: Vec2,
    angle: f32,
}

#[derive(Resource, Serialize, Deserialize)]
struct GameState {
    players: std::collections::HashMap<u64, PlayerState>,
    bullets: Vec<BulletState>,
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetClientPlugin)
            .add_plugins(NetcodeClientPlugin)
            .insert_resource(new_client())
            .insert_resource(new_transport())
            .insert_resource(GameState {
                players: std::collections::HashMap::new(),
                bullets: Vec::new(),
            })
            .insert_resource(PlayerState {
                position: Vec2::ZERO,
                linvel: Vec2::ZERO,
                rotation: 0.0,
            })
            .add_systems(
                PostUpdate,
                (send_movement_system, receive_game_state_system),
            );
    }
}

fn new_client() -> RenetClient {
    RenetClient::new(ConnectionConfig::default())
}

fn new_transport() -> NetcodeClientTransport {
    let server_ip: Ipv4Addr = SERVER_ADDR.parse().expect("Invalid SERVER_ADDR");
    let socket_addr = std::net::SocketAddr::new(server_ip.into(), 5000);

    let authentication = ClientAuthentication::Unsecure {
        server_addr: socket_addr,
        client_id: 1,
        user_data: None,
        protocol_id: 12345,
    };

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    NetcodeClientTransport::new(current_time, authentication, socket).unwrap()
}

// Send input data
fn send_movement_system(
    mut client: ResMut<RenetClient>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_state: ResMut<PlayerState>,
) {
    let mut moved = false;

    if keyboard_input.pressed(KeyCode::KeyW) {
        player_state.position.y += 1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        player_state.position.y -= 1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        player_state.position.x -= 1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player_state.position.x += 1.0;
        moved = true;
    }

    if moved {
        println!("Sending movement: {:?}", player_state.position);
        let movement_data = bincode::serialize(&*player_state).unwrap();
        client.send_message(DefaultChannel::ReliableOrdered, movement_data);
    }
}

// Receive game state
fn receive_game_state_system(mut client: ResMut<RenetClient>, mut game_state: ResMut<GameState>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        if let Ok(state) = bincode::deserialize::<GameState>(&message) {
            *game_state = state;
        }
    }
}
