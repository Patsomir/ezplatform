use ezplatform::tilemap::*;
use ggez::{graphics::Rect, mint::Point2};

#[macro_use]
mod float_asserts;

#[macro_export]
macro_rules! assert_eq_tilemap_segment {
    ($expected:expr , $actual:expr) => {
        assert_eq!($expected.left_bound, $actual.left_bound);
        assert_eq!($expected.top_bound, $actual.top_bound);
        assert_eq!($expected.right_bound, $actual.right_bound);
        assert_eq!($expected.bottom_bound, $actual.bottom_bound);
    };
}

const DELTA: f32 = 0.00001;

struct TestTilemap {
    origin: Point2<i32>,
    tile_width: f32,
    tile_height: f32,
}

impl Tilemap for TestTilemap {
    fn origin(&self) -> Point2<i32> {
        self.origin
    }

    fn tile_width(&self) -> f32 {
        self.tile_width
    }

    fn tile_height(&self) -> f32 {
        self.tile_height
    }
}

impl TestTilemap {
    fn new(x: i32, y: i32, tile_width: f32, tile_height: f32) -> Self {
        Self {
            origin: Point2 { x, y },
            tile_width,
            tile_height,
        }
    }
}

#[test]
fn test_tilemap_to_world_with_zero_origin() {
    let tilemap = TestTilemap::new(0, 0, 1.0, 1.0);

    let tilemap_point = Point2 { x: 0.0, y: 0.0 };
    let expected_world_point = Point2 { x: 0.0, y: 0.0 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap_point = Point2 { x: 1.0, y: 1.0 };
    let expected_world_point = Point2 { x: 1.0, y: 1.0 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap_point = Point2 { x: -2.0, y: -0.5 };
    let expected_world_point = Point2 { x: -2.0, y: -0.5 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap = TestTilemap::new(0, 0, 0.5, 2.0);

    let tilemap_point = Point2 { x: 0.0, y: 0.0 };
    let expected_world_point = Point2 { x: 0.0, y: 0.0 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap_point = Point2 { x: 1.0, y: 1.0 };
    let expected_world_point = Point2 { x: 0.5, y: 2.0 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap_point = Point2 { x: -2.0, y: -0.5 };
    let expected_world_point = Point2 { x: -1.0, y: -1.0 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );
}

#[test]
fn test_tilemap_to_world_with_non_zero_origin() {
    let tilemap = TestTilemap::new(2, 4, 1.0, 1.0);

    let tilemap_point = Point2 { x: 0.0, y: 0.0 };
    let expected_world_point = Point2 { x: -2.0, y: -4.0 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap_point = Point2 { x: 1.0, y: 1.0 };
    let expected_world_point = Point2 { x: -1.0, y: -3.0 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap_point = Point2 { x: -2.0, y: -0.5 };
    let expected_world_point = Point2 { x: -4.0, y: -4.5 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap = TestTilemap::new(2, 4, 0.5, 2.0);

    let tilemap_point = Point2 { x: 0.0, y: 0.0 };
    let expected_world_point = Point2 { x: -1.0, y: -8.0 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap_point = Point2 { x: 1.0, y: 1.0 };
    let expected_world_point = Point2 { x: -0.5, y: -6.0 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap_point = Point2 { x: -2.0, y: -0.5 };
    let expected_world_point = Point2 { x: -2.0, y: -9.0 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap = TestTilemap::new(-2, 4, 1.0, 1.0);

    let tilemap_point = Point2 { x: 0.0, y: 0.0 };
    let expected_world_point = Point2 { x: 2.0, y: -4.0 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap_point = Point2 { x: 1.0, y: 1.0 };
    let expected_world_point = Point2 { x: 3.0, y: -3.0 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );

    let tilemap_point = Point2 { x: -2.0, y: -0.5 };
    let expected_world_point = Point2 { x: 0.0, y: -4.5 };
    assert_eq_point!(
        expected_world_point,
        tilemap.tilemap_to_world(tilemap_point),
        DELTA
    );
}

#[test]
fn test_world_to_tilemap_with_zero_origin() {
    let tilemap = TestTilemap::new(0, 0, 1.0, 1.0);

    let world_point = Point2 { x: 0.0, y: 0.0 };
    let expected_tilemap_point = Point2 { x: 0.0, y: 0.0 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let world_point = Point2 { x: 1.0, y: 1.0 };
    let expected_tilemap_point = Point2 { x: 1.0, y: 1.0 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let world_point = Point2 { x: -2.0, y: -0.5 };
    let expected_tilemap_point = Point2 { x: -2.0, y: -0.5 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let tilemap = TestTilemap::new(0, 0, 0.5, 2.0);

    let world_point = Point2 { x: 0.0, y: 0.0 };
    let expected_tilemap_point = Point2 { x: 0.0, y: 0.0 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let world_point = Point2 { x: 1.0, y: 1.0 };
    let expected_tilemap_point = Point2 { x: 2.0, y: 0.5 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let world_point = Point2 { x: -2.0, y: -0.5 };
    let expected_tilemap_point = Point2 { x: -4.0, y: -0.25 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );
}

#[test]
fn test_world_to_tilemap_with_non_zero_origin() {
    let tilemap = TestTilemap::new(2, 4, 1.0, 1.0);

    let world_point = Point2 { x: 0.0, y: 0.0 };
    let expected_tilemap_point = Point2 { x: 2.0, y: 4.0 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let world_point = Point2 { x: 1.0, y: 1.0 };
    let expected_tilemap_point = Point2 { x: 3.0, y: 5.0 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let world_point = Point2 { x: -2.0, y: -0.5 };
    let expected_tilemap_point = Point2 { x: 0.0, y: 3.5 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let tilemap = TestTilemap::new(2, 4, 0.5, 2.0);

    let world_point = Point2 { x: 0.0, y: 0.0 };
    let expected_tilemap_point = Point2 { x: 2.0, y: 4.0 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let world_point = Point2 { x: 1.0, y: 1.0 };
    let expected_tilemap_point = Point2 { x: 4.0, y: 4.5 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let world_point = Point2 { x: -2.0, y: -0.5 };
    let expected_tilemap_point = Point2 { x: -2.0, y: 3.75 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let tilemap = TestTilemap::new(-2, 4, 0.5, 2.0);

    let world_point = Point2 { x: 0.0, y: 0.0 };
    let expected_tilemap_point = Point2 { x: -2.0, y: 4.0 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let world_point = Point2 { x: 1.0, y: 1.0 };
    let expected_tilemap_point = Point2 { x: 0.0, y: 4.5 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );

    let world_point = Point2 { x: -2.0, y: -0.5 };
    let expected_tilemap_point = Point2 { x: -6.0, y: 3.75 };
    assert_eq_point!(
        expected_tilemap_point,
        tilemap.world_to_tilemap(world_point),
        DELTA
    );
}

#[test]
fn test_rect_overlap() {
    let tilemap = TestTilemap::new(0, 0, 1.0, 2.0);

    let rect = Rect::new(0.0, 0.0, 0.0, 0.0);
    let expected = TilemapSegment {
        top_bound: 0,
        bottom_bound: 0,
        left_bound: 0,
        right_bound: 0,
    };
    assert_eq_tilemap_segment!(expected, tilemap.rect_overlap(rect));

    let rect = Rect::new(0.0, 0.0, 1.5, 1.5);
    let expected = TilemapSegment {
        top_bound: 0,
        bottom_bound: 0,
        left_bound: -1,
        right_bound: 1,
    };
    assert_eq_tilemap_segment!(expected, tilemap.rect_overlap(rect));

    let rect = Rect::new(-2.0, 0.0, 1.5, 2.5);
    let expected = TilemapSegment {
        top_bound: 1,
        bottom_bound: -1,
        left_bound: -3,
        right_bound: -1,
    };
    assert_eq_tilemap_segment!(expected, tilemap.rect_overlap(rect));

    let tilemap = TestTilemap::new(2, 4, 1.0, 2.0);

    let rect = Rect::new(0.0, 0.0, 0.0, 0.0);
    let expected = TilemapSegment {
        top_bound: 4,
        bottom_bound: 4,
        left_bound: 2,
        right_bound: 2,
    };
    assert_eq_tilemap_segment!(expected, tilemap.rect_overlap(rect));

    let rect = Rect::new(0.0, 0.0, 1.5, 1.5);
    let expected = TilemapSegment {
        top_bound: 4,
        bottom_bound: 4,
        left_bound: 1,
        right_bound: 3,
    };
    assert_eq_tilemap_segment!(expected, tilemap.rect_overlap(rect));

    let rect = Rect::new(-2.0, 0.0, 1.5, 2.5);
    let expected = TilemapSegment {
        top_bound: 5,
        bottom_bound: 3,
        left_bound: -1,
        right_bound: 1,
    };
    assert_eq_tilemap_segment!(expected, tilemap.rect_overlap(rect));
}

#[test]
fn test_point_overlap() {
    let tilemap = TestTilemap::new(0, 0, 1.0, 2.0);

    let point = Point2 { x: 0.0, y: 0.0 };
    let expected = Point2 { x: 0, y: 0 };
    let actual = tilemap.point_overlap(point);
    assert_eq!(expected.x, actual.x);
    assert_eq!(expected.y, actual.y);

    let point = Point2 { x: 1.0, y: 1.5 };
    let expected = Point2 { x: 1, y: 1 };
    let actual = tilemap.point_overlap(point);
    assert_eq!(expected.x, actual.x);
    assert_eq!(expected.y, actual.y);

    let tilemap = TestTilemap::new(2, 4, 1.0, 2.0);

    let point = Point2 { x: 0.0, y: 0.0 };
    let expected = Point2 { x: 2, y: 4 };
    let actual = tilemap.point_overlap(point);
    assert_eq!(expected.x, actual.x);
    assert_eq!(expected.y, actual.y);

    let point = Point2 { x: 1.0, y: 1.5 };
    let expected = Point2 { x: 3, y: 5 };
    let actual = tilemap.point_overlap(point);
    assert_eq!(expected.x, actual.x);
    assert_eq!(expected.y, actual.y);
}

#[test]
fn test_tile_to_world() {
    let tilemap = TestTilemap::new(0, 0, 1.0, 2.0);

    let point = Point2 { x: 0, y: 0 };
    let expected = Rect::new(0.0, 0.0, 1.0, 2.0);
    assert_eq_rect!(expected, tilemap.tile_to_world(point), DELTA);

    let point = Point2 { x: 1, y: -1 };
    let expected = Rect::new(1.0, -2.0, 1.0, 2.0);
    assert_eq_rect!(expected, tilemap.tile_to_world(point), DELTA);

    let tilemap = TestTilemap::new(2, 4, 1.0, 2.0);

    let point = Point2 { x: 0, y: 0 };
    let expected = Rect::new(-2.0, -8.0, 1.0, 2.0);
    assert_eq_rect!(expected, tilemap.tile_to_world(point), DELTA);

    let point = Point2 { x: 1, y: -1 };
    let expected = Rect::new(-1.0, -10.0, 1.0, 2.0);
    assert_eq_rect!(expected, tilemap.tile_to_world(point), DELTA);
}

#[test]
fn test_segment_to_world() {
    let tilemap = TestTilemap::new(0, 0, 1.0, 2.0);

    let segment = TilemapSegment {
        top_bound: 0,
        bottom_bound: 0,
        left_bound: 0,
        right_bound: 0,
    };
    let expected = Rect::new(0.0, 0.0, 1.0, 2.0);
    assert_eq_rect!(expected, tilemap.segment_to_world(&segment), DELTA);

    let segment = TilemapSegment {
        top_bound: 1,
        bottom_bound: -1,
        left_bound: -1,
        right_bound: 1,
    };
    let expected = Rect::new(0.0, 0.0, 3.0, 6.0);
    assert_eq_rect!(expected, tilemap.segment_to_world(&segment), DELTA);

    let tilemap = TestTilemap::new(2, 4, 1.0, 2.0);

    let segment = TilemapSegment {
        top_bound: 0,
        bottom_bound: 0,
        left_bound: 0,
        right_bound: 0,
    };
    let expected = Rect::new(-2.0, -8.0, 1.0, 2.0);
    assert_eq_rect!(expected, tilemap.segment_to_world(&segment), DELTA);

    let segment = TilemapSegment {
        top_bound: 1,
        bottom_bound: -1,
        left_bound: -1,
        right_bound: 1,
    };
    let expected = Rect::new(-2.0, -8.0, 3.0, 6.0);
    assert_eq_rect!(expected, tilemap.segment_to_world(&segment), DELTA);
}
