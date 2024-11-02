#!/usr/bin/env bash

set +x

cargo build --release

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
TARGETPATH="$(realpath $SCRIPTPATH/../../target/release/)"
SERVICEPOINT_SO="$TARGETPATH/libservicepoint_binding_uniffi.so"

BINDGEN="cargo run --features=uniffi/cli --bin uniffi-bindgen -- "
BINDGEN_CS="cargo run --features=cs --bin uniffi-bindgen-cs -- "
GENERATE_ARGS="--library $SERVICEPOINT_SO --out-dir generated"

${BINDGEN} generate ${GENERATE_ARGS} --language python
${BINDGEN} generate ${GENERATE_ARGS} --language kotlin
${BINDGEN} generate ${GENERATE_ARGS} --language swift
${BINDGEN_CS} ${GENERATE_ARGS}
