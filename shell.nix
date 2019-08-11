with import (fetchTarball {
  url = https://github.com/NixOS/nixpkgs-channels/archive/4557b9f1f50aa813ae673fe6fcd30ca872968947.tar.gz;
  sha256 = "0cam48cn042axcik9vqxsqjc2hwyb2grjbjxacsn4w0y1zk6k6l2";
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
    alias make="cargo make"
  '';
}
