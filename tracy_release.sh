#!/bin/sh
set -eu
# Own process group, so the trap also kills tracy, not just the nix wrapper.
set -m
nix develop --command tracy -a 127.0.0.1 &
tracy_pgid=$!
set +m
trap 'kill -- "-$tracy_pgid" 2>/dev/null || true' EXIT
cargo run --release --features tracy
