#!/usr/bin/env sh

rustup run nightly cargo build
RUST_LOG=trace target/debug/game-server
