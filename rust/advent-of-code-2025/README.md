# Advent of Code 2025

## Run

`cargo run -p day-XX --bin partX`

## Test

`cargo test -p day-XX partX`

## Benchmark performance

`cargo bench --bench day-XX-bench`

## Benchmarks
```
day_01_bench  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part1      62.56 µs      │ 149.1 µs      │ 63.16 µs      │ 67.26 µs      │ 100     │ 100
╰─ part2      966.9 µs      │ 1.084 ms      │ 988.5 µs      │ 993 µs        │ 100     │ 100

day_02_bench  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part1      27.91 ms      │ 30.54 ms      │ 28.09 ms      │ 28.17 ms      │ 100     │ 100
╰─ part2      786.9 ms      │ 846.9 ms      │ 805.8 ms      │ 814.6 ms      │ 100     │ 100

day_03_bench  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part1      59.28 µs      │ 579.9 µs      │ 60.53 µs      │ 78.43 µs      │ 100     │ 100
╰─ part2      145.3 µs      │ 209.8 µs      │ 151.8 µs      │ 152.8 µs      │ 100     │ 100

day_04_bench  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part1      253.2 µs      │ 404.8 µs      │ 263.2 µs      │ 273.2 µs      │ 100     │ 100
╰─ part2      1.343 ms      │ 2.667 ms      │ 1.415 ms      │ 1.509 ms      │ 100     │ 100

day_05_bench  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part1      78.84 µs      │ 133.7 µs      │ 80.17 µs      │ 83.59 µs      │ 100     │ 100
╰─ part2      23.24 µs      │ 32.51 µs      │ 23.56 µs      │ 24.04 µs      │ 100     │ 100

day_06_bench  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part1      32.29 µs      │ 82.15 µs      │ 32.54 µs      │ 35.35 µs      │ 100     │ 100
╰─ part2      1.783 ms      │ 2.009 ms      │ 1.818 ms      │ 1.826 ms      │ 100     │ 100

day_07_bench  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part1      237.8 µs      │ 1.572 ms      │ 245.1 µs      │ 262.9 µs      │ 100     │ 100
╰─ part2      258.9 µs      │ 279.3 µs      │ 261.7 µs      │ 262.5 µs      │ 100     │ 100
```
