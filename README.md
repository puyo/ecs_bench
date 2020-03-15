# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 368 µs/iter (+/- 21)      | 14 µs/iter (+/- 0)      | 678 µs/iter (+/- 25)      | 61 µs/iter (+/- 1)
 [constellation] | 289 µs/iter (+/- 15) | 8 µs/iter (+/- 0) | 514 µs/iter (+/- 16) | 127 µs/iter (+/- 4)
 [ecs]           | 1,231 µs/iter (+/- 46)           | 184 µs/iter (+/- 7)           | 1,295 µs/iter (+/- 81)           | 2,015 µs/iter (+/- 70)
 [froggy]        | 489 µs/iter (+/- 15)        | 9 µs/iter (+/- 0)        | 1,082 µs/iter (+/- 33)        | 93 µs/iter (+/- 4)
 [specs]         | 345 µs/iter (+/- 10)         | 3 µs/iter (+/- 0)         | 545 µs/iter (+/- 13)         | 44 µs/iter (+/- 1)
 [trex]          | 1,044 µs/iter (+/- 29)          | 228 µs/iter (+/- 7)          | 1,438 µs/iter (+/- 49)          | 369 µs/iter (+/- 9)
 [shipyard]      | 647 µs/iter (+/- 33)      | 2 µs/iter (+/- 0)      | 2,279 µs/iter (+/- 127)      | 7 µs/iter (+/- 1)

[calx-ecs]: https://github.com/rsaarelm/calx-ecs
[constellation]: https://github.com/TomGillen/constellation/
[ecs]: https://github.com/HeroesGrave/ecs-rs
[froggy]: https://github.com/kvark/froggy
[specs]: https://github.com/slide-rs/specs
[trex]: https://github.com/rcolinray/trex
[shipyard]: https://github.com/leudz/shipyard


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
