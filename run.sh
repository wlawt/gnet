#!/bin/bash
set -e
cargo build --bin gnet && RUST_LOG=info cargo run --bin gnet