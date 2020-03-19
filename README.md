# Rask [![Build Status](https://ci.kobert.dev/api/badges/squirrel-labs/ratatosk/status.svg)](https://ci.kobert.dev/squirrel-labs/ratatosk)

You might have heard of a game called [Nidhogg](https://github.com/TrueDoctor/ratatosk/wiki/Nidhogg). The aim of this
project is to provide a similar game accessible through the web.

## Implementation details

The game is written in the programming language [Rust](https://doc.rust-lang.org/book/). On the frontend we use [Web Assembly](https://developer.mozilla.org/en-US/docs/WebAssembly) so we can also use Rust on the frontend.

## Wiki

For documentation and overview purposes we have a
**[Wiki](https://github.com/TrueDoctor/ratatosk/wiki)**.

## Setup

This is a simple recipe on how to get `ratatosk` working locally. Further details can be
found in Wiki.

### Prerequisites

* [`rustup` and the Rust toolchain](https://rustup.rs/)
* [`binaryen`](https://github.com/WebAssembly/binaryen) (at least version [89](https://github.com/WebAssembly/binaryen/releases/tag/version_89), `emscripten` is currently not used)
* [`python3`](https://www.python.org/) for the [exemplary web server](https://github.com/TrueDoctor/ratatosk/wiki/Frontend#installation).
* [`cargo-make`](https://github.com/sagiegurari/cargo-make) for all build scripts.
* [`wabt`](https://github.com/WebAssembly/wabt) to introspect compiled wasm code.
* [`pkg-config`](https://www.freedesktop.org/wiki/Software/pkg-config/) to find additional libraries.
* [`libopenssl`](https://www.archlinux.org/packages/core/x86_64/openssl/)

Optionally, all dependencies can be obtained with [Nix](https://nixos.org/nix/) by running
[`nix-shell`](https://nixos.org/nixos/nix-pills/developing-with-nix-shell.html) in the project's root.

### Install and build

First of all, you need to install the `nightly` toolchain using rustup:

```
rustup install nightly-2020-02-28
rustup default nightly-2020-02-28
rustup target add wasm32-unknown-unknown  # for the frontend part
cargo install cargo-make --git git://github.com/Ma27/cargo-make --branch loglevel
cargo install wasm-bindgen-cli  # for optimizing wasm code
```

Please note that we explicitly pin the nightly channel to a certain date to ensure that
each developer uses the same toolchain which helps reproducing compiler issues on multiple setups.

Now you can build the project with [`cargo`](https://doc.rust-lang.org/cargo/):

```
cargo make all
```

The following scripts can be used to start test servers

```
cargo make serve-frontend # for the wasm/frontend setup
cargo make serve-backend  # for the backend/game_server
```
