use rask_engine::boxes::{AABox, RBox};
use rask_engine::collide::*;
use rask_engine::math::Vec2;

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
fn test_collide_aabox_aabox_same() {
    let aa_box = AABox {
        pos: Vec2::zero(),
        size: Vec2::new(1.0, 1.0),
    };

    assert!(aa_box.collides(&aa_box));
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
    let rbox = RBox {
        pos: a,
        v1: b,
        v2: c,
    };

    let c = Vec2::new(1.6, 0.6);

    assert!(rbox.collides(&c));
}

#[test]
fn test_not_collide_rbox_dot() {
    let a = Vec2::new(1.0, 1.0);
    let b = Vec2::new(1.0, 1.0);
    let c = Vec2::new(1.0, -1.0);
    let rbox = RBox {
        pos: a,
        v1: b,
        v2: c,
    };

    let c = Vec2::new(1.4, 0.4);

    assert!(!(rbox.collides(&c)));
}

#[test]
fn test_collide_rbox_aabox_intersecting() {
    let box1 = RBox {
        pos: Vec2::new(1.0, 2.5),
        v1: Vec2::new(3.0, 0.0),
        v2: Vec2::new(0.0, 2.5),
    };
    let box2 = AABox {
        pos: Vec2::new(2.0, 3.5),
        size: Vec2::new(3.0, 7.5),
    };

    assert!(box1.collides(&box2));
}

#[test]
fn test_collide_rbox_aabox_edges_touch() {
    let a = Vec2::new(4.0, 5.5);
    let b = Vec2::new(1.0, 7.5);
    let box1 = RBox::new(a, b, 3.9);
    let box2 = AABox {
        pos: Vec2::new(0.0, 0.5),
        size: Vec2::new(4.0, 5.0),
    };

    assert!(box1.collides(&box2));
}

#[test]
fn test_collide_rbox_aabox_crossed() {
    let a = Vec2::new(2.0, 0.5);
    let b = Vec2::new(1.0, 7.5);
    let box1 = RBox::new(a, b, 3.9);
    let box2 = AABox {
        pos: Vec2::new(0.0, 4.5),
        size: Vec2::new(15.0, 1.5),
    };

    assert!(box1.collides(&box2));
}

#[test]
fn test_not_collide_rbox_aabox_next_to() {
    let a = Vec2::new(2.0, 0.5);
    let b = Vec2::new(1.0, 7.5);
    let box1 = RBox::new(a, b, 3.9);
    let box2 = AABox {
        pos: Vec2::new(5.0, 40.5),
        size: Vec2::new(15.0, 1.5),
    };

    assert!(!box1.collides(&box2));
}

#[test]
fn test_not_collide_rbox_aabox() {
    let box1 = RBox {
        pos: Vec2::new(1.0, 1.0),
        v1: Vec2::new(0.0, 1.0),
        v2: Vec2::new(1.0, 0.0),
    };
    let box2 = AABox {
        pos: Vec2::new(3.0, 3.5),
        size: Vec2::new(3.0, 7.5),
    };

    assert!(!(box1.collides(&box2)));
}

#[test]
fn test_collide_rbox_rbox_intersecting() {
    let box1 = RBox {
        pos: Vec2::new(1.0, 2.5),
        v1: Vec2::new(3.0, 0.0),
        v2: Vec2::new(0.0, 2.5),
    };
    let box2: RBox = AABox {
        pos: Vec2::new(2.0, 3.5),
        size: Vec2::new(3.0, 7.5),
    }
    .into();

    assert!(box1.collides(&box2));
}

#[test]
fn test_collide_rbox_rbox_edges_touch() {
    let a = Vec2::new(4.0, 5.5);
    let b = Vec2::new(1.0, 7.5);
    let box1 = RBox::new(a, b, 3.9);
    let box2: RBox = AABox {
        pos: Vec2::new(0.0, 0.5),
        size: Vec2::new(4.0, 5.0),
    }
    .into();

    assert!(box1.collides(&box2));
}

#[test]
fn test_collide_rbox_rbox_crossed() {
    let a = Vec2::new(2.0, 0.5);
    let b = Vec2::new(1.0, 7.5);
    let box1 = RBox::new(a, b, 3.9);
    let box2: RBox = AABox {
        pos: Vec2::new(0.0, 4.5),
        size: Vec2::new(15.0, 1.5),
    }
    .into();

    assert!(box1.collides(&box2));
}

#[test]
fn test_collide_rbox_rbox_same() {
    let rbox = RBox::new(Vec2::zero(), Vec2::new(1.0, 1.0), 1.0);

    assert!(rbox.collides(&rbox));
}

#[test]
fn test_not_collide_rbox_rbox_next_to() {
    let a = Vec2::new(2.0, 0.5);
    let b = Vec2::new(1.0, 7.5);
    let box1 = RBox::new(a, b, 3.9);
    let box2: RBox = AABox {
        pos: Vec2::new(5.0, 40.5),
        size: Vec2::new(15.0, 1.5),
    }
    .into();

    assert!(!box1.collides(&box2));
}

#[test]
fn test_not_collide_rbox_rbox() {
    let box1 = RBox {
        pos: Vec2::new(1.0, 1.0),
        v1: Vec2::new(0.0, 1.0),
        v2: Vec2::new(1.0, 0.0),
    };
    let box2 = RBox::new(Vec2::new(3.0, 3.5), Vec2::new(1.0, 1.0), 1.0);

    assert!(!(box1.collides(&box2)));
}
