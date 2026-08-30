# futoshiki

My bachelor thesis: **How to Solve Futoshiki by Means of Automata**.

> [!NOTE]
> This is the submitted version of my bachelor thesis. I later made some
> changes, which are in the `master` branch

## Table of Contents

- [Running the project](#running-the-project)
- [Running on merlin server](#running-on-merlin-server)
- [Usage](#usage)
- [Detailed description](#detailed-description)
    - [What is Futoshiki?](#what-is-futoshiki)
    - [Game](#game)
    - [Benchmark](#benchmark)
        - [Running](#running)
        - [Result](#result)
    - [Config](#config)
- [Links](#links)

## Running the project

> App was tested on Linux only, other operating systems might not work.

To build the project, you need to have the Rust Toolchain installed (see
[rust installation page](https://www.rust-lang.org/tools/install)). When you
have the Rust Toochain, you can build the project with `cargo`:

```bash
cargo build -r
```

If you have issues with permissions for cargo registry cache, you can build the
project like this (this is how you have to build on merlin):

```bash
CARGO_HOME=target/cargo-home/ cargo build -r
```

After it's done compiling, the binary will be `target/release/futoshiki`.
Because the project is a console program, it needs to be run from terminal.

## Running on merlin server

The binary in the archive (`futoshiki`) can be used on merlin. To create a new
build on merlin, you need to run the project like this:

```bash
CARGO_HOME=target/cargo-home cargo build -r
```

## Usage

In all the examples, the path to the project binary is substituted by
`futoshiki`. If you want run the project, you need to use the path to the built
binary from the previous step.

You can start `futoshiki` in default size _(4×4)_ by running:

```bash
futoshiki
```

To play a game with different size and different solving algorithm, you can do
it like this:

```bash
futoshiki -s 10 --solver bt
```

All the other usage and options can be seen in the help:

```bash
./futoshiki -h
```

## Detailed description

### What is Futoshiki?

It's a board-based puzzle game which is quite similar to Sudoku. It is also
known as Unequal. It's played on the square board of any size
_(4×4 is common)_. The goal is to fill the board with missing numbers so each
row and column contains every number from 1 to board size exactly once, plus
all the inequalities must be satisfied.

### Game

Game is implemented using TUI. When you start a game, you get put into
**board solver screen**. On this screen you can solve board, generate new one
or let an algorithm to solve the board for you. To change selected cells you
can use `Arrow` keys, place number by typing the desired `digit`.

You can also switch to **board creation screen** _(by pressing `b` key)_, where
you can create your own board. You can add digits the same way as in
**board solver screen**, but you can also add conditions. To add condition, you
have to press corresponding condition _(`<` or `>`)_ and `Arrow` in which
direction the condition should be placed.

All other keybinds are listed in the help on the bottom of each page.

### Benchmark

#### Running

When running the program with **bench** argument:

```bash
./futoshiki bench --solver ac3 --solver fc -s 4 -s 5 -r 10 -b 5
```

you start a benchmark of given algorithms. Algorithms can be given using the
`--solver` flag followed by any solver type _(all solver types are displayed_
_in help)_. If no solver is provided, all algorithms are used.

You have to also provide board sizes _(`-s` flag)_ which should be tested.
Example above benchmarks boards with sizes 4 and 5.

You can also set how many times board will be tested to eliminate any
fluctuations _(`-r` flag)_ and number of boards that should be tested for each
size _(`-b` flag)_. Default value for board repeats is 10 and number of boards
is 1.

#### Result

Results are printed to the console ordered by the board size. Each board size
is also printed with all the tested algorithms ordered by their average speed
and each algorithm is printed with its best, average and the worst time.

Benchmark also generates a graf _(`benchmark.png`)_, which contains plotted
average times for each algorithm and board size.

### Config

You can also set default values in the config file, which is saved in the
`config` directory _(on linux it should be `~/.config/futoshiki/config.json`)_.
You can set a default size, solver or theme with which the app will be started
without having to use command line arguments.

You can edit the config by running:

```bash
./futoshiki config
```

which opens the `config.json` file in default editor. The config file is
created only when running the command above and config file doesn't exist
already.

> Config file is not created in any other case! You can either create it
> manually or use the command above.

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [futoshiki](https://github.com/Martan03/futoshiki)
- **Author website:** [martan03.github.io](https://martan03.github.io)
