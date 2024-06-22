use std::time::Duration;

use glam::{U16Vec2, Vec2};

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;

const WORLD_MAP: [[u8; MAP_WIDTH]; MAP_HEIGHT] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
];

pub struct RayCasting {
    width: usize,
    height: usize,
    world: Vec<Vec<u8>>,
    pos: Vec2,
    dir: Vec2,
    plane: Vec2,
}

impl Default for RayCasting {
    fn default() -> Self {
        Self {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            world: WORLD_MAP.to_vec().iter().map(|v| v.to_vec()).collect(),
            pos: Vec2::new(MAP_WIDTH as f32 / 2.0, MAP_HEIGHT as f32 / 2.0),
            dir: Vec2::new(-1.0, 0.0),
            plane: Vec2::new(0.0, 0.66),
        }
    }
}

impl RayCasting {
    pub fn lines(&mut self, _: Duration, w: u16, h: u16) -> Vec<(U16Vec2, U16Vec2, u8)> {
        let mut lines = Vec::<(U16Vec2, U16Vec2, u8)>::new();
        for x in 0..w {
            let ray_dir = self.dir + self.plane * Vec2::splat((2 * x as i64 / w as i64 - 1) as f32);

            let mut map_x = self.pos.x as usize;
            let mut map_y = self.pos.y as usize;

            let mut side_dist = Vec2::ZERO;

            let delta_dist_x = if ray_dir.x == 0.0 {
                1e30
            } else {
                (1.0 / ray_dir.x).abs()
            };
            let delta_dist_y = if ray_dir.y == 0.0 {
                1e30
            } else {
                (1.0 / ray_dir.y).abs()
            };

            let step_x: i8;
            let step_y: i8;

            let mut hit: usize = 0;
            let mut side: usize = 0;

            if ray_dir.x < 0.0 {
                step_x = -1;
                side_dist.x = (self.pos.x - map_x as f32) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist.x = (map_x as f32 + 1.0 - self.pos.x) * delta_dist_x;
            }

            if ray_dir.y < 0.0 {
                step_y = -1;
                side_dist.y = (self.pos.y - map_y as f32) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist.y = (map_y as f32 + 1.0 - self.pos.y) * delta_dist_y;
            }

            while hit == 0 {
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist_x;
                    map_x = (map_x as i8 + step_x) as usize;
                    side = 0;
                } else {
                    side_dist.y += delta_dist_x;
                    map_y = (map_y as i8 + step_y) as usize;
                    side = 1;
                }

                if self.world[map_y][map_x] > 0 {
                    hit = 1;
                }
            }

            let perp_wall_dist = if side == 0 {
                side_dist.x - delta_dist_x
            } else {
                side_dist.y - delta_dist_y
            };

            let line_height = h as f32 / perp_wall_dist;
            let mut draw_start = -line_height / 2.0 + h as f32 / 2.0;
            if draw_start < 0.0 {
                draw_start = 0.0;
            }
            let mut draw_end = line_height / 2.0 + h as f32 / 2.0;
            if draw_end >= h as f32 {
                draw_end = h as f32 - 1.0;
            }

            lines.push((
                U16Vec2::new(x, draw_start as u16),
                U16Vec2::new(x, draw_end as u16),
                self.world[map_y][map_x],
            ));
        }
        lines
    }
}
