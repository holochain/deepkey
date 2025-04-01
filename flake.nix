{
  description = "Flake for Holochain app development";

  inputs = {
    holonix = {
      url = "github:holochain/holonix?ref=support-integrate-k2";
      inputs.holochain.url = "github:holochain/holochain?ref=feat/integrate-k2";
    };

    nixpkgs.follows = "holonix/nixpkgs";
    flake-parts.follows = "holonix/flake-parts";
  };

  outputs = inputs@{ flake-parts, ... }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = builtins.attrNames inputs.holonix.devShells;
    perSystem = { inputs', pkgs, ... }:
      let
        # Disable default features and enable wasmer_wamr for a wasm interpreter,
        # as well as re-enabling tx5 and sqlite-encrypted.
        cargoExtraArgs = "--features unstable-dpki";
        # Override arguments passed in to Holochain build with above feature arguments.
        customHolochain = inputs'.holonix.packages.holochain.override { inherit cargoExtraArgs; };
      in
      {
        formatter = pkgs.nixpkgs-fmt;

        devShells.default = pkgs.mkShell {
          packages = [
            # Include custom build of Holochain in dev shell.
            customHolochain
          ]
          ++ (with inputs'.holonix.packages; [
            holochain
            lair-keystore
            rust
          ]) ++ (with pkgs; [
            nodejs_22
          ]);

          shellHook = ''
            export PS1='\[\033[1;34m\][holonix:\w]\$\[\033[0m\] '
          '';
        };
      };
  };
}
