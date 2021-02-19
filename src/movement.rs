use std::time::Duration;

use ggez::{graphics::Rect, mint::{Point2, Vector2}};

use crate::{collision::DynamicCollider, physics::PhysicsObject};

pub struct MovementController {
    body: DynamicCollider,
    move_force: f32,
    jump_impulse: f32,
    max_speed: f32,
    move_speed_decay: f32,
    gravity_acceleration: f32,
    horizontal_force: f32,
    ground_check_offsets: Vec<Vector2<f32>>
}

impl MovementController {
    pub fn from_components(
        body: DynamicCollider,
        move_force: f32,
        jump_impulse: f32,
        max_speed: f32,
        move_speed_decay: f32,
        gravity_acceleration: f32,
        ground_check_offsets: &[Vector2<f32>],
    ) -> Self {
        MovementController {
            body,
            move_force,
            jump_impulse,
            max_speed,
            move_speed_decay,
            gravity_acceleration,
            horizontal_force: 0.0,
            ground_check_offsets: ground_check_offsets.into(),
        }
    }

    pub fn move_left(&mut self) {
        self.horizontal_force = -self.move_force;
    }

    pub fn move_right(&mut self) {
        self.horizontal_force = self.move_force;
    }

    pub fn stop(&mut self) {
        self.horizontal_force = 0.0;
    }

    pub fn jump(&mut self) {
        self.body.set_velocity(Vector2 {
            x: self.body.velocity().x,
            y: self.jump_impulse,
        });
    }

    pub fn collider(&self) -> &DynamicCollider {
        &self.body
    }

    pub fn collider_mut(&mut self) -> &mut DynamicCollider {
        &mut self.body
    }

    pub fn rect(&self) -> Rect {
        self.body.rect()
    }

    pub fn ground_check_points(&self) -> Vec<Point2<f32>> {
        let rect = self.body.rect();
        return self.ground_check_offsets.iter().map(|offset| Point2 {
            x: rect.x + 0.5 * offset.x * rect.w,
            y: rect.y + 0.5 * offset.y * rect.h,
        }).collect();
    }

    pub fn update(&mut self, deltatime: Duration) {
        self.body.set_force(Vector2 {
            x: self.horizontal_force,
            y: -self.gravity_acceleration * self.body.mass(),
        });
        self.body.update(deltatime);

        if self.body.velocity().x > self.max_speed {
            self.body.velocity_mut().x = self.max_speed;
        } else if self.body.velocity().x < -self.max_speed {
            self.body.velocity_mut().x = -self.max_speed;
        }

        let deltaseconds = deltatime.as_secs_f32();
        if self.horizontal_force.abs() < 0.01 || self.horizontal_force * self.body.velocity().x < 0.0
        {
            if self.body.velocity().x.abs() < 0.01 {
                self.body.velocity_mut().x = 0.0;
            } else if self.body.velocity().x > 0.0 {
                self.body.velocity_mut().x -= self.move_speed_decay * deltaseconds;
            } else {
                self.body.velocity_mut().x += self.move_speed_decay * deltaseconds;
            }
        }
    }
}
