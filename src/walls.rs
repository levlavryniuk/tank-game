use bevy::{prelude::*, render::primitives::Aabb, utils::HashSet};
use bevy_rapier2d::prelude::*;
use rand::seq::SliceRandom;

const H_WALL_HALF_SIZE: (f32, f32, f32) = (GRID_CELL_SIZE / 2., 2.5, 0.);
const V_WALL_HALF_SIZE: (f32, f32, f32) = (2.5, GRID_CELL_SIZE / 2., 0.);

use crate::{
    constants::{
        GAME_FIELD_HEIGHT, GAME_FIELD_WIDTH, GRID_CELL_HORIZONTAL_AMOUNT, GRID_CELL_SIZE,
        GRID_CELL_VERTICAL_AMOUNT,
    },
    plugins::collision::Static,
};

#[derive(Clone)]
pub enum WallType {
    Horizontal,
    Vertical,
}
#[derive(Component)]
pub struct Wall {
    pub wall_type: WallType,
}
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
    pub fn offset_f32(&self) -> (f32, f32, f32) {
        let (x, y) = self.offset();
        (x as f32, y as f32, 0.0)
    }
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl From<Direction> for WallType {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up | Direction::Down => WallType::Horizontal,
            Direction::Left | Direction::Right => WallType::Vertical,
        }
    }
}

pub fn setup_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mut frontier = vec![];
    let mut visited =
        vec![vec![false; GRID_CELL_VERTICAL_AMOUNT as usize]; GRID_CELL_HORIZONTAL_AMOUNT as usize];
    let mut walls = HashSet::new();

    for x in 0..GRID_CELL_HORIZONTAL_AMOUNT as usize {
        for y in 0..GRID_CELL_VERTICAL_AMOUNT as usize {
            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                walls.insert((x, y, direction));
            }
        }
    }

    let horizontal_wall_mesh: Handle<Mesh> = meshes.add(Rectangle::new(GRID_CELL_SIZE, 5.0)).into();
    let vertical_wall_mesh: Handle<Mesh> = meshes.add(Rectangle::new(5.0, GRID_CELL_SIZE)).into();

    let start_x = 0;
    let start_y = 0;
    mark_cell_as_maze(start_x, start_y, &mut visited, &mut frontier);

    while let Some((x, y)) = frontier.pop() {
        let maze_neighbors = get_maze_neighbors(x, y, &visited);

        if let Some(&(nx, ny, direction)) = maze_neighbors.choose(&mut rng) {
            walls.remove(&(x, y, direction));
            walls.remove(&(nx, ny, direction.opposite()));

            mark_cell_as_maze(x, y, &mut visited, &mut frontier);
        }

        frontier.shuffle(&mut rng);
    }

    let material = materials.add(Color::srgb(1.0, 1.0, 1.0));

    for (x, y, direction) in walls.iter() {
        place_wall(
            &mut commands,
            &horizontal_wall_mesh,
            &vertical_wall_mesh,
            &material,
            *x,
            *y,
            *direction,
        );
    }
}
fn place_wall(
    commands: &mut Commands,
    horizontal_wall_mesh: &Handle<Mesh>,
    vertical_wall_mesh: &Handle<Mesh>,
    matrial: &Handle<ColorMaterial>,
    x: usize,
    y: usize,
    direction: Direction,
) {
    let cell_x_center = -GAME_FIELD_WIDTH / 2.0 + GRID_CELL_SIZE * x as f32 + GRID_CELL_SIZE / 2.0;
    let cell_y_center = -GAME_FIELD_HEIGHT / 2.0 + GRID_CELL_SIZE * y as f32 + GRID_CELL_SIZE / 2.0;

    let wall_pos = match direction {
        Direction::Up => (cell_x_center, cell_y_center + GRID_CELL_SIZE / 2.0),
        Direction::Down => (cell_x_center, cell_y_center - GRID_CELL_SIZE / 2.0),
        Direction::Left => (cell_x_center - GRID_CELL_SIZE / 2.0, cell_y_center),
        Direction::Right => (cell_x_center + GRID_CELL_SIZE / 2.0, cell_y_center),
    };

    if is_within_bounds(x as i32, y as i32) {
        let center = (wall_pos.0, wall_pos.1, 0.).into();
        let (wall_mesh, aabb) = if matches!(direction, Direction::Up | Direction::Down) {
            let aabb = Aabb {
                center,
                half_extents: H_WALL_HALF_SIZE.into(),
            };
            (horizontal_wall_mesh, aabb)
        } else {
            let aabb = Aabb {
                center,
                half_extents: V_WALL_HALF_SIZE.into(),
            };
            (vertical_wall_mesh, aabb)
        };
        println!("wall center {center}");
        //commands.spawn(Text2dBundle {
        //    transform: Transform::from_xyz(center.x, center.y, 1.),
        //    text: Text::from_section(
        //        format!("{}", center),
        //        TextStyle {
        //            color: Color::srgb(230., 100., 100.),
        //            font_size: 10.,
        //            ..default()
        //        },
        //    ),
        //    ..default()
        //});

        commands
            .spawn((
                Mesh2d(wall_mesh.clone()),
                MeshMaterial2d(matrial.clone()),
                RigidBody::Fixed,
            ))
            .insert(Transform::from_xyz(wall_pos.0, wall_pos.1, 0.))
            .insert(Wall {
                wall_type: direction.into(),
            })
            .insert(Static)
            .insert(direction)
            .insert(Collider::cuboid(aabb.half_extents.x, aabb.half_extents.y));
    }
}

fn mark_cell_as_maze(
    x: usize,
    y: usize,
    visited: &mut [Vec<bool>],
    frontier: &mut Vec<(usize, usize)>,
) {
    visited[x][y] = true;

    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let (dx, dy) = direction.offset();
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if is_within_bounds(nx, ny) && !visited[nx as usize][ny as usize] {
            let new_frontier = (nx as usize, ny as usize);
            if !frontier.contains(&new_frontier) {
                frontier.push(new_frontier);
            }
        }
    }
}

fn get_maze_neighbors(x: usize, y: usize, visited: &[Vec<bool>]) -> Vec<(usize, usize, Direction)> {
    let mut neighbors = Vec::new();
    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let (dx, dy) = direction.offset();
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if is_within_bounds(nx, ny) && visited[nx as usize][ny as usize] {
            neighbors.push((nx as usize, ny as usize, direction));
        }
    }
    neighbors
}

fn is_within_bounds(x: i32, y: i32) -> bool {
    x >= 0
        && y >= 0
        && x < GRID_CELL_HORIZONTAL_AMOUNT as i32
        && y < GRID_CELL_VERTICAL_AMOUNT as i32
}
