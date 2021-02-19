use ggez::{Context, GameResult, graphics::{self, DrawParam, Rect}, mint::Point2};

use crate::{collision::TilemapCollider, rendering::WorldDrawable, tilemap::Tilemap, world::World};

impl WorldDrawable for TilemapCollider {
    fn draw_in_world(&self, ctx: &mut Context, world: &World, _: Rect) -> GameResult {
        let tiles = self.tiles_ref();
        for row in 0..tiles.len() {
            for col in 0..tiles[row].len() {
                if tiles[row][col] {
                    let w = self.tile_width();
                    let h = self.tile_height();
                    let Point2 { x, y } = self.tilemap_to_world(Point2 {
                        x: col as f32,
                        y: row as f32,
                    });
                    draw_rect_in_world(ctx, Rect::new(x, y, w, h), world)?
                }
            }
        }
        Ok(())
    }
}

pub fn draw_rect_in_world(ctx: &mut Context, rect: Rect, world: &World) -> GameResult {
    let mut rect = rect;
    rect.translate(Point2 {
        x: -rect.w / 2.0,
        y: rect.h / 2.0
    });
    let mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::stroke(1.0),
        world.world_to_screen_rect(rect),
        graphics::WHITE,
    )?;
    graphics::draw(ctx, &mesh, DrawParam::default())
}
