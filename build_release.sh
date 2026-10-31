#!/bin/sh
set -eu
cargo run -- --process-assets
cargo build --profile dist
# skip Blender backups (.blend1, ...): the processor copies them verbatim.
rsync -a --delete --delete-excluded \
    --exclude='*.blend[0-9]' --exclude='*.blend[0-9].meta' \
    imported_assets/ target/dist/imported_assets/
echo "Release build ready in target/dist/"
