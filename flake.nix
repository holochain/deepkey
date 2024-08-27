{
  description = "Holochain Development Env";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import ./pkgs.nix {
          pkgs = nixpkgs.legacyPackages.${system};
          inherit system;
        };
      in
      {
        devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              holochain_0-4
              lair-keystore_0-5
              hc_0-4

              rustup
              cargo
              rustc

              nodejs_22
            ];

            shellHook = ''
              export PS1="\[\e[1;32m\](flake-env)\[\e[0m\] \[\e[1;34m\]\u@\h:\w\[\e[0m\]$ "
              export CARGO_HOME=$(pwd)/.cargo
              export RUSTUP_HOME=$(pwd)/.rustup
              rustup default stable
              rustup target add wasm32-unknown-unknown
            '';
        };
      }
    );
}
