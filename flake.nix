{
  description = "A multiplayer piano using UDP sockets that can be played using computer keyboard, in the terminal.";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        toolchain = fenix.packages.${system}.minimal.toolchain;
        pkgs = import nixpkgs { inherit system; };
        manifest = (pkgs.lib.importTOML ./Cargo.toml);
      in {

        packages.default = (pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        }).buildRustPackage rec {
          pname = manifest.package.name;
          version = manifest.package.version;
          src = pkgs.lib.cleanSource ./.;

          cargoLock.lockFile = ./Cargo.lock;
          cargoLock.allowBuiltinFetchGit = true;
          buildType = "release";
          nativeBuildInputs = with pkgs; [
            makeWrapper
            pkg-config
          ];
          buildInputs = with pkgs; [
            python310
            alsa-lib
          ];
          postInstall = ''
            mkdir -p "$out"/lib
            install -Dm644 "${pkgs.alsa-lib.out}"/lib/libasound* "$out"/lib/

            cp -r assets "$out"/
            wrapProgram "$out"/bin/piano-rs --set ASSETS "$out"/assets
          '';
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          buildInputs = with pkgs; [
            python310
            alsa-lib
          ];
        };
      }
    );
}
