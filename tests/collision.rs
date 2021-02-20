use ezplatform::{collision::*, physics::PhysicsObject};
use ggez::{graphics::Rect, mint::Point2};

#[macro_use]
mod float_asserts;

const DELTA: f32 = 0.00001;

#[test]
fn test_tilemap_collider_template_creation() {
    let collider = TilemapCollider::from_template(&[
        &[false, false, true],
        &[false, true, true],
        &[true, true, true],
    ]);

    let expected = vec![
        vec![true, true, true],
        vec![false, true, true],
        vec![false, false, true],
    ];

    assert_eq!(expected, *collider.tiles_ref());
}

#[test]
fn test_tilemap_collider_get_collision_tiles() {
    let collider = TilemapCollider::from_template(&[
        &[false, false, true],
        &[false, true, true],
        &[true, true, true],
    ]);

    let rect = Rect::new(0.0, 0.0, 0.0, 0.0);
    let expected = vec![Rect::new(0.0, 0.0, 1.0, 1.0)];

    assert_eq!(expected, collider.get_collision_tiles(rect));

    let rect = Rect::new(0.0, 0.0, 2.0, 2.0);
    assert_eq!(3, collider.get_collision_tiles(rect).len());

    let rect = Rect::new(1.0, 1.0, 3.0, 3.0);
    assert_eq!(6, collider.get_collision_tiles(rect).len());

    let rect = Rect::new(1.0, 1.0, 3.0, 0.5);
    assert_eq!(2, collider.get_collision_tiles(rect).len());

    let rect = Rect::new(0.0, 1.0, 0.5, 0.5);
    assert_eq!(0, collider.get_collision_tiles(rect).len());

    let rect = Rect::new(0.0, 2.0, 2.0, 0.5);
    assert_eq!(0, collider.get_collision_tiles(rect).len());

    let rect = Rect::new(-1.0, 1.0, 0.5, 3.0);
    assert_eq!(0, collider.get_collision_tiles(rect).len());

    let rect = Rect::new(3.0, 1.0, 0.5, 3.0);
    assert_eq!(0, collider.get_collision_tiles(rect).len());
}

#[test]
fn test_tilemap_collider_get_collision_lines() {
    let collider = TilemapCollider::from_template(&[
        &[false, false, true],
        &[false, true, true],
        &[true, true, true],
    ]);

    let rect = Rect::new(1.0, 1.0, 3.0, 0.5);
    let expected = vec![Rect::new(1.5, 1.0, 2.0, 1.0)];

    assert_eq!(expected, collider.get_collision_lines(rect));

    let rect = Rect::new(0.0, 0.0, 2.0, 2.0);
    assert_eq!(2, collider.get_collision_lines(rect).len());

    let rect = Rect::new(1.0, 1.0, 3.0, 3.0);
    assert_eq!(3, collider.get_collision_lines(rect).len());

    let rect = Rect::new(0.0, 1.0, 0.5, 0.5);
    assert_eq!(0, collider.get_collision_lines(rect).len());

    let rect = Rect::new(-1.0, 1.0, 0.5, 3.0);
    assert_eq!(0, collider.get_collision_lines(rect).len());

    let rect = Rect::new(0.0, 1.0, 3.0, 0.5);
    assert_eq!(1, collider.get_collision_lines(rect).len());

    let rect = Rect::new(0.0, 0.5, 3.0, 1.0);
    assert_eq!(2, collider.get_collision_lines(rect).len());
}

#[test]
fn test_tilemap_collider_check_collision() {
    let collider = TilemapCollider::from_template(&[
        &[false, false, true],
        &[false, true, true],
        &[true, true, true],
    ]);

    let point = Point2 { x: 0.0, y: 0.0 };
    assert!(collider.check_collision(point));

    let point = Point2 { x: 1.0, y: 0.0 };
    assert!(collider.check_collision(point));

    let point = Point2 { x: 1.0, y: 1.0 };
    assert!(collider.check_collision(point));

    let point = Point2 { x: 0.0, y: 1.0 };
    assert!(!collider.check_collision(point));

    let point = Point2 { x: -1.0, y: 0.0 };
    assert!(!collider.check_collision(point));

    let point = Point2 { x: 3.0, y: 0.0 };
    assert!(!collider.check_collision(point));
}

#[test]
fn test_new_dynamic_collider() {
    let collider = DynamicCollider::from_rect(Rect::new(1.0, 1.0, 2.0, 3.0), 1.0);

    assert_eq_point!(Point2 { x: 1.0, y: 1.0 }, collider.position(), DELTA);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, collider.velocity(), DELTA);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, collider.acceleration(), DELTA);
    assert_eq_point!(Point2 { x: 0.0, y: 0.0 }, collider.force(), DELTA);
    assert_eq_float!(1.0, collider.mass(), DELTA);
    assert_eq_float!(2.0, collider.width(), DELTA);
    assert_eq_float!(3.0, collider.height(), DELTA);
}

#[test]
fn test_dynamic_collider_resolve_collision_no_contact() {
    let initial_rect = Rect::new(1.0, 1.0, 2.0, 3.0);
    let mut collider = DynamicCollider::from_rect(initial_rect, 1.0);

    let obstacle = Rect::new(-1.0, 1.0, 1.0, 1.0);
    collider.resolve_collision(&obstacle);
    assert_eq_rect!(initial_rect, collider.rect(), DELTA);

    let obstacle = Rect::new(3.0, 1.0, 1.0, 1.0);
    collider.resolve_collision(&obstacle);
    assert_eq_rect!(initial_rect, collider.rect(), DELTA);

    let obstacle = Rect::new(1.0, -1.0, 1.0, 1.0);
    collider.resolve_collision(&obstacle);
    assert_eq_rect!(initial_rect, collider.rect(), DELTA);

    let obstacle = Rect::new(1.0, 3.0, 1.0, 1.0);
    collider.resolve_collision(&obstacle);
    assert_eq_rect!(initial_rect, collider.rect(), DELTA);
}

#[test]
fn test_dynamic_collider_resolve_collision_with_contact() {
    let mut collider = DynamicCollider::from_rect(Rect::new(1.0, 1.0, 2.0, 3.0), 1.0);

    let obstacle = Rect::new(0.0, 1.0, 1.0, 1.0);
    let expected = Rect::new(1.5, 1.0, 2.0, 3.0);
    collider.resolve_collision(&obstacle);
    assert_eq_rect!(expected, collider.rect(), DELTA);

    let obstacle = Rect::new(2.0, 1.0, 1.0, 1.0);
    let expected = Rect::new(0.5, 1.0, 2.0, 3.0);
    collider.resolve_collision(&obstacle);
    assert_eq_rect!(expected, collider.rect(), DELTA);

    let obstacle = Rect::new(0.0, 0.0, 1.0, 1.0);
    let expected = Rect::new(0.5, 2.0, 2.0, 3.0);
    collider.resolve_collision(&obstacle);
    assert_eq_rect!(expected, collider.rect(), DELTA);

    let obstacle = Rect::new(0.0, 3.0, 1.0, 1.0);
    let expected = Rect::new(0.5, 1.0, 2.0, 3.0);
    collider.resolve_collision(&obstacle);
    assert_eq_rect!(expected, collider.rect(), DELTA);
}
