use std::time::Duration;

use ggez::mint::{Point2, Vector2};

pub const ZERO_VECTOR: Vector2<f32> = Vector2 { x: 0.0, y: 0.0 };
pub struct PhysicsObject {
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
    pub force: Vector2<f32>,
}

impl PhysicsObject {
    pub fn new(position: Point2<f32>, mass: f32) -> Self {
        PhysicsObject {
            position,
            mass,
            velocity: ZERO_VECTOR,
            force: ZERO_VECTOR,
        }
    }

    pub fn acceleration(&self) -> Vector2<f32> {
        Vector2 {
            x: self.force.x / self.mass,
            y: self.force.y / self.mass,
        }
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        self.force.x += force.x;
        self.force.y += force.y;
    }

    pub fn set_force(&mut self, force: Vector2<f32>) {
        self.force = force;
    }

    pub fn update(&mut self, deltatime: Duration) {
        let acceleration = self.acceleration();
        let seconds = deltatime.as_secs_f32();

        let old_velocity_x = self.velocity.x;
        let old_velocity_y = self.velocity.y;
        let deltavelocity_x = acceleration.x * seconds;
        let deltavelocity_y = acceleration.y * seconds;

        self.velocity.x += deltavelocity_x;
        self.velocity.y += deltavelocity_y;

        self.position.x += old_velocity_x + deltavelocity_x * seconds / 2.0;
        self.position.y += old_velocity_y + deltavelocity_x * seconds / 2.0;
    }
}
