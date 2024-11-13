use bevy::prelude::*;

use crate::collider::{Collider, Obb};
use crate::{
    constants::{GAME_FIELD_HEIGHT, GAME_FIELD_WIDTH, GRID_CELL_SIZE},
    Velocity,
};

use super::collision::Dynamic;
pub const TANK_LENGTH: f32 = 60.;
pub const TANK_WIDTH: f32 = 40.;
pub const TANK_ROTATION_SPEED: f32 = 1.5;

pub const TANK_X_HALF_EXTENT: f32 = TANK_LENGTH / 2.;
pub const TANK_Y_HALF_EXTENT: f32 = TANK_WIDTH / 2.;
pub const TANK_SPEED: f32 = 100.;
pub const TANK_SIZE: (f32, f32) = (60., 40.);
#[derive(Component, Default)]
pub struct Tank {
    pub rotation_speed: f32,
    pub speed: f32,
}

impl Tank {
    pub fn size() -> Vec2 {
        TANK_SIZE.into()
    }
    pub fn new() -> Self {
        Self {
            speed: TANK_SPEED,
            rotation_speed: TANK_ROTATION_SPEED,
        }
    }
    pub fn half_extents() -> Vec2 {
        (TANK_LENGTH / 2.0, TANK_WIDTH / 2.0).into()
    }
}

pub struct TankPlugin;
impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, tank_movement_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let center = (
        -GAME_FIELD_WIDTH / 2. + GRID_CELL_SIZE / 2.,
        GAME_FIELD_HEIGHT / 2. - GRID_CELL_SIZE / 2.,
    );
    let obb = Obb {
        center: center.into(),
        half_extents: Tank::half_extents(),
        rotation: 0.,
    };
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("tank.png"),
            sprite: Sprite {
                custom_size: Some(Tank::size()),
                ..default()
            },
            transform: Transform::from_xyz(center.0, center.1, 0.),

            ..default()
        })
        .insert(Tank::new())
        .insert(Collider::Obb(obb))
        .insert(Dynamic)
        .insert(Velocity::default());
}
fn tank_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Tank, &mut Transform, &mut Velocity)>,
) {
    let (tank, mut transform, mut velocity) = query.single_mut();
    let rotation_amount = if keyboard_input.pressed(KeyCode::KeyA) {
        tank.rotation_speed
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        -tank.rotation_speed
    } else {
        0.0
    };

    let rotation_radians = rotation_amount.to_radians();
    transform.rotate(Quat::from_rotation_z(rotation_radians));

    let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
    let direction = Vec2::new(angle.cos(), angle.sin());

    let movement = if keyboard_input.pressed(KeyCode::KeyW) {
        tank.speed
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        -tank.speed
    } else {
        0.0
    };
    velocity.x = direction.x * movement;
    velocity.y = direction.y * movement;
}
