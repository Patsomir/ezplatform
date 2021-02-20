use ggez::{graphics::Rect, mint::Point2};

pub struct World {
    screen_width: f32,
    screen_height: f32,
    width: f32,
    height: f32,
    distance: f32,
    camera_position: Point2<f32>,
}

impl World {
    pub fn new(screen_width: f32, screen_height: f32, distance: f32) -> Self {
        let mut world = World {
            screen_width,
            screen_height,
            width: 0.0,
            height: 0.0,
            distance,
            camera_position: Point2 { x: 0.0, y: 0.0 },
        };
        world.set_distance(distance);
        world
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
        if self.screen_width > self.screen_height {
            self.height = 2.0 * distance;
            self.width = 2.0 * distance * self.screen_width / self.screen_height;
        } else {
            self.width = 2.0 * distance;
            self.height = 2.0 * distance * self.screen_height / self.screen_width;
        }
    }

    pub fn look_at(&mut self, position: Point2<f32>) {
        self.camera_position = position;
    }

    pub fn set_screen_dims(&mut self, screen_width: f32, screen_height: f32) {
        self.screen_width = screen_width;
        self.screen_height = screen_height;
        self.set_distance(self.distance);
    }

    pub fn world_to_screen_pos(&self, position: Point2<f32>) -> Point2<f32> {
        Point2 {
            x: (0.5 + (position.x - self.camera_position.x) / self.width) * self.screen_width,
            y: (0.5 - (position.y - self.camera_position.y) / self.height) * self.screen_height,
        }
    }

    pub fn world_to_screen_rect(&self, rect: Rect) -> Rect {
        let new_pos = self.world_to_screen_pos(rect.point());
        Rect::new(
            new_pos.x,
            new_pos.y,
            self.screen_width * rect.w / self.width,
            self.screen_height * rect.h / self.height,
        )
    }

    pub fn camera_position(&self) -> Point2<f32> {
        self.camera_position
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn screen_width(&self) -> f32 {
        self.screen_width
    }

    pub fn screen_height(&self) -> f32 {
        self.screen_height
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }
}
