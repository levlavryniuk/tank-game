mod collider;
mod constants;
mod plugins;
mod walls;
use bevy::prelude::*;

use plugins::{
    collision::CollisionPlugin,
    shooting::BulletPlugin,
    tank::{Tank, TankPlugin},
};
use walls::setup_walls;

#[derive(Component, Default)]
pub struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((TankPlugin, BulletPlugin, CollisionPlugin))
        .add_systems(Startup, (camera_setup, setup_walls).chain())
        .add_systems(Update, (movement_system).chain())
        .run();
}

fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
