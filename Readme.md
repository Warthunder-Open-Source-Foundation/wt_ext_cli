# wt_ext_cli

> **Download, extract and transform in-game files**

## Installing

### Easy way:

- [Download from the latest stable-release](https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/releases?q=prerelease:false)

### Hard way (from source):

#### Building the project from git

1. <a href="https://www.rust-lang.org/tools/install">Install a working Rust-toolchain through Rustup</a>
2. Clone the repository using `git clone` and enter the directory
   `cd wt_ext_cli` ([requires git](https://github.com/git-guides/install-git))
3. Depending on the goal, do the following:
   | Goal | Command |
   |-----------------------------|----------------------------------|
   | Build a portable executable | `cargo build --release` binary can be found in `target/release/wt_ext_cli(.exe)`|
   | Install the tool locally (added to path as wt_ext_cli)    | `cargo install --profile release --path .`|
   | Run the tool directly | `cargo run --release -- {FLAGS}` replace FLAGS with CLI args|

## For lesser experienced users

The GUI toolkit provides a simple graphical interface for anyone not as confident with the commandline.  
Its repository and subsequent installation instructions are here: https://github.com/axiangcoding/WT-Toolkit

#### Building the project from crates-io

Currently a non-goal, as publishing git-dependecy binaries is not possible.

## Usage

For usage,
view [this guide](https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/releases/latest/download/MANUAL.md)

Licensed under the [Apache 2.0](https://github.com/Warthunder-Open-Source-Foundation/wt_blk/blob/master/LICENSE) license
