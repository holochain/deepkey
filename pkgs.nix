{ pkgs, system }:

import (pkgs.fetchFromGitHub {
  owner = "spartan-holochain-counsel";
  repo = "nix-overlay";
  rev = "9c0ed332596994faaacb35593a2d427f9ac38bf2";
  sha256 = "E8dKMuhnd51Rh1tLud/asf/4TTFuZaIs0fZbe/rsUuQ=";
}) {
  inherit pkgs;
  inherit system;
}
