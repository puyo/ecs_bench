#![feature(test)]
extern crate test;
use test::Bencher;

extern crate ecs_bench;
extern crate shipyard;

use shipyard::*;

use ecs_bench::parallel::*;

fn sys1(mut w1s: ViewMut<W1>, rs: View<R>) {
    for (mut w1, r) in (&mut w1s, &rs).iter() {
        w1.x = r.x;
    }
}

fn sys2(mut w2s: ViewMut<W1>, rs: View<R>) {
    for (mut w2, r) in (&mut w2s, &rs).iter() {
        w2.x = r.x;
    }
}

fn build() -> World {
    let world = World::new();

    world.run(
        |mut entities: EntitiesViewMut,
         mut w1s: ViewMut<W1>,
         mut w2s: ViewMut<W2>,
         mut rs: ViewMut<R>| {
            for _ in 0..N {
                entities.add_entity(
                    (&mut rs, &mut w1s, &mut w2s),
                    (R { x: 0. }, W1 { x: 0. }, W2 { x: 0. }),
                );
            }
        },
    );

    world
        .add_workload("Bench")
        .with_system(system!(sys1))
        .with_system(system!(sys2))
        .build();

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
        world.run_workload("Bench");
    })
}
