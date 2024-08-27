{ pkgs, system }:

import (pkgs.fetchFromGitHub {
  owner = "spartan-holochain-counsel";
  repo = "nix-overlay";
  rev = "4bf90e85448392512d8bf4dac91fdeb56bc7d610";
  sha256 = "lxGLA0KMecdt6xRy9SqApDfh9UiQd9OYnwj9xeMLJcQ=";
}) {
  inherit pkgs;
  inherit system;
}
