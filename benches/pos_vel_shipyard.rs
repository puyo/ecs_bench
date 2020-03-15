#![feature(test)]
extern crate test;
use test::Bencher;

extern crate ecs_bench;
extern crate shipyard;

use ecs_bench::pos_vel::*;
use shipyard::prelude::*;

#[system(VelSys)]
fn run(velocities: &Velocity, mut positions: &mut Position) {
    (&mut positions, &velocities).iter().for_each(|(pos, vel)| {
        pos.x += vel.dx;
        pos.y += vel.dy;
    });
}

fn build() -> World {
    let world = World::new();

    {
        let (mut entities, mut positions, mut velocities) =
            world.borrow::<(EntitiesMut, &mut Position, &mut Velocity)>();

        (&mut positions, &mut velocities).tight_pack();

        for i in 0..N_POS {
            if i % N_POS_PER_VEL == 0 {
                entities.add_entity(
                    (&mut positions, &mut velocities),
                    (Position { x: 0., y: 0. }, Velocity { dx: 0., dy: 0. }),
                );
            } else {
                entities.add_entity(&mut positions, Position { x: 0., y: 0. });
            }
        }
    }

    world
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(build);
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let world = build();

    b.iter(|| {
        world.run_system::<VelSys>();
    });
}
