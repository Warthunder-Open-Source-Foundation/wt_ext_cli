# wt_ext_cli
>**Download, extract and transform in-game files**

Licensed under the [Apache 2.0](https://github.com/Warthunder-Open-Source-Foundation/wt_blk/blob/master/LICENSE) license

## Installing
### Easy way:
- [Download from the latest stable-release](https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/releases?q=prerelease:false)
- [Download from the latest pre-release](https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/releases?q=prerelease:true)

### From source
#### Building the project from git
1. <a href="https://www.rust-lang.org/tools/install">Install a working Rust-toolchain through Rustup</a>
2. Clone the repository  using `git clone` and enter the directory `cd wt_ext_cli` ([requires git](https://github.com/git-guides/install-git))
3. Depending on the goal, do the following:
   | Goal                        | Command                          |
   |-----------------------------|----------------------------------|
   | Build a portable executable | `cargo build --release` binary can be found in `target/release/wt_ext_cli(.exe)`|
   | Install the tool locally    | `cargo install --profile release --path .`|
   | Run the tool directly       | `cargo run --release -- {FLAGS}` replace FLAGS with CLI args|

#### Building the project from crates-io
>Currently not supported, as the tool is work in progress. It will be published once stabilized to a certain degree.

## Usage
For usage, view [this guide](https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/blob/master/usage_manual.md)
