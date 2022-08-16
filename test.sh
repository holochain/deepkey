#!/usr/bin/env bash
cargo test  --verbose -j 2 --manifest-path zomes/deepkey/Cargo.toml --lib --features="mock" -- --nocapture
