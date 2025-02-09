use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::{GAME_FIELD_HEIGHT, GAME_FIELD_WIDTH, GRID_CELL_SIZE};
pub const TANK_LENGTH: f32 = 60.;
pub const TANK_WIDTH: f32 = 40.;
pub const TANK_ROTATION_SPEED: f32 = 90.0;

pub const TANK_X_HALF_EXTENT: f32 = TANK_LENGTH / 2.;
pub const TANK_Y_HALF_EXTENT: f32 = TANK_WIDTH / 2.;
pub const TANK_SPEED: f32 = 150.;
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
    commands
        .spawn((
            Sprite {
                image: asset_server.load("tank.png").into(),
                custom_size: Some(Tank::size()),
                ..default()
            },
            RigidBody::Dynamic,
        ))
        .insert(Transform::from_xyz(center.0, center.1, 0.))
        .insert(GravityScale(0.))
        .insert(KinematicCharacterController { ..default() })
        .insert(Velocity {
            linvel: Vec2::ZERO,
            angvel: 1.,
        })
        .insert(Collider::cuboid(TANK_X_HALF_EXTENT, TANK_Y_HALF_EXTENT))
        .insert(Tank::new());
}
fn tank_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Tank, &mut Transform, &mut Velocity)>,
) {
    let (tank, transform, mut velocity) = query.single_mut();

    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity.angvel = tank.rotation_speed.to_radians();
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.angvel = -tank.rotation_speed.to_radians();
    } else {
        velocity.angvel = 0.0;
    }

    let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
    let direction = Vec2::new(angle.cos(), angle.sin());

    let movement = if keyboard_input.pressed(KeyCode::KeyW) {
        tank.speed
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        -tank.speed
    } else {
        0.0
    };

    velocity.linvel = direction * movement;
}
