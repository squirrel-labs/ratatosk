use rask_game_engine::boxes::{AABox, RBox};
use rask_game_engine::collide::*;
use rask_game_engine::math::Vec2;

#[test]
fn test_collide_dot_dot() {
    let a = Vec2::new(1.0, 7.5);
    assert!(a.collides(&a));
}

#[test]
fn test_not_collide_dot_dot() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(5.0, 7.5);
    assert!(!a.collides(&b));
}

#[test]
fn test_collide_aabox_dot() {
    let a = Vec2::new(1.0, 2.5);
    let b = Vec2::new(3.0, 7.5);
    let c = Vec2::new(1.5, 5.0);
    let aa_box = AABox { pos: a, size: b };

    assert!(aa_box.collides(&c));
}

#[test]
fn test_not_collide_aabox_dot() {
    let a = Vec2::new(1.0, 7.5);
    let b = Vec2::new(3.0, 2.5);
    let c = Vec2::new(0.5, 5.0);
    let aa_box = AABox { pos: a, size: b };

    assert!(!(aa_box.collides(&c)));
}

#[test]
fn test_collide_aabox_aabox_intersecting() {
    let a = Vec2::new(1.0, 2.5);
    let b = Vec2::new(3.0, 2.5);
    let aa_box = AABox { pos: a, size: b };
    let a = Vec2::new(2.0, 3.5);
    let b = Vec2::new(3.0, 7.5);
    let bb_box = AABox { pos: a, size: b };

    assert!(aa_box.collides(&bb_box));
}

#[test]
fn test_collide_aabox_aabox_crossed() {
    let a = Vec2::new(2.0, 0.5);
    let b = Vec2::new(1.0, 7.5);
    let aa_box = AABox { pos: a, size: b };
    let a = Vec2::new(1.0, 3.5);
    let b = Vec2::new(5.0, 4.5);
    let bb_box = AABox { pos: a, size: b };

    assert!(aa_box.collides(&bb_box));
}

#[test]
fn test_not_collide_aabox_aabox() {
    let a = Vec2::new(1.0, 1.0);
    let b = Vec2::new(1.0, 1.0);
    let aa_box = AABox { pos: a, size: b };
    let a = Vec2::new(3.0, 3.5);
    let b = Vec2::new(3.0, 7.5);
    let bb_box = AABox { pos: a, size: b };

    assert!(!(aa_box.collides(&bb_box)));
}

#[test]
fn test_collide_rbox_dot() {
    let a = Vec2::new(1.0, 1.0);
    let b = Vec2::new(1.0, 1.0);
    let c = Vec2::new(1.0, -1.0);
    let aa_box = RBox {
        pos: a,
        v1: b,
        v2: c,
    };

    let c = Vec2::new(1.6, 0.6);

    assert!(aa_box.collides(&c));
}

#[test]
fn test_not_collide_rbox_dot() {
    let a = Vec2::new(1.0, 1.0);
    let b = Vec2::new(1.0, 1.0);
    let c = Vec2::new(1.0, -1.0);
    let aa_box = RBox {
        pos: a,
        v1: b,
        v2: c,
    };

    let c = Vec2::new(1.4, 0.4);

    assert!(!(aa_box.collides(&c)));
}

#[test]
fn test_collide_rbox_aabox_intersecting() {
    let a = Vec2::new(1.0, 2.5);
    let b = Vec2::new(0.0, 2.5);
    let c = Vec2::new(3.0, 0.5);
    let aa_box = RBox {
        pos: a,
        v1: b,
        v2: c,
    };
    let a = Vec2::new(2.0, 3.5);
    let b = Vec2::new(3.0, 7.5);
    let bb_box = AABox { pos: a, size: b };

    assert!(aa_box.collides(&bb_box));
}

#[test]
fn test_collide_rbox_aabox_edges_touch() {
    let a = Vec2::new(4.0, 5.5);
    let b = Vec2::new(1.0, 7.5);
    let aa_box = RBox::new(a, b, 3.9);
    let a = Vec2::new(0.0, 0.5);
    let b = Vec2::new(4.0, 5.0);
    let bb_box = AABox { pos: a, size: b };

    assert!(aa_box.collides(&bb_box));
}

#[test]
fn test_collide_rbox_aabox_crossed() {
    let a = Vec2::new(2.0, 0.5);
    let b = Vec2::new(1.0, 7.5);
    let aa_box = RBox::new(a, b, 3.9);
    let a = Vec2::new(0.0, 4.5);
    let b = Vec2::new(15.0, 1.5);
    let bb_box = AABox { pos: a, size: b };

    assert!(aa_box.collides(&bb_box));
}

#[test]
fn test_not_collide_rbox_aabox_next_to() {
    let a = Vec2::new(2.0, 0.5);
    let b = Vec2::new(1.0, 7.5);
    let aa_box = RBox::new(a, b, 3.9);
    let a = Vec2::new(5.0, 40.5);
    let b = Vec2::new(15.0, 1.5);
    let bb_box = AABox { pos: a, size: b };

    assert!(!aa_box.collides(&bb_box));
}

#[test]
fn test_not_collide_rbox_aabox() {
    let a = Vec2::new(1.0, 1.0);
    let b = Vec2::new(0.0, 1.0);
    let c = Vec2::new(1.0, 0.0);
    let aa_box = RBox {
        pos: a,
        v1: b,
        v2: c,
    };
    let a = Vec2::new(3.0, 3.5);
    let b = Vec2::new(3.0, 7.5);
    let bb_box = AABox { pos: a, size: b };

    assert!(!(aa_box.collides(&bb_box)));
}
