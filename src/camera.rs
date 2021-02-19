use std::time::Duration;

use ggez::mint::Point2;

pub enum FollowDirection {
    Horizontal,
    Vertical,
    Both
}

pub trait Camera {
    fn set_destination(&mut self, point: Point2<f32>);

    fn set_follow_direction(&mut self, follow_direction: FollowDirection);

    fn update(&mut self, deltatime: Duration);

    fn position(&self) -> Point2<f32>;
}
pub struct SharpCamera {
    position: Point2<f32>,
    destination: Point2<f32>,
    follow_direction: FollowDirection, 
}

impl SharpCamera {
    pub fn new(position: Point2<f32>) -> Self {
        Self {
            position,
            destination: position,
            follow_direction: FollowDirection::Both
        }
    }
}

impl Camera for SharpCamera {
    fn set_destination(&mut self, point: Point2<f32>) {
        self.destination = point;
    }

    fn set_follow_direction(&mut self, follow_direction: FollowDirection) {
        self.follow_direction = follow_direction;
    }

    fn update(&mut self, _: Duration) {
        let mut new_position = self.destination;
        match self.follow_direction {
            FollowDirection::Horizontal => new_position.y = self.position.y,
            FollowDirection::Vertical => new_position.x = self.position.x,
            _ => ()
        }
        self.position = new_position;
    }

    fn position(&self) -> Point2<f32> {
        self.position
    }
}

pub struct SmoothCamera {
    position: Point2<f32>,
    destination: Point2<f32>,
    follow_direction: FollowDirection, 
    smoothness: f32
}

impl SmoothCamera {
    pub fn new(position: Point2<f32>, smoothness: f32) -> Self {
        Self {
            position,
            destination: position,
            follow_direction: FollowDirection::Both,
            smoothness
        }
    }
}

impl Camera for SmoothCamera {
    fn set_destination(&mut self, point: Point2<f32>) {
        self.destination = point;
    }

    fn set_follow_direction(&mut self, follow_direction: FollowDirection) {
        self.follow_direction = follow_direction;
    }

    fn update(&mut self, deltatime: Duration) {
        let seconds = deltatime.as_secs_f32();
        let mut new_position = Point2 {
            x: self.position.x + (self.destination.x - self.position.x) * self.smoothness * seconds,
            y: self.position.y + (self.destination.y - self.position.y) * self.smoothness * seconds,
        };
        match self.follow_direction {
            FollowDirection::Horizontal => new_position.y = self.position.y,
            FollowDirection::Vertical => new_position.x = self.position.x,
            _ => ()
        }
        self.position = new_position;
    }

    fn position(&self) -> Point2<f32> {
        self.position
    }
}