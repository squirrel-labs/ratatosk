with import (fetchTarball {
  url = https://github.com/NixOS/nixpkgs-channels/archive/c4fec1c6314c0c9c7af59bb465a17d1950ec7464.tar.gz;
  sha256 = "1w8wjvmsap0jn4gq2gg76yphsgvl6a9v5vsnkjr0jzda1q83zw4h";
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
    binaryen
    cargo-make
    wabt
  ];

  shellHook = ''
    # ugly, I know
    export PATH="$PATH:$HOME/.cargo/bin"
  '';
}
