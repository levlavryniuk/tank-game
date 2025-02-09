pub const GAME_FIELD_WIDTH: f32 = 720.0;
pub const GAME_FIELD_HEIGHT: f32 = 720.0;
pub const GRID_CELL_SIZE: f32 = 120.0;
pub const GRID_CELL_HORIZONTAL_AMOUNT: u32 = (GAME_FIELD_WIDTH / GRID_CELL_SIZE) as u32;
pub const GRID_CELL_VERTICAL_AMOUNT: u32 = (GAME_FIELD_HEIGHT / GRID_CELL_SIZE) as u32;

pub const SERVER_ADDR: &str = "127.0.0.1"; // Ensure this is a valid IPv4 address
pub const BULLET_SPEED: f32 = 250.;
pub const BULLET_SIZE: f32 = 5.;
pub const BULLET_HALF_EXTENTS: (f32, f32, f32) = (2.5, 2.5, 0.);
pub const BULLET_OFFSET: f32 = 20.;
