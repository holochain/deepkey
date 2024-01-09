{
  description = "Holochain Development Env";

  inputs = {
    nixpkgs.follows = "holochain-flake/nixpkgs";
    flake-parts.follows = "holochain-flake/flake-parts";
    holochain-nix-versions.url  = "github:holochain/holochain/?dir=versions/0_2";

    holochain-flake = {
      url = "github:holochain/holochain";
      inputs.holochain.url = "github:holochain/holochain/holochain-0.2.2";
      inputs.lair.url = "github:holochain/lair/lair_keystore-v0.3.0";
    };
  };

  outputs = inputs @ { ... }:
    inputs.holochain-flake.inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = builtins.attrNames inputs.holochain-flake.devShells;
        perSystem =
          { config
          , pkgs
          , system
          , ...
          }: {
            devShells.default = pkgs.mkShell {
              inputsFrom = [ inputs.holochain-flake.devShells.${system}.holonix ];
              packages = with pkgs; [
                nodejs-18_x
              ];
            };
          };
      };
}
