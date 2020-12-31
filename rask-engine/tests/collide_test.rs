use rask_engine::boxes::{AABox, RBox};
use rask_engine::collide::*;
use rask_engine::math::Vec2;

const TEST_EPSILON: f32 = 1e-5;

macro_rules! assert_f32_eq {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        assert!(
            (Into::<f32>::into($a) - Into::<f32>::into($b)).abs() < TEST_EPSILON,
            "Inequality: {} != {} in expression `{} == {}`",
            $a,
            $b,
            stringify!($c),
            stringify!($d)
        )
    };
    ($a:expr, $b:expr) => {
        assert_f32_eq!($a, $b, $a, $b)
    };
}

macro_rules! assert_f32_option_eq {
    ($a:expr, $b:expr) => {{
        let m: (Option<f32>, Option<f32>) = ($a, $b);
        match m {
            (Some(a), Some(b)) => {
                assert_f32_eq!(Into::<f32>::into(a), Into::<f32>::into(b), $a, $b)
            }
            (None, None) => (),
            (a, b) => assert!(
                false,
                "Inequality: {:?} != {:?} in expression `{} == {}`",
                a,
                b,
                stringify!($a),
                stringify!($b)
            ),
        }
    }};
}

macro_rules! assert_collide {
    ($a:expr, $b:expr, ($x:expr, $y:expr), $r:expr) => {
        assert_f32_option_eq!($a.collide_after(&$b, Vec2::new($x, $y)), $r)
    };
}

#[test]
fn test_collide_dot_dot_at_half() {
    assert_collide!(
        Vec2::new(3.0, 3.0),
        Vec2::new(2.0, 4.0),
        (4.0 - 6.0, 6.0 - 4.0),
        Some(0.5)
    )
}

#[test]
fn test_collide_dot_dot_at_one_third() {
    assert_collide!(
        Vec2::new(1.0, 1.0),
        Vec2::new(3.0, 8.0),
        (6.0 - 3.0, 9.0 + 1.5),
        Some(1.0 / 3.0)
    )
}

#[test]
fn test_collide_dot_dot_at_one_third_reverse() {
    assert_collide!(
        Vec2::new(3.0, 8.0),
        Vec2::new(1.0, 1.0),
        (3.0 - 6.0, -1.5 - 9.0),
        Some(1.0 / 3.0)
    )
}

#[test]
fn test_collide_dot_dot_failiure() {
    assert_collide!(
        Vec2::new(1.0, 2.0),
        Vec2::new(3.0, 1.0),
        (3.0 + 2.0, 3.0 - 4.0),
        None
    )
}

#[test]
fn test_collide_dot_dot_parallel() {
    assert_collide!(Vec2::new(1.0, 1.0), Vec2::new(2.0, 1.0), (0.0, 0.0), None)
}

#[test]
fn test_collide_dot_aabox_straight() {
    assert_collide!(
        AABox {
            pos: Vec2::new(3.0, 1.0),
            size: Vec2::new(4.0, 2.0),
        },
        Vec2::new(1.0, 3.0),
        (0.0 - 3.0, 2.0 - 1.0),
        Some(1.0 / 3.0)
    )
}

#[test]
fn test_collide_dot_aabox_from_all_sides() {
    let aabox = AABox {
        pos: Vec2::new(3.0, 4.0),
        size: Vec2::new(2.0, 3.0),
    };
    let tests = [
        ((2.0, 2.0), (2.0, 3.0), 1.0 / 3.0),   // bottom left
        ((4.0, 2.0), (0.0, 3.0), 1.0 / 3.0),   // bottom
        ((6.0, 2.0), (-2.0, 3.0), 1.0 / 3.0),  // bottom right
        ((7.0, 6.0), (-3.0, 0.0), 1.0 / 3.0),  // right
        ((6.0, 8.0), (-2.0, -2.5), 0.5),       // top right
        ((4.0, 10.0), (0.0, -3.5), 1.0 / 7.0), // top
        ((0.0, 9.0), (4.5, -2.5), 1.0 / 5.0),  // top left
        ((2.5, 5.5), (1.5, -0.5), 2.0 / 3.0),  // left
    ];
    for &((ax, ay), v, r) in &tests {
        assert_collide!(Vec2::new(ax, ay), aabox, (v.0, v.1), Some(r));
    }
}

#[test]
fn test_collide_dot_aabox_intersect_pass_through() {
    assert_collide!(
        Vec2::new(4.0, 4.0),
        AABox {
            pos: Vec2::new(1.0, 1.0),
            size: Vec2::new(5.0, 2.0),
        },
        (-5.0, -3.0),
        Some(2.0 / 3.0)
    )
}

#[test]
fn test_collide_dot_aabox_trivial_failiure() {
    assert_collide!(
        AABox {
            pos: Vec2::new(3.0, 2.0),
            size: Vec2::new(3.0, 2.0),
        },
        Vec2::new(1.0, 2.0),
        (3.0 - 3.0, 2.0 - 1.0),
        None
    )
}

#[test]
fn test_collide_dot_aabox_pass_parallel_x() {
    assert_collide!(
        Vec2::new(-2.0, -1.0),
        AABox {
            pos: Vec2::new(2.0, 1.0),
            size: Vec2::new(4.0, 3.0),
        },
        (9.0, 0.0),
        None
    )
}

#[test]
fn test_collide_dot_aabox_pass_parallel_y() {
    assert_collide!(
        Vec2::new(6.5, -5.5),
        AABox {
            pos: Vec2::new(2.0, 1.0),
            size: Vec2::new(4.0, 3.0),
        },
        (0.0, -3.0),
        None
    )
}

#[test]
fn test_collide_dot_aabox_pass_diagonal() {
    assert_collide!(
        Vec2::new(3.0, 0.0),
        AABox {
            pos: Vec2::new(2.0, 1.0),
            size: Vec2::new(4.0, 3.0),
        },
        (-4.0, 3.0),
        None
    )
}

#[test]
fn test_collide_aabox_aabox_trivial_intersect() {
    assert_collide!(
        AABox {
            pos: Vec2::new(1.0, 1.0),
            size: Vec2::new(2.0, 2.0),
        },
        AABox {
            pos: Vec2::new(1.0, 4.0),
            size: Vec2::new(2.0, 2.0),
        },
        (0.0, 2.0),
        Some(0.5)
    )
}

#[test]
fn test_collide_aabox_aabox_up() {
    assert_collide!(
        AABox {
            pos: Vec2::new(0.0, 0.0),
            size: Vec2::new(3.0, 2.0),
        },
        AABox {
            pos: Vec2::new(4.0, 3.0),
            size: Vec2::new(1.0, 5.0),
        },
        (3.0, 2.0),
        Some(0.5)
    )
}

#[test]
fn test_collide_aabox_aabox_right() {
    assert_collide!(
        AABox {
            pos: Vec2::new(-2.0, 6.0),
            size: Vec2::new(1.0, 1.0),
        },
        AABox {
            pos: Vec2::new(4.0, 2.0),
            size: Vec2::new(1.0, 5.0),
        },
        (5.5, -3.5),
        Some(1.0 / 11.0)
    )
}

#[test]
fn test_collide_aabox_aabox_left() {
    assert_collide!(
        AABox {
            pos: Vec2::new(6.0, 3.0),
            size: Vec2::new(3.0, 0.07),
        },
        AABox {
            pos: Vec2::new(1.0, 1.0),
            size: Vec2::new(4.0, 2.0),
        },
        (-6.0, -5.0),
        Some(5.0 / 6.0)
    )
}
#[test]
fn test_collide_aabox_aabox_down() {
    assert_collide!(
        AABox {
            pos: Vec2::new(-2.5, 0.0),
            size: Vec2::new(0.5, 5.0),
        },
        AABox {
            pos: Vec2::new(-2.0, 7.0),
            size: Vec2::new(1.0, 1.0),
        },
        (0.5, 5.5),
        Some(7.0 / 11.0)
    )
}

#[test]
fn test_collide_aabox_aabox_pass_tight() {
    assert_collide!(
        AABox {
            pos: Vec2::new(6.0, 3.0),
            size: Vec2::new(1.0, 4.0),
        },
        AABox {
            pos: Vec2::new(1.0, 1.0),
            size: Vec2::new(4.0, 2.0),
        },
        (-2.0, -15.0),
        None
    )
}

#[test]
fn test_collide_aabox_aabox_konverge() {
    assert_collide!(
        AABox {
            pos: Vec2::new(1.0, 2.0),
            size: Vec2::new(6.0, 2.0),
        },
        AABox {
            pos: Vec2::new(3.0, 6.0),
            size: Vec2::new(1.0, 1.0),
        },
        (-0.05, 1.95),
        None
    )
}

#[test]
fn test_collide_dot_rbox_simple() {
    assert_collide!(
        Vec2::new(4.0, -1.0),
        RBox {
            pos: Vec2::new(3.0, 1.0),
            v1: Vec2::new(-2.0, 2.0),
            v2: Vec2::new(4.0, 4.0),
        },
        (0.0, 5.0),
        Some(2.0 / 5.0)
    )
}

#[test]
fn test_collide_dot_rbox_equivalent_rboxes() {
    fn test(pos: Vec2, v1: Vec2, v2: Vec2) {
        for &(v1, v2) in &[(v1, v2), (v2, v1)] {
            assert_collide!(
                Vec2::new(1.5, 1.0),
                RBox { pos, v1, v2 },
                (1.0, 3.0),
                Some(3.0 / 4.0)
            )
        }
    }
    let (v1, v2) = (Vec2::new(-1.0, 3.0), Vec2::new(6.0, 2.0));
    test(Vec2::new(2.0, 1.0), v1, v2);
    test(Vec2::new(8.0, 3.0), -v2, v1);
    test(Vec2::new(7.0, 6.0), -v1, -v2);
    test(Vec2::new(1.0, 4.0), v2, -v1);
}

#[test]
fn test_collide_dot_rbox_pass_through() {
    assert_collide!(
        Vec2::new(6.0, 5.0),
        RBox {
            pos: Vec2::new(1.0, 5.0),
            v1: Vec2::new(4.0, 1.0),
            v2: Vec2::new(1.0, -4.0),
        },
        (-4.0, 1.0),
        Some(0.8)
    )
}
