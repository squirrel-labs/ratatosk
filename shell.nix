with import (fetchTarball {
  url = https://github.com/NixOS/nixpkgs-channels/archive/239fffc90d792b5362a20ec1a009978de7b8f91a.tar.gz;
  sha256 = "0z0c438b5q1066x6p9qfriym5cipw8f52a456f91qzg6q1r296f8";
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
