ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
RUST_TARGET_DIR := ${ROOT_DIR}rust-target
export SP2_INCLUDE_DIR := ${ROOT_DIR}sp2
OUT_DIR := ${ROOT_DIR}out

.PHONY: clean

run: ${OUT_DIR}/lang_c
	out/lang_c

all: ${OUT_DIR}/lang_c

${OUT_DIR}/lang_c: ${SP2_INCLUDE_DIR}/sp2-bindings.h main.c
	mkdir -p ${OUT_DIR}
	gcc ${SP2_INCLUDE_DIR}/sp2-bindings.h main.c -L ${RUST_TARGET_DIR}/release/ -Wl,-Bstatic -l servicepoint2 -Wl,-Bdynamic  -o ${OUT_DIR}/lang_c --verbose

${SP2_INCLUDE_DIR}/sp2-bindings.h:
	cd ../../servicepoint2/ && cargo build --release --target-dir ${RUST_TARGET_DIR} --all-features

clean:
	rm -r ${SP2_INCLUDE_DIR} || true
	rm -r ${OUT_DIR} || true
	rm -r ${RUST_TARGET_DIR} || true