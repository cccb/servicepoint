ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
OUT_DIR := ${ROOT_DIR}/out

SP2_DIR := ${ROOT_DIR}/../../servicepoint
SP2_INCLUDE := ${ROOT_DIR}/../../servicepoint-binding-c
SP2_TARGET_RELEASE := ${SP2_DIR}/target/release

.PHONY: build run clean

all: ${OUT_DIR}/lang_c

run: ${OUT_DIR}/lang_c
	out/lang_c

clean:
	rm -r ${OUT_DIR} || true
	rm -r ${SP2_TARGET_RELEASE} || true

${OUT_DIR}/lang_c: main.c
	cd ${SP2_DIR} && cargo build --release --all-features
	mkdir -p ${OUT_DIR}
	gcc main.c -I ${SP2_INCLUDE} -L ${SP2_TARGET_RELEASE} -Wl,-Bstatic -lservicepoint -Wl,-Bdynamic -o ${OUT_DIR}/lang_c
