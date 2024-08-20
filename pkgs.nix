{ pkgs, system }:

import (pkgs.fetchFromGitHub {
  owner = "spartan-holochain-counsel";
  repo = "nix-overlay";
  rev = "09a503d2a04909df03433b9cf0ec53a8c23699af";
  sha256 = "sdX6kksfoHRXvvkwktll36nwNI91QERDt4h6X6gZxIM=";
}) {
  inherit pkgs;
  inherit system;
}
