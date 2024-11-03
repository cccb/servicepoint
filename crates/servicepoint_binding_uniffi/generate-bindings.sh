#!/usr/bin/env bash

set -x
set -e

cargo build --release

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
TARGETPATH="$(realpath $SCRIPTPATH/../../target/release/)"
SERVICEPOINT_SO="$TARGETPATH/libservicepoint_binding_uniffi.so"
CONFIG_TOML="$(realpath $SCRIPTPATH/../uniffi.toml)"

BINDGEN="cargo run --features=uniffi/cli --bin uniffi-bindgen -- "
BINDGEN_CS="cargo run --features=cs --bin uniffi-bindgen-cs -- "
COMMON_ARGS="--library $SERVICEPOINT_SO"

${BINDGEN} generate $COMMON_ARGS --language python --out-dir libraries/python
${BINDGEN} generate $COMMON_ARGS --language kotlin --out-dir libraries/kotlin
${BINDGEN} generate $COMMON_ARGS --language swift --out-dir libraries/swift
${BINDGEN_CS} $COMMON_ARGS --out-dir libraries/csharp/ServicePoint
