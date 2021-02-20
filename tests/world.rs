use ezplatform::world::*;
use ggez::{graphics::Rect, mint::Point2};

const DELTA: f32 = 0.00001;

struct WorldParams {
    screen_width: f32,
    screen_height: f32,
    width: f32,
    height: f32,
    distance: f32,
    camera_position: Point2<f32>,
}

macro_rules! assert_eq_float {
    ($expected:expr , $actual:expr, $delta: expr) => {
        assert!(
            ($expected - $actual).abs() < $delta,
            format!("expected: {}, actual: {}", $expected, $actual)
        );
    };
}

macro_rules! assert_eq_point {
    ($expected:expr , $actual:expr, $delta: expr) => {
        assert_eq_float!($expected.x, $actual.x, $delta);
        assert_eq_float!($expected.y, $actual.y, $delta);
    };
}

macro_rules! assert_eq_rect {
    ($expected:expr , $actual:expr, $delta: expr) => {
        assert_eq_float!($expected.x, $actual.x, $delta);
        assert_eq_float!($expected.y, $actual.y, $delta);
        assert_eq_float!($expected.w, $actual.w, $delta);
        assert_eq_float!($expected.h, $actual.h, $delta);
    };
}

macro_rules! assert_world_params {
    ($world:expr , $expected_params:expr, $delta:expr) => {
        assert_eq_float!($expected_params.screen_width, $world.screen_width(), $delta);
        assert_eq_float!(
            $expected_params.screen_height,
            $world.screen_height(),
            $delta
        );
        assert_eq_float!($expected_params.distance, $world.distance(), $delta);
        assert_eq_float!($expected_params.width, $world.width(), $delta);
        assert_eq_float!($expected_params.height, $world.height(), $delta);
        assert_eq_point!(
            $expected_params.camera_position,
            $world.camera_position(),
            $delta
        );
    };
}

#[test]
fn test_new_horizontal_world_params() {
    let world = World::new(800.0, 400.0, 8.0);

    assert_world_params!(
        &world,
        WorldParams {
            screen_width: 800.0,
            screen_height: 400.0,
            width: 32.0,
            height: 16.0,
            distance: 8.0,
            camera_position: Point2 { x: 0.0, y: 0.0 }
        },
        DELTA
    );
}

#[test]
fn test_new_vertical_world() {
    let world = World::new(400.0, 800.0, 8.0);

    assert_world_params!(
        &world,
        WorldParams {
            screen_width: 400.0,
            screen_height: 800.0,
            width: 16.0,
            height: 32.0,
            distance: 8.0,
            camera_position: Point2 { x: 0.0, y: 0.0 }
        },
        DELTA
    );
}

#[test]
fn test_world_resize() {
    let mut world = World::new(800.0, 400.0, 8.0);
    world.set_screen_dims(400.0, 800.0);

    assert_world_params!(
        &world,
        WorldParams {
            screen_width: 400.0,
            screen_height: 800.0,
            width: 16.0,
            height: 32.0,
            distance: 8.0,
            camera_position: Point2 { x: 0.0, y: 0.0 }
        },
        DELTA
    );

    world.set_screen_dims(1600.0, 800.0);

    assert_world_params!(
        &world,
        WorldParams {
            screen_width: 1600.0,
            screen_height: 800.0,
            width: 32.0,
            height: 16.0,
            distance: 8.0,
            camera_position: Point2 { x: 0.0, y: 0.0 }
        },
        DELTA
    );

    world.set_screen_dims(400.0, 200.0);

    assert_world_params!(
        &world,
        WorldParams {
            screen_width: 400.0,
            screen_height: 200.0,
            width: 32.0,
            height: 16.0,
            distance: 8.0,
            camera_position: Point2 { x: 0.0, y: 0.0 }
        },
        DELTA
    );
}

#[test]
fn test_world_distance_change() {
    let mut world = World::new(800.0, 400.0, 8.0);
    world.set_distance(16.0);

    assert_world_params!(
        &world,
        WorldParams {
            screen_width: 800.0,
            screen_height: 400.0,
            width: 64.0,
            height: 32.0,
            distance: 16.0,
            camera_position: Point2 { x: 0.0, y: 0.0 }
        },
        DELTA
    );

    world.set_distance(4.0);

    assert_world_params!(
        &world,
        WorldParams {
            screen_width: 800.0,
            screen_height: 400.0,
            width: 16.0,
            height: 8.0,
            distance: 4.0,
            camera_position: Point2 { x: 0.0, y: 0.0 }
        },
        DELTA
    );
}

#[test]
fn test_world_viewpoint_change() {
    let mut world = World::new(800.0, 400.0, 8.0);
    let expected_camera_position = Point2 { x: 5.0, y: 10.0 };
    world.look_at(expected_camera_position);

    assert_world_params!(
        &world,
        WorldParams {
            screen_width: 800.0,
            screen_height: 400.0,
            width: 32.0,
            height: 16.0,
            distance: 8.0,
            camera_position: expected_camera_position
        },
        DELTA
    );
}

#[test]
fn test_world_to_screen_pos_on_new_world() {
    let world = World::new(800.0, 400.0, 8.0);

    let world_point = Point2 { x: 0.0, y: 0.0 };
    let expected_screen_point = Point2 { x: 400.0, y: 200.0 };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );

    let world_point = Point2 { x: 1.0, y: 1.0 };
    let expected_screen_point = Point2 { x: 425.0, y: 175.0 };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );

    let world_point = Point2 { x: 0.0, y: 8.0 };
    let expected_screen_point = Point2 { x: 400.0, y: 0.0 };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );

    let world_point = Point2 { x: -16.0, y: 0.0 };
    let expected_screen_point = Point2 { x: 0.0, y: 200.0 };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );

    let world_point = Point2 { x: -20.0, y: 10.0 };
    let expected_screen_point = Point2 {
        x: -100.0,
        y: -50.0,
    };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );
}

#[test]
fn test_world_to_screen_pos_on_non_default_viewpoint() {
    let mut world = World::new(800.0, 400.0, 8.0);
    world.look_at(Point2 { x: 4.0, y: 8.0 });

    let world_point = Point2 { x: 0.0, y: 0.0 };
    let expected_screen_point = Point2 { x: 300.0, y: 400.0 };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );

    let world_point = Point2 { x: 1.0, y: 1.0 };
    let expected_screen_point = Point2 { x: 325.0, y: 375.0 };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );

    let world_point = Point2 { x: 0.0, y: 8.0 };
    let expected_screen_point = Point2 { x: 300.0, y: 200.0 };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );

    let world_point = Point2 { x: -16.0, y: 0.0 };
    let expected_screen_point = Point2 {
        x: -100.0,
        y: 400.0,
    };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );

    let world_point = Point2 { x: -20.0, y: 10.0 };
    let expected_screen_point = Point2 {
        x: -200.0,
        y: 150.0,
    };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );
}

#[test]
fn test_world_to_screen_pos_on_non_default_viewpoint_after_distance_change() {
    let mut world = World::new(800.0, 400.0, 8.0);
    world.look_at(Point2 { x: 4.0, y: 8.0 });
    world.set_distance(2.0);

    let world_point = Point2 { x: 0.0, y: 0.0 };
    let expected_screen_point = Point2 { x: 0.0, y: 1000.0 };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );

    let world_point = Point2 { x: 1.0, y: 1.0 };
    let expected_screen_point = Point2 { x: 100.0, y: 900.0 };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );

    let world_point = Point2 { x: 0.0, y: 8.0 };
    let expected_screen_point = Point2 { x: 0.0, y: 200.0 };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );

    let world_point = Point2 { x: -16.0, y: 0.0 };
    let expected_screen_point = Point2 {
        x: -1600.0,
        y: 1000.0,
    };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );

    let world_point = Point2 { x: -20.0, y: 10.0 };
    let expected_screen_point = Point2 { x: -2000.0, y: 0.0 };
    assert_eq_point!(
        expected_screen_point,
        world.world_to_screen_pos(world_point),
        DELTA
    );
}

#[test]
fn test_world_to_screen_rect() {
    let world = World::new(800.0, 400.0, 8.0);

    let world_rect = Rect::new(0.0, 0.0, 0.0, 0.0);
    let expected_screen_rect = Rect::new(400.0, 200.0, 0.0, 0.0);
    assert_eq_rect!(
        expected_screen_rect,
        world.world_to_screen_rect(world_rect),
        DELTA
    );

    let world_rect = Rect::new(1.0, 1.0, 1.0, 1.0);
    let expected_screen_rect = Rect::new(425.0, 175.0, 25.0, 25.0);
    assert_eq_rect!(
        expected_screen_rect,
        world.world_to_screen_rect(world_rect),
        DELTA
    );

    let world_rect = Rect::new(0.0, 0.0, 32.0, 16.0);
    let expected_screen_rect = Rect::new(400.0, 200.0, 800.0, 400.0);
    assert_eq_rect!(
        expected_screen_rect,
        world.world_to_screen_rect(world_rect),
        DELTA
    );

    let world_rect = Rect::new(0.0, 0.0, -1.0, -1.0);
    let expected_screen_rect = Rect::new(400.0, 200.0, -25.0, -25.0);
    assert_eq_rect!(
        expected_screen_rect,
        world.world_to_screen_rect(world_rect),
        DELTA
    );
}
