{ inCI ? false }:

with import (fetchTarball {
  url = https://github.com/NixOS/nixpkgs/archive/7f46b19d767b9e085c4ee656ed2ee3f25e35e0af.tar.gz;
  sha256 = "17mg4g3zqfgcj3v7xks48vfkpifhrq9w917n0ps3r95i1yyqjxz1";
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
    (binaryen.overrideAttrs (lib.const rec {
      name = "binaryen-${version}";
      version = "100";
      src = fetchFromGitHub {
        owner = "WebAssembly";
        repo = "binaryen";
        rev = "version_${version}";
        sha256 = "sha256-FgvOy8G6Yb0Zy75KoRK/TaUGiojXfT+KN4+DSFfcuFM=";
      };
      patches = [];
    }))
  ] ++ lib.optionals (!inCI) [
    cargo-audit
    wabt
  ];
}
