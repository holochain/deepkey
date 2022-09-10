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
      url = "https://github.com/holochain/holochain";
      rev = "holochain-0.0.158"; # Aug 31, 2022 - 0cd777c789ee13ca655f9bffbd7fe11b49e47824
      sha256 = "04zcz6mjnczrw6612a2miqs1zf274jxwpj8jblrlavgfi7qzqr92";
      cargoLock = {
        outputHashes = {
        };
      };

      binsFilter = [
        "holochain"
        "hc"
        "kitsune-p2p-tx2-proxy"
      ];

      rustVersion = "1.63.0";

      lair = {
        url = "https://github.com/holochain/lair";
        rev = "lair_keystore-v0.2.0"; # Jun 20, 2022 - 20b18781d217f172187f16a0ef86b78eb1fcd3bd
        sha256 = "1j3a8sgcg0dki65cqda2dn5wn85m8ljlvnzyglaayhvljk4xkfcz";

        binsFilter = [
          "lair-keystore"
        ];

        rustVersion = "1.63.0";

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
    sqlite  # for Rust unit tests
  ];
}
