let
  pkgs = import (builtins.fetchTarball {
    # Pinned to `master` on 20th July, 2024.
    # https://github.com/NixOS/nixpkgs/tree/e95cc1274981e089d793531efab9c383915edad0
    url = https://github.com/NixOS/nixpkgs/archive/e95cc1274981e089d793531efab9c383915edad0.tar.gz;
    sha256 = "sha256:15x779wpbgm2a5xr6zh8cl4gwr3j9xgrma30igsn33szs933lzh0";
  }) {};
in pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    # Sticking to python3.10 as later versions break
    # termbox during compilation.
    python310
    alsa-lib
    pkg-config
  ];
}
