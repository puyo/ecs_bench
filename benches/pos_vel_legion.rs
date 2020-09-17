#![feature(test)]

extern crate ecs_bench;
extern crate legion;
extern crate test;

use ecs_bench::pos_vel::{Position, Velocity, N_POS, N_POS_PER_VEL};
use legion::*;
use test::Bencher;

const N_POS_ONLY: usize = N_POS * (100 - N_POS_PER_VEL) / 100;
const N_POS_AND_VEC: usize = N_POS * (N_POS_PER_VEL) / 100;

fn build() -> World {
    let mut world = World::default();

    world.extend(
        (0..N_POS_AND_VEC).map(|_| (Position { x: 0.0, y: 0.0 }, Velocity { dx: 1.0, dy: 1.0 })),
    );
    world.extend((0..N_POS_ONLY).map(|_| (Position { x: 0.0, y: 0.0 },)));

    world
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut world = build();
    let mut query = <(&Velocity, &mut Position)>::query();

    b.iter(|| {
        for (vel, pos) in query.iter_mut(&mut world) {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }
    })
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}
