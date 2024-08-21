{ pkgs, system }:

import (pkgs.fetchFromGitHub {
  owner = "spartan-holochain-counsel";
  repo = "nix-overlay";
  rev = "5bae4a38735d74633c9c089b5c896cb9631a295b";
  sha256 = "E/FvMyUgEB5MYL+s46YSiHPOAJiurKZDMQy1oxak6bg=";
}) {
  inherit pkgs;
  inherit system;
}
