use rask_game_engine::boxes::*;
use rask_game_engine::math::Vec2;

#[test]
fn test_add_aabox_vec2() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-3.0, 2.5);
    let aa_box = AABox { pos: a, size: b };
    let bb_box = AABox {
        pos: a + b,
        size: b,
    };

    assert_eq!(aa_box + b, bb_box);
}

#[test]
fn test_add_assign_aabox_vec2() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-3.0, 2.5);
    let mut aa_box = AABox { pos: a, size: b };
    let bb_box = AABox {
        pos: a + b,
        size: b,
    };
    aa_box += b;

    assert_eq!(aa_box, bb_box);
}

#[test]
fn test_sub_aabox_vec2() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-3.0, 2.5);
    let aa_box = AABox { pos: a, size: b };
    let bb_box = AABox {
        pos: a - b,
        size: b,
    };

    assert_eq!(aa_box - b, bb_box);
}

#[test]
fn test_sub_assign_aabox_vec2() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-3.0, 2.5);
    let mut aa_box = AABox { pos: a, size: b };
    let bb_box = AABox {
        pos: a - b,
        size: b,
    };
    aa_box -= b;

    assert_eq!(aa_box, bb_box);
}

#[test]
fn test_add_rbox_vec2() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-3.0, 2.5);
    let c = Vec2::new(-3.0, 2.5);
    let aa_box = RBox {
        pos: a,
        v1: b,
        v2: c,
    };
    let bb_box = RBox {
        pos: a + b,
        v1: b,
        v2: c,
    };

    assert_eq!(aa_box + b, bb_box);
}

#[test]
fn test_add_assign_rbox_vec2() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-3.0, 2.5);
    let c = Vec2::new(-3.0, 2.5);
    let mut aa_box = RBox {
        pos: a,
        v1: b,
        v2: c,
    };
    let bb_box = RBox {
        pos: a + b,
        v1: b,
        v2: c,
    };
    aa_box += b;

    assert_eq!(aa_box, bb_box);
}

#[test]
fn test_sub_rbox_vec2() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-3.0, 2.5);
    let c = Vec2::new(-3.0, 2.5);
    let aa_box = RBox {
        pos: a,
        v1: b,
        v2: c,
    };
    let bb_box = RBox {
        pos: a - b,
        v1: b,
        v2: c,
    };

    assert_eq!(aa_box - b, bb_box);
}

#[test]
fn test_sub_assign_rbox_vec2() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(-3.0, 2.5);
    let c = Vec2::new(-3.0, 2.5);
    let mut aa_box = RBox {
        pos: a,
        v1: b,
        v2: c,
    };
    let bb_box = RBox {
        pos: a - b,
        v1: b,
        v2: c,
    };
    aa_box -= b;

    assert_eq!(aa_box, bb_box);
}
