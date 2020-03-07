#![feature(test)]
extern crate test;
use test::Bencher;

extern crate ecs_bench;
extern crate rayon;
extern crate shipyard;

use rayon::iter::ParallelIterator;
use shipyard::prelude::*;

use ecs_bench::parallel::*;

#[system(Sys1)]
fn run(mut w1s: &mut W1, rs: &R) {
    (&mut w1s, &rs).par_iter().for_each(|(w1, r)| {
        w1.x = r.x;
    });
}

#[system(Sys2)]
fn run(mut w2s: &mut W2, rs: &R) {
    (&mut w2s, &rs).par_iter().for_each(|(w2, r)| {
        w2.x = r.x;
    })
}

fn build() -> World {
    let world = World::new();

    {
        let (mut entities, mut rs, mut w1s, mut w2s) =
            world.borrow::<(EntitiesMut, &mut R, &mut W1, &mut W2)>();

        (&mut w1s, &mut rs).tight_pack();
        (&mut w2s, &mut rs).loose_pack();

        for _ in 0..N {
            entities.add_entity(
                (&mut rs, &mut w1s, &mut w2s),
                (R { x: 0. }, W1 { x: 0. }, W2 { x: 0. }),
            );
        }
    }

    world.add_workload::<(Sys1, Sys2), _>("Bench");

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
        world.run_default();
    })
}
