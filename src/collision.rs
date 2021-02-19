use crate::{physics::{PhysicsObject, ZERO_VECTOR}, rendering::TilemapRenderer};
use crate::tilemap::Tilemap;
use ggez::{graphics::Rect, mint::{Point2, Vector2}};

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

    pub fn get_collisions(&self, rect: Rect) -> Vec<Rect> {
        let local_x = rect.x / self.tile_width + self.origin.x as f32;
        let local_y = rect.y / self.tile_height + self.origin.y as f32;
        let half_local_w = 0.5 * rect.w / self.tile_width;
        let half_local_h = 0.5 * rect.h / self.tile_height;

        let left_bound = (local_x - half_local_w).round() as isize;
        let right_bound = (local_x + half_local_w).round() as isize;
        let bottom_bound = (local_y - half_local_h).round() as isize;
        let top_bound = (local_y + half_local_h).round() as isize;

        let mut result: Vec<Rect> = Vec::new();
        for row in bottom_bound..(top_bound + 1) {
            for col in left_bound..(right_bound + 1) {
                if row >= 0 && col >= 0 {
                    if let Some(row_vec) = self.tiles.get(row as usize) {
                        if let Some(tile) = row_vec.get(col as usize) {
                            if *tile {
                                let Point2 { x, y } = self.tilemap_to_world(Point2 {
                                    x: col as f32,
                                    y: row as f32,
                                });
                                result.push(Rect::new(x, y, self.tile_width, self.tile_height));
                            }
                        }
                    }
                }
            }
        }
        result
    }

    pub fn tiles_ref(&self) -> &Vec<Vec<bool>> {
        &self.tiles
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
        Rect::new(
            self.position.x,
            self.position.y,
            self.width,
            self.height
        )
    }

    pub fn resolve_collision(&mut self, rect: &Rect) {
        let self_rect = self.rect();
        if !self_rect.overlaps(&rect) {
            return;
        }
        let collision_direction = Vector2 {
            x: self_rect.x - rect.x,
            y: self_rect.y - rect.y
        };
        self.apply_velocity(collision_direction);
    }

    pub fn resolve_collisions(&mut self, walls: &[Rect]) {
        for rect in walls {
            self.resolve_collision(rect);
        }
    }
}
