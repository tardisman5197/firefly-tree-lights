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

Various parameters of the simulation can be changed (*See Usage*).

## Prerequisites

* [Rust + Cargo](https://www.rust-lang.org/tools/install)

## Building

```
cargo build --release
```

This will create a binary within the `./target/release/` directory.

## Usage

```
USAGE:
    firefly [FLAGS] [OPTIONS] --input <FILE> --output <FILE>

FLAGS:
    -r, --colour     Use calculated red/green colours for a Fly's flash
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Logs with verbose messages

OPTIONS:
    -n, --neighbours <NO OF NEIGHBOURS>    The initial number of neighbours each Firefly starts with
    -i, --input <FILE>                     The path to the input csv
    -c, --countdown <NO OF TICKS>          The number of ticks between a Firefly's flash
    -t, --ticks <NO OF TICKS>              The maximum number of ticks the simulation can run for
    -n, --nudge <NO OF TICKS>              The number of ticks that a Firefly can be nudged closer to other Fireflies
    -o, --output <FILE>                    The path to where the output csv should be stored
    -s, --sync <NO OF SYNCS>               The number of totally synced flashes to execute before stopping the
                                           simulation
```

*Running after building use this command:*

```
./target/release/firefly -i ./input/matts-tree.csv -o ./output/test.csv
```
