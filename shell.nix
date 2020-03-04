{ inCI ? false }:

with import (fetchTarball {
  url = https://github.com/NixOS/nixpkgs/archive/be346a1f4bd9bf272c1388b7791cdb0f28bfa2fb.tar.gz;
  sha256 = "1f0p8x4h5k190vizlan5yljqvsay2krn93jl3m4yqlg808yglsr3";
}) { };

mkShell {
  name = "ratatosk";
  buildInputs = [
    git
    python3
    rustup
    pkgconfig
    rustc
    cargo
    openssl
    cargo-make
    binaryen
    wasm-bindgen-cli
  ] ++ lib.optionals (!inCI) [
    cargo-audit
  ];

  shellHook = ''
    # ugly, I know
    export PATH="$PATH:$HOME/.cargo/bin"
  '';
}
