{ inCI ? false }:

with import (fetchTarball {
  url = https://github.com/NixOS/nixpkgs/archive/7a7952bce6c1e11a18831189ce4a97642013bf03.tar.gz;
  sha256 = "17g76bkjh2dxwx4nbfksaydd4wkrcisa9sjfq1dhq11dzn7z2yh4";
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
}
