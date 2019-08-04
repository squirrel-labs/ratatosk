# Rask [![Build Status](https://jenkins.kobert.dev/buildStatus/icon?job=Ratatosk)](https://jenkins.kobert.dev/job/Ratatosk/) [![Test Status](https://jenkins.kobert.dev/buildStatus/icon?job=Test&subject=tests)](https://jenkins.kobert.dev/job/Test/) [![Format Status](https://jenkins.kobert.dev/buildStatus/icon?job=Format&subject=format)](https://jenkins.kobert.dev/job/Format/) 

## Motivation

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
* [`binaryen`](https://github.com/WebAssembly/binaryen) (at least version [84](https://github.com/WebAssembly/binaryen/releases/tag/version_84), `emscripten` is currently not used)
* [`python3`](https://www.python.org/) for the [exemplary web server](https://github.com/TrueDoctor/ratatosk/wiki/Frontend#installation).
* [`cargo-make`](https://github.com/sagiegurari/cargo-make) for all build scripts.
* [`wabt`](https://github.com/WebAssembly/wabt) to introspect compiled wasm code.

Optionally, all dependencies can be obtained with [Nix](https://nixos.org/nix/) by running
[`nix-shell`](https://nixos.org/nixos/nix-pills/developing-with-nix-shell.html) in the project's root.

### Install and build

First of all, you need to install the `nightly` toolchain using rustup:

```
rustup install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown # for the frontend part
```

Now you can build the project with [`cargo`](https://doc.rust-lang.org/cargo/):

```
cargo build
```
