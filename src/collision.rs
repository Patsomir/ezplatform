use crate::tilemap::Tilemap;
use crate::{
    physics::{PhysicsObject, ZERO_VECTOR},
    rendering::TilemapRenderer,
    tilemap::TilemapSegment,
};
use ggez::{
    graphics::Rect,
    mint::{Point2, Vector2},
};

pub struct TilemapCollider {
    tile_width: f32,
    tile_height: f32,
    tiles: Vec<Vec<bool>>,
    origin: Point2<i32>,
}

impl Tilemap for TilemapCollider {
    fn origin(&self) -> Point2<i32> {
        self.origin
    }

    fn tile_width(&self) -> f32 {
        self.tile_width
    }

    fn tile_height(&self) -> f32 {
        self.tile_height
    }
}

impl TilemapCollider {
    pub fn from_components(
        template: &[&[bool]],
        tile_width: f32,
        tile_height: f32,
        origin: Point2<i32>,
    ) -> Self {
        let tiles: Vec<Vec<bool>> = template.iter().map(|arr| Vec::from(*arr)).rev().collect();
        TilemapCollider {
            tile_width,
            tile_height,
            tiles,
            origin,
        }
    }

    pub fn from_template(template: &[&[bool]]) -> Self {
        Self::from_components(template, 1.0, 1.0, Point2 { x: 0, y: 0 })
    }

    pub fn set_origin(&mut self, origin: Point2<i32>) {
        self.origin = origin;
    }

    pub fn get_collision_tiles(&self, rect: Rect) -> Vec<Rect> {
        let TilemapSegment {
            left_bound,
            right_bound,
            bottom_bound,
            top_bound,
        } = self.rect_overlap(rect);

        let mut result: Vec<Rect> = Vec::new();
        for row in bottom_bound..(top_bound + 1) {
            if row < 0 {
                continue;
            }
            if let Some(row_vec) = self.tiles.get(row as usize) {
                for col in left_bound..(right_bound + 1) {
                    if col < 0 {
                        continue;
                    }
                    if let Some(tile) = row_vec.get(col as usize) {
                        if *tile {
                            result.push(self.tile_to_world(Point2 { x: col, y: row }));
                        }
                    }
                }
            }
        }
        result
    }

    fn get_row_rects(&self, row: i32, left_bound: i32, right_bound: i32) -> Vec<Rect> {
        let mut result: Vec<Rect> = Vec::new();
        let row_vec = self.tiles.get(row as usize);
        if row < 0 || row_vec.is_none() {
            return result;
        }
        let row_vec = row_vec.unwrap();
        let mut segment_left: Option<i32> = None;
        let mut segment_right: i32 = -1;
        for col in left_bound..(right_bound + 1) {
            if col < 0 {
                continue;
            }
            let tile = match row_vec.get(col as usize) {
                None => false,
                Some(tile_bool) => *tile_bool,
            };

            if tile {
                if segment_left.is_none() {
                    segment_left = Some(col);
                }
                segment_right = col;
            } else {
                if let Some(segment_left) = segment_left {
                    result.push(self.segment_to_world(&TilemapSegment {
                        left_bound: segment_left,
                        right_bound: segment_right,
                        bottom_bound: row,
                        top_bound: row,
                    }));
                };
            }
        }
        if let Some(segment_left) = segment_left {
            result.push(self.segment_to_world(&TilemapSegment {
                left_bound: segment_left,
                right_bound: segment_right,
                bottom_bound: row,
                top_bound: row,
            }));
        };
        result
    }

    pub fn get_collision_lines(&self, rect: Rect) -> Vec<Rect> {
        let TilemapSegment {
            left_bound,
            right_bound,
            bottom_bound,
            top_bound,
        } = self.rect_overlap(rect);

        let mut result: Vec<Rect> = Vec::new();
        for row in bottom_bound..(top_bound + 1) {
            result.append(&mut self.get_row_rects(row, left_bound, right_bound));
        }
        result
    }

    pub fn tiles_ref(&self) -> &Vec<Vec<bool>> {
        &self.tiles
    }

    pub fn check_collision(&self, point: Point2<f32>) -> bool {
        let tile = self.point_overlap(point);
        if tile.x < 0 || tile.y < 0 {
            return false;
        }
        if let Some(row_vec) = self.tiles.get(tile.y as usize) {
            if let Some(tile_bool) = row_vec.get(tile.x as usize) {
                return *tile_bool;
            }
        }
        false
    }
}

impl From<&TilemapRenderer> for TilemapCollider {
    fn from(tilemap: &TilemapRenderer) -> Self {
        let tiles: Vec<_> = tilemap
            .tiles()
            .iter()
            .map(|row| row.iter().map(|tile| *tile != 0).collect::<Vec<_>>())
            .collect();
        Self {
            tiles,
            tile_width: tilemap.tile_width(),
            tile_height: tilemap.tile_height(),
            origin: tilemap.origin(),
        }
    }
}

pub struct DynamicCollider {
    position: Point2<f32>,
    width: f32,
    height: f32,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    force: Vector2<f32>,
    mass: f32,
}

impl PhysicsObject for DynamicCollider {
    fn mass_mut(&mut self) -> &mut f32 {
        &mut self.mass
    }

    fn force_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.force
    }

    fn velocity_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.velocity
    }

    fn acceleration_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.acceleration
    }

    fn position_mut(&mut self) -> &mut Point2<f32> {
        &mut self.position
    }

    fn mass(&self) -> f32 {
        self.mass
    }

    fn force(&self) -> Vector2<f32> {
        self.force
    }

    fn acceleration(&self) -> Vector2<f32> {
        self.acceleration
    }

    fn velocity(&self) -> Vector2<f32> {
        self.velocity
    }

    fn position(&self) -> Point2<f32> {
        self.position
    }
}

impl DynamicCollider {
    pub fn from_rect(rect: Rect, mass: f32) -> Self {
        Self {
            position: rect.point(),
            width: rect.w,
            height: rect.h,
            velocity: ZERO_VECTOR,
            acceleration: ZERO_VECTOR,
            force: ZERO_VECTOR,
            mass,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.width, self.height)
    }

    pub fn resolve_collision(&mut self, rect: &Rect) {
        let collision_magnitude_x = 0.5 * (rect.w + self.width) - (self.position.x - rect.x).abs();
        let collision_magnitude_y = 0.5 * (rect.h + self.height) - (self.position.y - rect.y).abs();
        let trigger_magnitude = 0.00001;

        if collision_magnitude_x < collision_magnitude_y {
            if collision_magnitude_x > trigger_magnitude {
                self.velocity.x = 0.0;
                if self.position.x > rect.x {
                    self.position.x = rect.x + 0.5 * rect.w + 0.5 * self.width;
                } else {
                    self.position.x = rect.x - 0.5 * rect.w - 0.5 * self.width;
                }
            }
        } else {
            if collision_magnitude_y > trigger_magnitude {
                self.velocity.y = 0.0;
                if self.position.y > rect.y {
                    self.position.y = rect.y + 0.5 * rect.h + 0.5 * self.height;
                } else {
                    self.position.y = rect.y - 0.5 * rect.h - 0.5 * self.height;
                }
            }
        }
    }

    pub fn resolve_collisions(&mut self, walls: &[Rect]) {
        for rect in walls {
            self.resolve_collision(rect);
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}
