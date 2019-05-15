#!/usr/bin/env sh

case $1 in
    ("")
        if rustup run nightly cargo --color always build; then
            echo build success!
            RUST_LOG=info target/debug/game-server
        else
            echo build failed!
        fi
        ;;
    -r)
        sh build.sh &> err && cat err | tac
        ;;
    -c)
        rustup run nightly cargo clean
        ;;
    *)
        echo invalid argument
        ;;
esac
