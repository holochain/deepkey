#!/bin/sh

set -x

exec nix-shell --pure https://github.com/holochain/holochain-nixpkgs/archive/develop.tar.gz \
  --arg flavors '["release"]' \
  --run "update-holochain-versions --git-src=${1:-branch:develop}  --output-file=holochain_version.nix"
