{ inCI ? false }:

with import (fetchTarball {
  url = https://github.com/NixOS/nixpkgs/archive/0657426ad90f0f940c5e296bd468e529cf159c6a.tar.gz;
  sha256 = "1aw0d1892nywhj9xvf5rz4l974xw0wv43z9l9bfh28rwqrah73bf";
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
