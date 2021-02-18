use ggez::{
    graphics::{DrawParam, Drawable, Image, Rect},
    mint::Point2,
    Context, GameResult,
};

use crate::world::World;

pub trait WorldDrawable {
    fn draw_in_world(&self, ctx: &mut Context, world: &World, rect: Rect) -> GameResult;
}

impl<T: Drawable> WorldDrawable for T {
    fn draw_in_world(&self, ctx: &mut Context, world: &World, rect: Rect) -> GameResult {
        let target_rect = world.world_to_screen_rect(rect);
        if let Some(initial_rect) = self.dimensions(ctx) {
            return self.draw(
                ctx,
                DrawParam::default()
                    .offset(Point2 { x: 0.5, y: 0.5 })
                    .dest(target_rect.point())
                    .scale(Point2 {
                        x: target_rect.w / initial_rect.w,
                        y: target_rect.h / initial_rect.h,
                    }),
            );
        };
        Ok(())
    }
}

pub struct SpriteSheet {
    spritesheet: Image,
    active_sprite_index: u32,
    active_sprite_rect: Rect,
    rows: u32,
    cols: u32,
    total_sprites: u32,
    sprite_width_proportion: f32,
    sprite_height_proportion: f32,
}

impl SpriteSheet {
    pub fn new(spritesheet: Image, rows: u32, cols: u32, total_sprites: u32) -> Self {
        let sprite_width_proportion = 1.0 / cols as f32;
        let sprite_height_proportion = 1.0 / rows as f32;
        let active_sprite_rect =
            Rect::new(0.0, 0.0, sprite_width_proportion, sprite_height_proportion);
        SpriteSheet {
            spritesheet,
            rows,
            cols,
            total_sprites,
            active_sprite_index: 0,
            active_sprite_rect,
            sprite_width_proportion,
            sprite_height_proportion,
        }
    }

    pub fn set_active(&mut self, sprite_index: u32) {
        self.active_sprite_index = sprite_index;
        let row_index = (sprite_index / self.cols) as f32;
        let col_index = (sprite_index % self.cols) as f32;
        self.active_sprite_rect = Rect::new(
            col_index * self.sprite_width_proportion,
            row_index * self.sprite_height_proportion,
            self.sprite_width_proportion,
            self.sprite_height_proportion,
        );
    }

    pub fn set_next(&mut self) {
        self.set_active((self.active_sprite_index + 1) % self.total_sprites);
    }

    pub fn set_previous(&mut self) {
        self.active_sprite_index =
            (self.active_sprite_index + self.total_sprites - 1) % self.total_sprites;
        self.set_active((self.active_sprite_index - 1) % self.total_sprites);
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn cols(&self) -> u32 {
        self.cols
    }
}

impl Drawable for SpriteSheet {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.spritesheet
            .draw(ctx, param.src(self.active_sprite_rect))
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        let mut rect = self.spritesheet.dimensions();
        rect.w /= self.cols as f32;
        rect.h /= self.rows as f32;
        Some(rect)
    }

    fn set_blend_mode(&mut self, mode: Option<ggez::graphics::BlendMode>) {
        self.spritesheet.set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<ggez::graphics::BlendMode> {
        self.spritesheet.blend_mode()
    }
}

struct TilemapRenderer {
    
}
