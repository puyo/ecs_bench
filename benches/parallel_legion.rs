#![feature(test)]

extern crate ecs_bench;
extern crate legion;
extern crate rayon;
extern crate test;

use ecs_bench::parallel::{N, R, W1, W2};
use legion::*;
use test::Bencher;

#[system(for_each)]
fn sys1(w1: &mut W1, r: &R) {
    w1.x = r.x;
}

#[system(for_each)]
fn sys2(w2: &mut W2, r: &R) {
    w2.x = r.x;
}

fn build() -> (World, Schedule, Resources) {
    let mut world = World::default();
    let resources = Resources::default();

    world.extend((0..N).map(|_| (R { x: 0.0 }, W1 { x: 0.0 }, W2 { x: 0.0 })));

    let schedule = Schedule::builder()
        .add_system(sys1_system())
        .add_system(sys2_system())
        .build();

    (world, schedule, resources)
}

// ---

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let (mut world, mut schedule, mut resources) = build();

    b.iter(|| {
        schedule.execute(&mut world, &mut resources);
    });
}
