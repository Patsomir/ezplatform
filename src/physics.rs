use std::time::Duration;

use ggez::mint::{Point2, Vector2};

pub const ZERO_VECTOR: Vector2<f32> = Vector2 { x: 0.0, y: 0.0 };

pub trait PhysicsObject {
    fn mass_mut(&mut self) -> &mut f32;

    fn force_mut(&mut self) -> &mut Vector2<f32>;

    fn velocity_mut(&mut self) -> &mut Vector2<f32>;

    fn acceleration_mut(&mut self) -> &mut Vector2<f32>;

    fn position_mut(&mut self) -> &mut Point2<f32>;

    fn mass(&self) -> f32;

    fn force(&self) -> Vector2<f32>;

    fn acceleration(&self) -> Vector2<f32>;

    fn velocity(&self) -> Vector2<f32>;

    fn position(&self) -> Point2<f32>;

    fn apply_force(&mut self, force: Vector2<f32>) {
        self.force_mut().x += force.x;
        self.force_mut().y += force.y;
    }

    fn set_force(&mut self, force: Vector2<f32>) {
        *self.force_mut() = force;
    }

    fn apply_velocity(&mut self, velocity: Vector2<f32>) {
        self.velocity_mut().x += velocity.x;
        self.velocity_mut().y += velocity.y;
    }

    fn set_velocity(&mut self, velocity: Vector2<f32>) {
        *self.velocity_mut() = velocity;
    }

    fn translate(&mut self, direction: Vector2<f32>) {
        self.position_mut().x += direction.x;
        self.position_mut().y += direction.y;
    }

    fn set_position(&mut self, position: Point2<f32>) {
        *self.position_mut() = position;
    }

    fn update(&mut self, deltatime: Duration) {
        self.acceleration_mut().x = self.force().x / self.mass();
        self.acceleration_mut().y = self.force().y / self.mass();

        let acceleration = self.acceleration();
        let seconds = deltatime.as_secs_f32();

        let old_velocity = self.velocity();
        let deltavelocity_x = acceleration.x * seconds;
        let deltavelocity_y = acceleration.y * seconds;

        self.velocity_mut().x += deltavelocity_x;
        self.velocity_mut().y += deltavelocity_y;

        self.position_mut().x += old_velocity.x + deltavelocity_x * seconds / 2.0;
        self.position_mut().y += old_velocity.y + deltavelocity_x * seconds / 2.0;
    }
}

pub struct PhysicsPoint {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    force: Vector2<f32>,
    mass: f32,
}

impl PhysicsObject for PhysicsPoint {
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

impl PhysicsPoint {
    pub fn new(position: Point2<f32>, mass: f32) -> Self {
        Self {
            position: position,
            velocity: ZERO_VECTOR,
            acceleration: ZERO_VECTOR,
            force: ZERO_VECTOR,
            mass,
        }
    }
}
