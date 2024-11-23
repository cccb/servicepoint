#!/usr/bin/env bash
set -e

cargo build --release

SCRIPT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
TARGET_PATH="$(realpath "$SCRIPT_PATH"/../../target/release)"
SERVICEPOINT_SO="$TARGET_PATH/libservicepoint_binding_uniffi.so"
LIBRARIES_PATH="$SCRIPT_PATH/libraries"

echo "Source: $SERVICEPOINT_SO"
echo "Output: $LIBRARIES_PATH"

BINDGEN="cargo run --features=uniffi/cli --bin uniffi-bindgen -- "
BINDGEN_CS="cargo run --features=cs --bin uniffi-bindgen-cs -- "
BINDGEN_GO="cargo run --features=go --bin uniffi-bindgen-go -- "
COMMON_ARGS="--library $SERVICEPOINT_SO"

${BINDGEN} generate $COMMON_ARGS --language python --out-dir "$LIBRARIES_PATH/python"
${BINDGEN} generate $COMMON_ARGS --language kotlin --out-dir "$LIBRARIES_PATH/kotlin"
${BINDGEN} generate $COMMON_ARGS --language swift --out-dir "$LIBRARIES_PATH/swift"
${BINDGEN} generate $COMMON_ARGS --language ruby --out-dir "$LIBRARIES_PATH/ruby/lib"
${BINDGEN_CS} $COMMON_ARGS --out-dir "$LIBRARIES_PATH/csharp/ServicePoint"
${BINDGEN_GO} $COMMON_ARGS --out-dir "$LIBRARIES_PATH/go/"
