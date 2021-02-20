#[macro_export]
macro_rules! assert_eq_float {
    ($expected:expr , $actual:expr, $delta: expr) => {
        assert!(
            ($expected - $actual).abs() < $delta,
            format!("expected: {}, actual: {}", $expected, $actual)
        );
    };
}

#[macro_export]
macro_rules! assert_eq_point {
    ($expected:expr , $actual:expr, $delta: expr) => {
        assert_eq_float!($expected.x, $actual.x, $delta);
        assert_eq_float!($expected.y, $actual.y, $delta);
    };
}

#[macro_export]
macro_rules! assert_eq_rect {
    ($expected:expr , $actual:expr, $delta: expr) => {
        assert_eq_float!($expected.x, $actual.x, $delta);
        assert_eq_float!($expected.y, $actual.y, $delta);
        assert_eq_float!($expected.w, $actual.w, $delta);
        assert_eq_float!($expected.h, $actual.h, $delta);
    };
}
