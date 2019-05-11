#!/usr/bin/env sh

if rustup run nightly cargo build; then
    echo build success!
    RUST_LOG=trace target/debug/game-server
else
    echo build failed!
fi
