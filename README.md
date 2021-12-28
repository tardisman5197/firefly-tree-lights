# firefly-tree-lights

An implementation of Firefly Synchronization for a christmas tree display.

## Why and What?

The reason for this project is to create a lighting animation for a christmas tree, specifically [Matt Parker's Christmas Tree](https://www.youtube.com/watch?v=WuMRJf6B5Q4).

This implementation is based on the [Firefly Algorithm](https://en.wikipedia.org/wiki/Firefly_algorithm). Where each light on the tree is a Firefly assigned an initial random time to flash. Then based on the proximity of each Firefly they adjusts their next flash to be closer to that of those around it, until all of the lights are in sync.

The input file for this program is a csv with x, y, z coordinates for each firefly per line.

The output file from this program is

```
<frame_number>,<r_0>,<g_0>,<b_0>,...,<r_n>,<g_n>,<b_n>
<frame_number>,<r_0>,<g_0>,<b_0>,...,<r_n>,<g_n>,<b_n>
...
<frame_number>,<r_0>,<g_0>,<b_0>,...,<r_n>,<g_n>,<b_n>
```

Where `r_n`, `g_n`, `b_n` are the red, green, blue values of the nth Firefly in the simulation.

Various parameters of the simulation can be changed such as:

```rust
// MAX_COUNTDOWN is the interval of ticks
// between a Fly flashing.
const MAX_COUNTDOWN: i16 = 40;

// NUDGE_VALUE is the number of ticks which
// a Fly will move towards its neighbouring
// Fly's flash.
const NUDGE_VALUE: i16 = 1;

// NO_OF_NEIGHBOURS is the initial number of 
// neighbours which each Fly has. 
const NO_OF_NEIGHBOURS: usize = 5;

// NO_OF_TICKS is the maximum number of ticks
// the simulation runs for.
const NO_OF_TICKS: usize = 60000;

// SYNC_STOP is the number of fully synced flashes
// the swarm performs before stopping.
const SYNC_STOP:usize = 10;
```

## Prerequisites

* [Rust + Cargo](https://www.rust-lang.org/tools/install)

## Building

```
cargo build --release
```

This will create a binary within the `./target/release/` directory.

## Usage

```
firefly <input.csv> <output.csv>
```

For Example:

```
firefly coords.csv output.csv
```

*Running after building use this command:*

```
./target/release/firefly ./input/matts-tree.csv ./output/test.csv
```
