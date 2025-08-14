# wt_ext_cli

> **Extract and transform BLK and VROMF archives**

[![LOC](https://tokei.rs/b1/github/Warthunder-Open-Source-Foundation/wt_ext_cli?type=Rust,Markdown,Javascript,Python,Shell)](https://github.com/search?q=repo%3AWarthunder-Open-Source-Foundation%2Fwt_ext_cli++language%3ARust&type=code)

## How to use
[View the manual shipped with every release](https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/releases/latest/download/MANUAL.md)

## Used By

| Project                                                                          | Name                | URL                                                           |
|----------------------------------------------------------------------------------|---------------------|---------------------------------------------------------------|
| <img src="https://statshark.net/assets/icon.svg" width="48">                     | Statshark           | [View Website](https://statshark.net/)                        |
| <img src="https://avatars.githubusercontent.com/u/45767091?v=4" width="48">      | gszabi99            | [View Repo](https://github.com/gszabi99/War-Thunder-Datamine) |
| <img src="https://avatars.githubusercontent.com/u/49201354?v=4" width="48">      | GUI Toolkit         | [View Repo](https://github.com/axiangcoding/WT-Toolkit)       |
| <img src="https://avatars.githubusercontent.com/u/80386324?s=48&v=4" width="48"> | WT Heatmaps project | [View Repo](https://github.com/Sgambe33/WT-Plotter)           |
| <img src="" width="48">                                                          | Thunder View        | [View Website](https://thunderview.net/)                      |

This list is not exhaustive, suggest any addition through a mail or issue!
<!--|<img src="" width="48"> |  | [View Repo]() | -->

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
