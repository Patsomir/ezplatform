use ggez::mint::Point2;

pub trait Tilemap {
    fn origin(&self) -> Point2<i32>;

    fn tile_width(&self) -> f32;

    fn tile_height(&self) -> f32;

    fn tilemap_to_world(&self, point: Point2<f32>) -> Point2<f32> {
        Point2 {
            x: (point.x - self.origin().x as f32) * self.tile_width(),
            y: (point.y - self.origin().y as f32) * self.tile_height(),
        }
    }

    fn world_to_tilemap(&self, point: Point2<f32>) -> Point2<f32> {
        Point2 {
            x: point.x / self.tile_width() + self.origin().x as f32,
            y: point.y / self.tile_height() + self.origin().y as f32,
        }
    }
}
