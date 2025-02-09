mod collider;
mod constants;
mod network_plugin;
mod plugins;
mod walls;
use bevy::prelude::*;

use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use plugins::tank::{Tank, TankPlugin};
use walls::setup_walls;

#[derive(Component, Default)]
pub struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((TankPlugin,))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, (camera_setup, setup_walls).chain())
        //.add_systems(PostUpdate, movement_system)
        .run();
}
fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
