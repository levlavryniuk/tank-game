use bevy::{prelude::*, render::primitives::Aabb};

use crate::walls::WallType;

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

pub fn get_axes(rotation: f32) -> [Vec2; 2] {
    let angle = rotation.to_radians();
    let axis_1 = Vec2::new(angle.cos(), angle.sin());
    let axis_2 = Vec2::new(-angle.sin(), angle.cos());
    [axis_1, axis_2]
}

pub fn get_obb_projection_radius(axis: Vec2, half_extents: Vec2, u_x: Vec2, u_y: Vec2) -> f32 {
    half_extents.x * axis.dot(u_x).abs() + half_extents.y * axis.dot(u_y).abs()
}

pub fn get_aabb_projection_radius(axis: Vec2, half_extents: Vec2) -> f32 {
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

    // Centers of AABB and OBB
    let aabb_center = aabb.center.xy();
    let obb_center = obb.center;

    // Calculate the vector from the AABB to the OBB
    let translation_vec = obb_center - aabb_center;

    // Axes to check for separation: the axes of the AABB and OBB
    let aabb_axes = [Vec2::X, Vec2::Y];
    let obb_axes = get_axes(obb.rotation); // OBB's rotated axes
    let axes = aabb_axes.iter().chain(&obb_axes);

    // Half-extents of AABB
    let aabb_half_extents = aabb.half_extents.xy();

    for &axis in axes {
        // Project AABB and OBB onto the axis
        let aabb_radius = get_aabb_projection_radius(axis, aabb_half_extents);
        let obb_radius =
            get_obb_projection_radius(axis, obb.half_extents, obb_axes[0], obb_axes[1]);

        // Calculate the distance between projections along this axis
        let distance = translation_vec.dot(axis).abs();

        // Debugging print statements for tracking the values
        println!(
            "Axis: {:?}, Translation Vec: {:?}, Distance: {}, AABB Radius: {}, OBB Radius: {}",
            axis, translation_vec, distance, aabb_radius, obb_radius
        );

        // If there is a separating axis (distance is greater than sum of radii), return false
        if distance > aabb_radius + obb_radius {
            return false;
        }
    }
    println!("Collision detected with wall {:?}", aabb);

    // If no separating axis was found, return true (collision detected)
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
            (Collider::Obb(_), Collider::Obb(_)) => panic!("Obb-Obb collision not implemented"),
            (Collider::Aabb(_), Collider::Obb(_)) => aabb_x_obb(self.clone(), other.clone()),
            (Collider::Obb(_), Collider::Aabb(_)) => aabb_x_obb(other.clone(), self.clone()),
        }
    }

    pub fn compute_collision_normal(&self, other: &Collider) -> Vec2 {
        let delta = self.center() - other.center();
        if delta.x.abs() > delta.y.abs() {
            Vec2::new(delta.x.signum(), 0.0)
        } else {
            Vec2::new(0.0, delta.y.signum())
        }
    }

    pub fn compute_penetration_depth(&self, other: &Collider) -> f32 {
        let overlap_x = (self.half_extents().x + other.half_extents().x)
            - (self.center().x - other.center().x).abs();
        let overlap_y = (self.half_extents().y + other.half_extents().y)
            - (self.center().y - other.center().y).abs();

        if overlap_x > 0.0 && overlap_y > 0.0 {
            overlap_x.min(overlap_y)
        } else {
            0.0
        }
    }
    pub fn collision_info(
        &self,
        other: &Collider,
        wall_type: Option<WallType>,
    ) -> Option<(Vec2, f32)> {
        let delta = self.center() - other.center();
        let (x_bonus, y_bonus) = match wall_type {
            Some(WallType::Horizontal) => (0.0, 10.0),
            Some(WallType::Vertical) => (5.0, 0.0),
            None => (0.0, 0.0),
        };
        let overlap_x = (self.half_extents().x + other.half_extents().x) + x_bonus - delta.x.abs();
        let overlap_y = (self.half_extents().y + other.half_extents().y) + y_bonus - delta.y.abs();
        if overlap_x > 0.0 && overlap_y > 0.0 {
            if overlap_x < overlap_y {
                let normal = Vec2::new(delta.x.signum(), 0.0);
                Some((normal, overlap_x))
            } else {
                let normal = Vec2::new(0.0, delta.y.signum());
                Some((normal, overlap_y))
            }
        } else {
            None
        }
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
