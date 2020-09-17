# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 237 µs/iter (+/- 55)      | 12 µs/iter (+/- 0)      | 397 µs/iter (+/- 52)      | 52 µs/iter (+/- 23)
 [constellation] | 245 µs/iter (+/- 19) | 8 µs/iter (+/- 0) | 423 µs/iter (+/- 58) | 140 µs/iter (+/- 19)
 [ecs]           | 873 µs/iter (+/- 60)           | 209 µs/iter (+/- 20)           | 963 µs/iter (+/- 98)           | 2,458 µs/iter (+/- 1,277)
 [froggy]        | 281 µs/iter (+/- 27)        | 16 µs/iter (+/- 1)        | 654 µs/iter (+/- 70)        | 49 µs/iter (+/- 7)
 [specs]         | 239 µs/iter (+/- 3)         | 3 µs/iter (+/- 0)         | 910 µs/iter (+/- 424)         | 40 µs/iter (+/- 15)
 [trex]          | 1,503 µs/iter (+/- 74)          | 173 µs/iter (+/- 3)          | 1,898 µs/iter (+/- 148)          | 419 µs/iter (+/- 47)
 [shipyard]      | 675 µs/iter (+/- 28)      | 10 µs/iter (+/- 0)      | 1,710 µs/iter (+/- 212)      | 176 µs/iter (+/- 14)
 [legion]        | 396 µs/iter (+/- 54)        | 1 µs/iter (+/- 0)        | 719 µs/iter (+/- 32)        | 48 µs/iter (+/- 5)

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
