use std::time::Duration;

use ezplatform::physics::*;
use ggez::mint::{Point2, Vector2};

#[macro_use]
mod float_asserts;

const DELTA: f32 = 0.00001;

#[test]
fn test_new_physics_point() {
    let point = PhysicsPoint::new(Point2 { x: 1.0, y: 1.0 }, 1.0);

    assert_eq_point!(Point2 { x: 1.0, y: 1.0 }, point.position(), DELTA);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.velocity(), DELTA);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.acceleration(), DELTA);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.force(), DELTA);
    assert_eq_float!(1.0, point.mass(), DELTA);
}

#[test]
fn test_translate() {
    let mut point = PhysicsPoint::new(Point2 { x: 0.0, y: 0.0 }, 1.0);

    let vector = Vector2 { x: 0.0, y: 0.0 };
    point.translate(vector);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.position(), DELTA);

    let vector = Vector2 { x: 1.0, y: 0.0 };
    point.translate(vector);
    assert_eq_point!(Point2 { x: 1.0, y: 0.0 }, point.position(), DELTA);

    let vector = Vector2 { x: 0.0, y: -2.0 };
    point.translate(vector);
    assert_eq_point!(Point2 { x: 1.0, y: -2.0 }, point.position(), DELTA);

    let vector = Vector2 { x: -1.0, y: 2.0 };
    point.translate(vector);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.position(), DELTA);
}

#[test]
fn test_apply_velocity() {
    let mut point = PhysicsPoint::new(Point2 { x: 0.0, y: 0.0 }, 1.0);

    let vector = Vector2 { x: 0.0, y: 0.0 };
    point.apply_velocity(vector);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.velocity(), DELTA);

    let vector = Vector2 { x: 1.0, y: 0.0 };
    point.apply_velocity(vector);
    assert_eq_point!(Point2 { x: 1.0, y: 0.0 }, point.velocity(), DELTA);

    let vector = Vector2 { x: 0.0, y: -2.0 };
    point.apply_velocity(vector);
    assert_eq_point!(Point2 { x: 1.0, y: -2.0 }, point.velocity(), DELTA);

    let vector = Vector2 { x: -1.0, y: 2.0 };
    point.apply_velocity(vector);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.velocity(), DELTA);
}

#[test]
fn test_apply_force() {
    let mut point = PhysicsPoint::new(Point2 { x: 0.0, y: 0.0 }, 2.0);

    let vector = Vector2 { x: 0.0, y: 0.0 };
    point.apply_force(vector);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.force(), DELTA);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.acceleration(), DELTA);

    let vector = Vector2 { x: 1.0, y: 0.0 };
    point.apply_force(vector);
    assert_eq_point!(Point2 { x: 1.0, y: 0.0 }, point.force(), DELTA);
    assert_eq_point!(Point2 { x: 0.5, y: 0.0 }, point.acceleration(), DELTA);

    let vector = Vector2 { x: 0.0, y: -2.0 };
    point.apply_force(vector);
    assert_eq_point!(Point2 { x: 1.0, y: -2.0 }, point.force(), DELTA);
    assert_eq_point!(Point2 { x: 0.5, y: -1.0 }, point.acceleration(), DELTA);

    let vector = Vector2 { x: -1.0, y: 2.0 };
    point.apply_force(vector);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.force(), DELTA);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.acceleration(), DELTA);
}

#[test]
fn test_still_object_update() {
    let mut point = PhysicsPoint::new(Point2 { x: 0.0, y: 0.0 }, 1.0);

    point.update(Duration::from_secs_f32(1.0));
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.position(), DELTA);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.velocity(), DELTA);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.acceleration(), DELTA);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, point.force(), DELTA);
    assert_eq_float!(1.0, point.mass(), DELTA);
}

#[test]
fn test_monotonous_movement_update() {
    let mut point = PhysicsPoint::new(Point2 { x: 0.0, y: 0.0 }, 1.0);
    point.set_velocity(Vector2 { x: 1.0, y: 1.0 });

    point.update(Duration::from_secs_f32(1.0));
    assert_eq_point!(Point2 { x: 1.0, y: 1.0 }, point.position(), DELTA);

    point.update(Duration::from_secs_f32(0.5));
    assert_eq_point!(Point2 { x: 1.5, y: 1.5 }, point.position(), DELTA);
}

#[test]
fn test_accelerating_movement_update() {
    let mut point = PhysicsPoint::new(Point2 { x: 0.0, y: 0.0 }, 2.0);
    point.set_force(Vector2 { x: 1.0, y: 1.0 });

    point.update(Duration::from_secs_f32(1.0));
    assert_eq_point!(Point2 { x: 0.5, y: 0.5 }, point.velocity(), DELTA);
    assert_eq_point!(Point2 { x: 0.25, y: 0.25 }, point.position(), DELTA);

    point.update(Duration::from_secs_f32(0.5));
    assert_eq_point!(Point2 { x: 0.75, y: 0.75 }, point.velocity(), DELTA);
    assert_eq_point!(
        Point2 {
            x: 0.5625,
            y: 0.5625
        },
        point.position(),
        DELTA
    );
}
