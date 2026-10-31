#!/bin/sh
# Builds the web (WebGPU) version into target/web/ and target/bevy-starter-web.zip.
# Serve the folder over HTTP, e.g.
#   python3 -m http.server -d target/web 8080
set -eu

NAME=bevy-starter
OUT=target/web
WASM=target/wasm32-unknown-unknown/web/$NAME.wasm

# wasm-bindgen-cli must match the wasm-bindgen crate version exactly.
BINDGEN_VERSION=$(awk '/^name = "wasm-bindgen"$/ { getline; gsub(/[^0-9.]/, ""); print }' Cargo.lock)
BINDGEN=target/tools/bin/wasm-bindgen
if [ "$("$BINDGEN" --version 2>/dev/null | cut -d' ' -f2)" != "$BINDGEN_VERSION" ]; then
    cargo install --locked --root target/tools wasm-bindgen-cli --version "$BINDGEN_VERSION"
fi

# The browser can't run Blender: ship pre-processed assets like native release builds.
cargo run -- --process-assets
cargo build --profile web --target wasm32-unknown-unknown

rm -rf "$OUT"
"$BINDGEN" --no-typescript --target web --out-dir "$OUT" "$WASM"

if command -v wasm-opt >/dev/null; then
    wasm-opt -Oz --enable-bulk-memory --enable-nontrapping-float-to-int \
        -o "$OUT/${NAME}_bg.wasm" "$OUT/${NAME}_bg.wasm"
else
    echo "wasm-opt not found (nix develop provides it); skipping size optimization"
fi

cp web/index.html "$OUT/"
# Skip Blender backups (.blend1, ...): the processor copies them verbatim.
rsync -a --delete --exclude='/log' --exclude='*.blend[0-9]' --exclude='*.blend[0-9].meta' \
    imported_assets/ "$OUT/imported_assets/"

# Upload-ready archive (e.g. itch.io: index.html at the top level; its CDN gzips
# .wasm/.js/.html on the fly, so no pre-compression needed).
ZIP=target/$NAME-web.zip
rm -f "$ZIP"
(cd "$OUT" && zip -qr -X "../$NAME-web.zip" .)

echo "Web build ready in $OUT/ and $ZIP"
