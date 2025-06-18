#!/usr/bin/env bash

RUST_LOG=debug cargo run -q -- main &> out_main
RUST_LOG=debug cargo run -q -- spawn &> out_spawn
