use bevy::{prelude::*, render::primitives::Aabb};

#[derive(Debug, Clone)]
pub struct Obb {
    pub center: Vec2,
    pub half_extents: Vec2,
    pub rotation: f32,
}

#[derive(Component, Debug, Clone)]
pub enum Collider {
    Aabb(Aabb),
    Obb(Obb),
}

pub fn get_axes(rotation: f32) -> [Vec2; 4] {
    let angle = rotation.to_radians();
    let axis_1 = (angle.cos(), angle.sin()).into();
    let axis_2 = (-angle.sin(), angle.cos()).into();
    [axis_1, axis_2, Vec2::X, Vec2::Y]
}

pub fn get_obb_projection_radius(axis: Vec2, half_extents: Vec2, u_x: Vec2, u_y: Vec2) -> f32 {
    half_extents.x * (axis.dot(u_x)).abs() + half_extents.y * (axis.dot(u_y)).abs()
}

pub fn get_aabb_projection_radius(axis: Vec2, half_extents: Vec2) -> f32 {
    // For AABB, project directly onto the axis using its half extents
    half_extents.x * axis.x.abs() + half_extents.y * axis.y.abs()
}
fn aabb_x_obb(aabb: Collider, obb: Collider) -> bool {
    let aabb = match aabb {
        Collider::Aabb(aabb) => aabb,
        _ => panic!("Expected Aabb"),
    };
    let obb = match obb {
        Collider::Obb(obb) => obb,
        _ => panic!("Expected Obb"),
    };
    let axes = get_axes(obb.rotation);
    let u_x = axes[0];
    let u_y = axes[1];
    let other_axes = [Vec2::X, Vec2::Y];
    let other_half_extents = aabb.half_extents.xy();
    let other_center = aabb.center.xy();
    let center = obb.center;
    let half_extents = obb.half_extents;
    let translation_vec = other_center - center;
    for axis in axes.iter().chain(other_axes.iter()) {
        let tank_radius = get_obb_projection_radius(*axis, half_extents, u_x, u_y);
        let wall_radius = get_aabb_projection_radius(*axis, other_half_extents);
        let distance = translation_vec.dot(*axis).abs();
        if distance > tank_radius + wall_radius {
            return false;
        }
    }
    true
}
fn aabb_x_aabb(aabb_1: Collider, aabb_2: Collider) -> bool {
    let aabb_1 = match aabb_1 {
        Collider::Aabb(aabb) => aabb,
        _ => panic!("Expected Aabb"),
    };
    let aabb_2 = match aabb_2 {
        Collider::Aabb(aabb) => aabb,
        _ => panic!("Expected Aabb"),
    };
    let min = aabb_1.min();
    let max = aabb_1.max();
    let other_min = aabb_2.min();
    let other_max = aabb_2.max();
    min.x <= other_max.x && max.x >= other_min.x && min.y <= other_max.y && max.y >= other_min.y
}
impl Collider {
    pub fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Collider::Aabb(_), Collider::Aabb(_)) => aabb_x_aabb(self.clone(), other.clone()),
            (Collider::Obb(_), Collider::Obb(_)) => panic!("Not implemented"),
            (Collider::Aabb(_), Collider::Obb(_)) => aabb_x_obb(self.clone(), other.clone()),
            (Collider::Obb(_), Collider::Aabb(_)) => aabb_x_obb(other.clone(), self.clone()),
        }
    }
    pub fn compute_collision_normal(&self, other: &Collider) -> Vec2 {
        // Simplified example for axis-aligned bounding boxes
        let delta = self.center() - other.center();
        if delta.x.abs() > delta.y.abs() {
            Vec2::new(delta.x.signum(), 0.0)
        } else {
            Vec2::new(0.0, delta.y.signum())
        }
    }

    pub fn compute_penetration_depth(&self, other: &Collider) -> f32 {
        // Simplified example for axis-aligned bounding boxes
        let overlap_x = (self.half_extents().x + other.half_extents().x)
            - (self.center().x - other.center().x).abs();
        let overlap_y = (self.half_extents().y + other.half_extents().y)
            - (self.center().y - other.center().y).abs();
        overlap_x.min(overlap_y)
    }

    pub fn center(&self) -> Vec2 {
        match self {
            Collider::Aabb(aabb) => aabb.center.xy(),
            Collider::Obb(obb) => obb.center,
        }
    }

    pub fn half_extents(&self) -> Vec2 {
        match self {
            Collider::Aabb(aabb) => aabb.half_extents.xy(),
            Collider::Obb(obb) => obb.half_extents,
        }
    }
}
