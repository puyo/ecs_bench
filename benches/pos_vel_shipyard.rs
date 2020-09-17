#![feature(test)]
extern crate test;
use test::Bencher;

extern crate ecs_bench;
extern crate shipyard;

use ecs_bench::pos_vel::*;
use shipyard::*;

fn run(velocities: View<Velocity>, mut positions: ViewMut<Position>) {
    for (vel, mut pos) in (&velocities, &mut positions).iter() {
        pos.x += vel.dx;
        pos.y += vel.dy;
    }
}

fn build() -> World {
    let world = World::new();

    world.run(
        |mut entities: EntitiesViewMut,
         mut positions: ViewMut<Position>,
         mut velocities: ViewMut<Velocity>| {
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
        },
    );

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
        world.run(run);
    });
}
