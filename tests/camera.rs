use std::time::Duration;

use ::ezplatform::camera::*;
use ggez::mint::Point2;

#[macro_use]
mod float_asserts;

const DELTA: f32 = 0.00001;

#[test]
fn test_new_sharp_camera() {
    let camera = SharpCamera::new(Point2 { x: 2.0, y: 2.0 });
    assert_eq_point!(Point2 { x: 2.0, y: 2.0 }, camera.position(), DELTA);
}

#[test]
fn test_sharp_camera_follow() {
    let mut camera = SharpCamera::new(Point2 { x: 0.0, y: 0.0 });
    let placeholder = Duration::default();

    let destination = Point2 { x: 2.0, y: 2.0 };
    camera.set_destination(destination);
    camera.set_follow_direction(FollowDirection::Both);
    camera.update(placeholder);
    assert_eq_point!(destination, camera.position(), DELTA);

    let destination = Point2 { x: 3.0, y: 3.0 };
    camera.set_destination(destination);
    camera.set_follow_direction(FollowDirection::Horizontal);
    camera.update(placeholder);
    assert_eq_point!(Point2 { x: 3.0, y: 2.0 }, camera.position(), DELTA);

    let destination = Point2 { x: 0.0, y: 0.0 };
    camera.set_destination(destination);
    camera.set_follow_direction(FollowDirection::Vertical);
    camera.update(placeholder);
    assert_eq_point!(Point2 { x: 3.0, y: 0.0 }, camera.position(), DELTA);
}

#[test]
fn test_new_smooth_camera() {
    let camera = SmoothCamera::new(Point2 { x: 2.0, y: 2.0 }, 1.0);
    assert_eq_point!(Point2 { x: 2.0, y: 2.0 }, camera.position(), DELTA);
}

#[test]
fn test_smooth_camera_follow() {
    let mut camera = SmoothCamera::new(Point2 { x: 0.0, y: 0.0 }, 0.5);

    let destination = Point2 { x: 2.0, y: 2.0 };
    camera.set_destination(destination);
    camera.set_follow_direction(FollowDirection::Both);
    camera.update(Duration::from_secs_f32(1.0));
    assert_eq_point!(Point2 { x: 1.0, y: 1.0 }, camera.position(), DELTA);
    camera.update(Duration::from_secs_f32(0.5));
    assert_eq_point!(Point2 { x: 1.25, y: 1.25 }, camera.position(), DELTA);

    let mut camera = SmoothCamera::new(Point2 { x: 0.0, y: 0.0 }, 0.5);

    let destination = Point2 { x: 2.0, y: 2.0 };
    camera.set_destination(destination);
    camera.set_follow_direction(FollowDirection::Horizontal);
    camera.update(Duration::from_secs_f32(1.0));
    assert_eq_point!(Point2 { x: 1.0, y: 0.0 }, camera.position(), DELTA);
    camera.update(Duration::from_secs_f32(0.5));
    assert_eq_point!(Point2 { x: 1.25, y: 0.0 }, camera.position(), DELTA);

    let mut camera = SmoothCamera::new(Point2 { x: 0.0, y: 0.0 }, 0.5);

    let destination = Point2 { x: 2.0, y: 2.0 };
    camera.set_destination(destination);
    camera.set_follow_direction(FollowDirection::Vertical);
    camera.update(Duration::from_secs_f32(1.0));
    assert_eq_point!(Point2 { x: 0.0, y: 1.0 }, camera.position(), DELTA);
    camera.update(Duration::from_secs_f32(0.5));
    assert_eq_point!(Point2 { x: 0.0, y: 1.25 }, camera.position(), DELTA);
}
