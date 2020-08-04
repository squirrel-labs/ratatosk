{ inCI ? false }:

with import (fetchTarball {
  url = https://github.com/NixOS/nixpkgs/archive/d971fd7cbaa7794b4cb632ad17ecbfbe3c17f8ee.tar.gz;
  sha256 = "19p52y2s9m7hc0z5ngjwj5dy6fm9yz2bn8fxsa0brbfwpw2xbl56";
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
  ] ++ lib.optionals (!inCI) [
    cargo-audit
    wabt
  ];
}
