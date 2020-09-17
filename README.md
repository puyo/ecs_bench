# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 404 µs/iter (+/- 128)      | 19 µs/iter (+/- 0)      | 458 µs/iter (+/- 36)      | 60 µs/iter (+/- 1)
 [constellation] | 388 µs/iter (+/- 7) | 12 µs/iter (+/- 0) | 495 µs/iter (+/- 23) | 183 µs/iter (+/- 84)
 [ecs]           | {pos_vel_build_ecs}           | {pos_vel_update_ecs}           | {parallel_build_ecs}           | {parallel_update_ecs}
 [froggy]        | 451 µs/iter (+/- 2)        | 24 µs/iter (+/- 1)        | 763 µs/iter (+/- 27)        | 55 µs/iter (+/- 42)
 [specs]         | 258 µs/iter (+/- 3)         | 3 µs/iter (+/- 0)         | 1,129 µs/iter (+/- 32)         | 46 µs/iter (+/- 91)
 [trex]          | 1,686 µs/iter (+/- 65)          | 196 µs/iter (+/- 9)          | 3,076 µs/iter (+/- 32)          | 651 µs/iter (+/- 4)
 [shipyard]      | 647 µs/iter (+/- 19)      | 11 µs/iter (+/- 0)      | 2,024 µs/iter (+/- 460)      | 211 µs/iter (+/- 3)
 [legion]        | 570 µs/iter (+/- 8)        | 2 µs/iter (+/- 0)        | 771 µs/iter (+/- 24)        | 43 µs/iter (+/- 10)

[calx-ecs]: https://github.com/rsaarelm/calx-ecs
[constellation]: https://github.com/TomGillen/constellation/
[ecs]: https://github.com/HeroesGrave/ecs-rs/
[froggy]: https://github.com/kvark/froggy/
[specs]: https://github.com/slide-rs/specs/
[trex]: https://github.com/rcolinray/trex/
[shipyard]: https://github.com/leudz/shipyard/
[legion]: https://github.com/amethyst/legion/


Visualization of benchmarks, smaller is better.
![update benchmarks graph](./graph/update.png)
![build benchmarks graph](./graph/build.png)

### pos_vel
 * 1000 entities with `position` and `velocity` components
 * 9000 entities with `position` components only
 * stub `render` system
 * `physics` system: `position += velocity`

### parallel
 * 10000 entities with 3 simple components `R`, `W1` and `W2`
 * `w1` system reads `R` and writes to `W1`
 * `w2` system reads `R` and writes to `W2`
 * systems could be run in parallel

## Notes
 * the benchmarks explore a limited subset of ECS use-cases and do not necessarily reflect the peformance of large-scale applications
 * [froggy](https://github.com/kvark/froggy) is technically not an ECS, but a Component Graph System (CGS)
