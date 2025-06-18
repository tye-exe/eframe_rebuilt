#!/usr/bin/env bash

RUST_LOG=debug cargo run -- main &> out_main
RUST_LOG=debug cargo run -- spawn &> out_spawn
