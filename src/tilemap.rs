use ggez::{graphics::Rect, mint::Point2};

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

    fn rect_overlap(&self, rect: Rect) -> TilemapSegment {
        let local_x = rect.x / self.tile_width() + self.origin().x as f32;
        let local_y = rect.y / self.tile_height() + self.origin().y as f32;
        let half_local_w = 0.5 * rect.w / self.tile_width();
        let half_local_h = 0.5 * rect.h / self.tile_height();

        TilemapSegment {
            left_bound: (local_x - half_local_w).round() as i32,
            right_bound: (local_x + half_local_w).round() as i32,
            bottom_bound: (local_y - half_local_h).round() as i32,
            top_bound: (local_y + half_local_h).round() as i32,
        }
    }

    fn point_overlap(&self, point: Point2<f32>) -> Point2<i32> {
        let tilemap_point = self.world_to_tilemap(point);
        Point2 {
            x: tilemap_point.x.round() as i32,
            y: tilemap_point.y.round() as i32,
        }
    }

    fn tile_to_world(&self, point: Point2<i32>) -> Rect {
        let Point2 { x, y } = self.tilemap_to_world(Point2 {
            x: point.x as f32,
            y: point.y as f32,
        });
        Rect::new(x, y, self.tile_width(), self.tile_height())
    }

    fn segment_to_world(&self, segment: &TilemapSegment) -> Rect {
        let Point2 { x, y } = self.tilemap_to_world(Point2 {
            x: (segment.left_bound as f32 + segment.right_bound as f32) / 2.0,
            y: (segment.bottom_bound as f32 + segment.top_bound as f32) / 2.0,
        });
        Rect::new(
            x,
            y,
            self.tile_width() * (segment.right_bound as f32 - segment.left_bound as f32 + 1.0),
            self.tile_height() * (segment.top_bound as f32 - segment.bottom_bound as f32 + 1.0),
        )
    }
}

pub struct TilemapSegment {
    pub left_bound: i32,
    pub right_bound: i32,
    pub bottom_bound: i32,
    pub top_bound: i32,
}
