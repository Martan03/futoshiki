# futoshiki

Futoshiki TUI implementation in Rust 🦀

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Detailed description](#detailed-description)
    - [What is Futoshiki?](#what-is-futoshiki)
    - [Game](#game)
    - [Benchmark](#benchmark)
        - [Running](#running)
        - [Result](#result)
    - [Config](#config)
- [Links](#links)

## Installation

You have to compile it yourself, but that shouldn't be a problem. Only thing
you need is [cargo](https://www.rust-lang.org/tools/install). You need to go to
the futoshiki project folder and run:

```bash
cargo build -r
```

After it's done compiling, you can start it in `./target/release/futoshiki`.

## Usage

You can start `futoshiki` in default size *(4×4)* by running:
```bash
./futoshiki
```

To play a game with different size, you can do it like this:
```bash
./futoshiki -s 10
```

All the usage and options can be seen in the help:
```bash
./futoshiki -h
```

## Detailed description

### What is Futoshiki?

It's a board-based puzzle game which is quite similar to Sudoku. It is also
known as Unequal. It's played on the square board of any size
*(4×4 is common)*. The goal is to fill the board with missing numbers so each
row and column contains every number from 1 to board size exactly once, plus
all the inequalities must be satisfied.

### Game

Game is implemented using TUI. When you start a game, you get put into game
screen in **solver** mode. In this mode you can solve board, generate new once
or let an algorithm to solve the board for you. To change selected cells you
can use `Arrow` keys, place number by typing the desired `digit`.

You can also switch to **builder** mode *(by pressing `b` key)*, where you can
create your own board. You can add digits the same way as in **solver** mode,
but you can also add conditions. To add condition, you have to press
corresponding condition *(`<` or `>`)* and `Arrow` in which direction the
condition should be placed.

All other keybinds are listed in the help on the bottom of each page.

### Benchmark

#### Running

When running the program with **bench** argument:
```bash
./futoshiki bench --solver ac3 --solver fc -s 4 -s 5 -r 10 -b 5
```

you start a benchmark of given algorithms. Algorithms can be given using the
`--solver` flag followed by any solver type *(all solver types are displayed*
*in help)*. If no solver is provided, all algorithms are used.

You have to also provide board sizes *(`-s` flag)* which should be tested.
Example above benchmarks boards with sizes 4 and 5.

You can also set how many times board will be tested to eliminate any
fluctuations *(`-r` flag)* and number of boards that should be tested for each
size *(`-b` flag)*. Default value for board repeats is 10 and number of boards
is 1.

#### Result

Results are printed to the console ordered by the board size. Each board size
is also printed with all the tested algorithms ordered by their average speed
and each algorithm is printed with its best, average and the worst time.

Benchmark also generates a graf *(`benchmark.png`)*, which contains plotted
average times for each algorithm and board size.

### Config

You can also set default values in the config file, which is saved in the
`config` directory *(on linux it should be `~/.config/futoshiki/config.json`)*.
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
