#!/usr/bin/env sh

case $1 in
    ("")
        if rustup run stable cargo --color always build; then
            echo build success!
            RUST_LOG=debug target/debug/game-server
        else
            echo build failed!
        fi
        ;;
    -r)
        sh build.sh &> err && cat err | tac
        ;;
    -c)
        rustup run stable cargo clean
        ;;
    *)
        echo invalid argument
        ;;
esac
