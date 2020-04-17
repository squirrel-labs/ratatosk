#![feature(test)]

extern crate test;

use test::Bencher;

use rask_engine::boxes::{AABox, RBox};
use rask_engine::collide::*;
use rask_engine::math::Vec2;

fn sat_collides(box1: &RBox, box2: &AABox) -> bool {
    let rbox: RBox = (*box2).into();
    box1.collides(&rbox)
}

#[bench]
fn bench_jan_collide_intersecting(bencher: &mut Bencher) {
    let box1 = RBox {
        pos: Vec2::new(1.0, 2.5),
        v1: Vec2::new(3.0, 0.0),
        v2: Vec2::new(0.0, 2.5),
    };
    let box2 = AABox {
        pos: Vec2::new(2.0, 3.5),
        size: Vec2::new(3.0, 7.5),
    };

    bencher.iter(|| box1.collides(&box2));
}

#[bench]
fn bench_jan_collide_crossed(bencher: &mut Bencher) {
    let a = Vec2::new(2.0, 0.5);
    let b = Vec2::new(1.0, 7.5);
    let box1 = RBox::new(a, b, 3.9);
    let box2 = AABox {
        pos: Vec2::new(0.0, 4.5),
        size: Vec2::new(15.0, 1.5),
    };

    bencher.iter(|| box1.collides(&box2));
}

#[bench]
fn bench_jan_not_collide(bencher: &mut Bencher) {
    let box1 = RBox {
        pos: Vec2::new(1.0, 1.0),
        v1: Vec2::new(0.0, 1.0),
        v2: Vec2::new(1.0, 0.0),
    };
    let box2 = AABox {
        pos: Vec2::new(3.0, 3.5),
        size: Vec2::new(3.0, 7.5),
    };

    bencher.iter(|| !box1.collides(&box2));
}

#[bench]
fn bench_sat_collide_intersecting(bencher: &mut Bencher) {
    let box1 = RBox {
        pos: Vec2::new(1.0, 2.5),
        v1: Vec2::new(3.0, 0.0),
        v2: Vec2::new(0.0, 2.5),
    };
    let box2 = AABox {
        pos: Vec2::new(2.0, 3.5),
        size: Vec2::new(3.0, 7.5),
    };

    bencher.iter(|| sat_collides(&box1, &box2));
}

#[bench]
fn bench_sat_collide_crossed(bencher: &mut Bencher) {
    let a = Vec2::new(2.0, 0.5);
    let b = Vec2::new(1.0, 7.5);
    let box1 = RBox::new(a, b, 3.9);
    let box2 = AABox {
        pos: Vec2::new(0.0, 4.5),
        size: Vec2::new(15.0, 1.5),
    };

    bencher.iter(|| sat_collides(&box1, &box2));
}

#[bench]
fn bench_sat_not_collide(bencher: &mut Bencher) {
    let box1 = RBox {
        pos: Vec2::new(1.0, 1.0),
        v1: Vec2::new(0.0, 1.0),
        v2: Vec2::new(1.0, 0.0),
    };
    let box2 = AABox {
        pos: Vec2::new(3.0, 3.5),
        size: Vec2::new(3.0, 7.5),
    };

    bencher.iter(|| !sat_collides(&box1, &box2));
}
