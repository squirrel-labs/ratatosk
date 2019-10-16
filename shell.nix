with import (fetchTarball {
  url = https://github.com/NixOS/nixpkgs/archive/f0057b6924dadb89a837010b22ba63097534ff0e.tar.gz;
  sha256 = "19sx328s0x3wrc4j7isw8alxask9nw7ds2n6x7s6bi0675sh8jcp";
}) { };

mkShell {
  name = "ratatosk";
  buildInputs = [
    python3
    rustup
    pkgconfig
    rustc
    cargo
    openssl
    cargo-make
    wabt
    binaryen
    wasm-bindgen-cli
  ];

  shellHook = ''
    # ugly, I know
    export PATH="$PATH:$HOME/.cargo/bin"
  '';
}
