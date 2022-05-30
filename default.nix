let
  holonixPath = builtins.fetchTarball { # main as of Mar 15, 2022
    url = "https://github.com/holochain/holonix/archive/391557dc5b3065b0d357ea9f9a2bc77e7347be8e.tar.gz";
    sha256 = "10dnbd3s8gm4bl7my7c168vyvi3358s1lb5yjnw3fwnp9z62vy09";
  };
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
      rev = "holochain-0.0.132"; # Mar 30, 2022 - b2eb2342d2feb68872e19636e83d199d38b01f66
      sha256 = "12zgx1icnq4jgra9q6bwqjhlzmm38s5kz6vidkylal11ynnd57ww";
      cargoLock = {
        outputHashes = {
        };
      };

      binsFilter = [
        "holochain"
        "hc"
        "kitsune-p2p-tx2-proxy"
      ];

      rustVersion = "1.58.1";

      lair = {
        url = "https://github.com/holochain/lair";
        rev = "v0.1.0";
        sha256 = "0jvk4dd42axwp5pawxayg2jnjx05ic0f6k8f793z8dwwwbvmqsqi";

        binsFilter = [
          "lair-keystore"
        ];

        rustVersion = "1.58.1";

        cargoLock = {
          outputHashes = {
          };
        };
      };
    };
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  buildInputs = with nixpkgs; [
    nodejs-14_x
  ];
}
