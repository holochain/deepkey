let
  holonixPath = (import ./nix/sources.nix).holonix;
  holonix = import (holonixPath) {
   include = {
      holochainBinaries = true;
      node = false;
      scaffolding = false;
      happs = false;
    };

    holochainVersionId = "custom";
    holochainVersion = {
      url = "https://github.com/pjkundert/holochain";
      rev = "914920835a36203961433ea2f305b8b0582b363a"; # Aug 1, 2022 - must-get-agent-activity-host-fn w/ ChainFilter, must_get_agent_activity
      sha256 = "13gil2nnc4bry6wqbblznp399g4nm6fv0hpv63c0pvfqbbkhkjw4";
      cargoLock = {
        outputHashes = {
        };
      };

      binsFilter = [
        "holochain"
        "hc"
        "kitsune-p2p-tx2-proxy"
      ];

      rustVersion = "1.59.0";

      lair = {
        url = "https://github.com/holochain/lair";
        rev = "lair_keystore-v0.2.0"; # Jun 20, 2022 - 20b18781d217f172187f16a0ef86b78eb1fcd3bd
        sha256 = "1j3a8sgcg0dki65cqda2dn5wn85m8ljlvnzyglaayhvljk4xkfcz";

        binsFilter = [
          "lair-keystore"
        ];

        rustVersion = "1.59.0";

        cargoLock = {
          outputHashes = {
          };
        };
      };
    };
  };

  nixpkgs = holonix.pkgs;
in
nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    nodejs-16_x
  ];
}
