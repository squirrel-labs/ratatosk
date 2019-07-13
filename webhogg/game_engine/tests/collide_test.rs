use game_engine::boxes::{AABox, RBox};
use game_engine::collide::*;
use game_engine::math::Vec2;

#[test]
fn test_collide_dot_dot() {
    let a = Vec2 { x: 1.0, y: 7.5 };
    assert!(a.collides(&a));
}

#[test]
fn test_not_collide_dot_dot() {
    let a = Vec2 { x: 1.0, y: 7.5 };
    let b = Vec2 { x: 5.0, y: 7.5 };
    assert!(!a.collides(&b));
}

#[test]
fn test_collide_aabox_dot() {
    let a = Vec2 { x: 1.0, y: 2.5 };
    let b = Vec2 { x: 3.0, y: 7.5 };
    let c = Vec2 { x: 1.5, y: 5.0 };
    let aa_box = AABox { pos: a, size: b };

    assert!(aa_box.collides(&c));
}

#[test]
fn test_not_collide_aabox_dot() {
    let a = Vec2 { x: 1.0, y: 7.5 };
    let b = Vec2 { x: 3.0, y: 2.5 };
    let c = Vec2 { x: 0.5, y: 5.0 };
    let aa_box = AABox { pos: a, size: b };

    assert!(!(aa_box.collides(&c)));
}

#[test]
fn test_collide_aabox_aabox_intersecting() {
    let a = Vec2 { x: 1.0, y: 2.5 };
    let b = Vec2 { x: 3.0, y: 2.5 };
    let aa_box = AABox { pos: a, size: b };
    let a = Vec2 { x: 2.0, y: 3.5 };
    let b = Vec2 { x: 3.0, y: 7.5 };
    let bb_box = AABox { pos: a, size: b };

    assert!(aa_box.collides(&bb_box));
}

#[test]
fn test_collide_aabox_aabox_crossed() {
    let a = Vec2 { x: 2.0, y: 0.5 };
    let b = Vec2 { x: 1.0, y: 7.5 };
    let aa_box = AABox { pos: a, size: b };
    let a = Vec2 { x: 1.0, y: 3.5 };
    let b = Vec2 { x: 5.0, y: 4.5 };
    let bb_box = AABox { pos: a, size: b };

    assert!(aa_box.collides(&bb_box));
}

#[test]
fn test_not_collide_aabox_aabox() {
    let a = Vec2 { x: 1.0, y: 1.0 };
    let b = Vec2 { x: 1.0, y: 1.0 };
    let aa_box = AABox { pos: a, size: b };
    let a = Vec2 { x: 3.0, y: 3.5 };
    let b = Vec2 { x: 3.0, y: 7.5 };
    let bb_box = AABox { pos: a, size: b };

    assert!(!(aa_box.collides(&bb_box)));
}

#[test]
fn test_collide_rbox_dot() {
    let a = Vec2 { x: 1.0, y: 1.0 };
    let b = Vec2 { x: 1.0, y: 1.0 };
    let c = Vec2 { x: 1.0, y: -1.0 };
    let aa_box = RBox {
        pos: a,
        v1: b,
        v2: c,
    };

    let c = Vec2 { x: 1.6, y: 0.6 };

    assert!(aa_box.collides(&c));
}

#[test]
fn test_not_collide_rbox_dot() {
    let a = Vec2 { x: 1.0, y: 1.0 };
    let b = Vec2 { x: 1.0, y: 1.0 };
    let c = Vec2 { x: 1.0, y: -1.0 };
    let aa_box = RBox {
        pos: a,
        v1: b,
        v2: c,
    };

    let c = Vec2 { x: 1.4, y: 0.4 };

    assert!(!(aa_box.collides(&c)));
}

#[test]
fn test_collide_rbox_aabox_intersecting() {
    let a = Vec2 { x: 1.0, y: 2.5 };
    let b = Vec2 { x: 0.0, y: 2.5 };
    let c = Vec2 { x: 3.0, y: 0.5 };
    let aa_box = RBox {
        pos: a,
        v1: b,
        v2: c,
    };
    let a = Vec2 { x: 2.0, y: 3.5 };
    let b = Vec2 { x: 3.0, y: 7.5 };
    let bb_box = AABox { pos: a, size: b };

    assert!(aa_box.collides(&bb_box));
}

#[test]
fn test_collide_rbox_aabox_edges_touch() {
    let a = Vec2 { x: 4.0, y: 5.5 };
    let b = Vec2 { x: 1.0, y: 7.5 };
    let aa_box = RBox::new(a, b, 3.9);
    let a = Vec2 { x: 0.0, y: 0.5 };
    let b = Vec2 { x: 4.0, y: 5.0 };
    let bb_box = AABox { pos: a, size: b };

    assert!(aa_box.collides(&bb_box));
}

#[test]
fn test_collide_rbox_aabox_crossed() {
    let a = Vec2 { x: 2.0, y: 0.5 };
    let b = Vec2 { x: 1.0, y: 7.5 };
    let aa_box = RBox::new(a, b, 3.9);
    let a = Vec2 { x: 0.0, y: 4.5 };
    let b = Vec2 { x: 15.0, y: 1.5 };
    let bb_box = AABox { pos: a, size: b };

    assert!(aa_box.collides(&bb_box));
}

#[test]
fn test_not_collide_rbox_aabox_next_to() {
    let a = Vec2 { x: 2.0, y: 0.5 };
    let b = Vec2 { x: 1.0, y: 7.5 };
    let aa_box = RBox::new(a, b, 3.9);
    let a = Vec2 { x: 5.0, y: 40.5 };
    let b = Vec2 { x: 15.0, y: 1.5 };
    let bb_box = AABox { pos: a, size: b };

    assert!(!aa_box.collides(&bb_box));
}

#[test]
fn test_not_collide_rbox_aabox() {
    let a = Vec2 { x: 1.0, y: 1.0 };
    let b = Vec2 { x: 0.0, y: 1.0 };
    let c = Vec2 { x: 1.0, y: 0.0 };
    let aa_box = RBox {
        pos: a,
        v1: b,
        v2: c,
    };
    let a = Vec2 { x: 3.0, y: 3.5 };
    let b = Vec2 { x: 3.0, y: 7.5 };
    let bb_box = AABox { pos: a, size: b };

    assert!(!(aa_box.collides(&bb_box)));
}
