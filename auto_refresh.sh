#!/usr/bin/env bash

trap 'kill %1; kill $pid2; exit' SIGINT

touch .trigger
cargo watch -i .trigger -i .vscode -i .git -i target -i perf.data --postpone -q -x build -s 'touch .trigger' &
cargo watch -w .trigger -q -c -s 'CARGO_MANIFEST_DIR=/home/johnny/Projects/personal/stroid-lazer/ RUST_LOG=info,wgpu=warn,stroid_lazer=debug target/debug/stroid-lazer ' &
pid2=$!

wait
