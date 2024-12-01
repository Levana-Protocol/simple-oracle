#!/usr/bin/env bash

set -euxo pipefail

SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH"
cd ..

WASM_DIR="$(pwd)/wasm"
TARGET_CACHE="$WASM_DIR/target"
REGISTRY_CACHE="$WASM_DIR/registry"
CARGO_GIT_CACHE="$WASM_DIR/git"
ARTIFACTS="$WASM_DIR/artifacts"

COSMWASM_OPTIMIZER_IMAGE="0.16.1"

if [[ -n "${OPTIMIZER_ARM64:-}" ]]; then
    OPTIMIZER_VERSION="cosmwasm/optimizer-arm64":"$COSMWASM_OPTIMIZER_IMAGE"
else
    OPTIMIZER_VERSION="cosmwasm/optimizer":"$COSMWASM_OPTIMIZER_IMAGE"
fi

mkdir -p "$TARGET_CACHE" "$REGISTRY_CACHE" "$ARTIFACTS" "$CARGO_GIT_CACHE"

# Delete the old file to avoid false positives if the compilation fails
rm -f "$WASM_DIR/artifacts/gitrev"

docker  run --rm --tty \
-u "$(id -u)":"$(id -g)" \
-v "$(pwd)":/code \
-v "$TARGET_CACHE":/target \
-v "$ARTIFACTS":/code/artifacts \
-v "$REGISTRY_CACHE":/usr/local/cargo/registry \
-v "$CARGO_GIT_CACHE":/usr/local/cargo/git \
$OPTIMIZER_VERSION

# not sure how this was created since we mapped the tool's /code/artifacts
# but it's empty (the real artifacts are in wasm/artifacts)
rm -rf ./artifacts

# Only write the gitrev file on success
git rev-parse HEAD > "$WASM_DIR/artifacts/gitrev"