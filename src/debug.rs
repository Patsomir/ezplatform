use ggez::{
    graphics::{self, Color, DrawParam, Mesh, Rect},
    mint::Point2,
    Context,
};

use crate::{collision::TilemapCollider, tilemap::Tilemap, world::World};

impl TilemapCollider {
    pub fn draw_in_world(&self, ctx: &mut Context, color: Color, world: &World) {
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
                    draw_rect_in_world(ctx, Rect::new(x, y, w, h), color, world);
                }
            }
        }
    }
}

pub fn draw_rect_in_world(ctx: &mut Context, rect: Rect, color: Color, world: &World) {
    draw_rectangle(
        ctx,
        world
            .new_screen_param(rect.x, rect.y, rect.w, rect.h)
            .color(color),
    );
}

pub fn draw_rectangle(ctx: &mut Context, param: DrawParam) {
    let rect = canonical_rect_mesh(ctx, param.color);
    graphics::draw(ctx, &rect, param).unwrap();
}

pub fn canonical_rect_mesh(ctx: &mut Context, color: Color) -> Mesh {
    graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        Rect::new(0., 0., 1.0, 1.0),
        color,
    )
    .unwrap()
}
