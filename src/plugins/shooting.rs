use bevy::{prelude::*, render::primitives::Aabb};

use crate::{
    constants::{BULLET_HALF_EXTENTS, BULLET_OFFSET, BULLET_SIZE, BULLET_SPEED},
    Tank, Velocity,
};

use super::collision::Dynamic;
use crate::collider::Collider;

#[derive(Component, Debug)]
pub struct Bullet {
    /// Angle in degrees
    pub angle: f32,
    pub speed: f32,
    pub bounce_count: u8,
    pub last_hit_wall: Option<Vec3>,
}

impl Bullet {
    fn new(angle: f32) -> Self {
        Self {
            angle,
            speed: BULLET_SPEED,
            bounce_count: 0,
            last_hit_wall: None,
        }
    }
    pub fn velocity(&self) -> Vec2 {
        let angle_rad = self.angle.to_radians();
        Vec2::new(angle_rad.cos(), angle_rad.sin()) * self.speed
    }
}
pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (shooting_system, bullet_velocity_system));
    }
}

fn bullet_velocity_system(mut query: Query<(&mut Velocity, &Bullet)>) {
    for (mut velocity, bullet) in query.iter_mut() {
        let x_modifier = bullet.angle.to_radians().cos();
        let y_modifier = bullet.angle.to_radians().sin();

        velocity.x = x_modifier * bullet.speed;
        velocity.y = y_modifier * bullet.speed;
    }
}

fn shooting_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Transform, With<Tank>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    let transform = query.single();

    let rotate = transform.rotation.to_euler(EulerRot::XYZ).2;

    let x_modifier = rotate.cos();
    let y_modifier = rotate.sin();
    let tank_x = transform.translation.x;
    let tank_y = transform.translation.y;

    let (bullet_x, bullet_y) = (
        tank_x + BULLET_OFFSET * x_modifier,
        tank_y + BULLET_OFFSET * y_modifier,
    );
    let center = (bullet_x, bullet_y, 0.).into();

    let aabb = Aabb {
        center,
        half_extents: BULLET_HALF_EXTENTS.into(),
    };
    let bullet = Bullet::new(rotate.to_degrees());
    commands
        //.spawn(ColorMesh2dBundle {
        //    mesh: meshes.add(Circle::new(BULLET_SIZE)).into(),
        //    transform: Transform::from_xyz(bullet_x, bullet_y, 0.),
        //    ..default()
        //})
        .spawn((
            Mesh2d(meshes.add(Circle::new(BULLET_SIZE))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from_color(Color::srgb(250.0, 50.0, 50.0))),
            ),
        ))
        .insert(Transform::from_xyz(bullet_x, bullet_y, 0.))
        .insert(Velocity { x: 0., y: 0. })
        .insert(bullet)
        .insert(Collider::Aabb(aabb))
        .insert(Dynamic);
}
