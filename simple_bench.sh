#!/usr/bin/env bash
cargo b -r &&\
hyperfine "./target/release/wt_ext_cli unpack_vromf -i "/home/flareflo/CLionProjects/wt_ext_cli/test_data/all_vromfs/aces.vromfs.bin" -o /tmp"