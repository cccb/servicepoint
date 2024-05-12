ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
OUT_DIR := ${ROOT_DIR}/out

SP2_DIR := ${ROOT_DIR}/../../servicepoint2
SP2_INCLUDE := ${SP2_DIR}/include
SP2_TARGET_RELEASE := ${ROOT_DIR}/../../target/release

.PHONY: clean

run: ${OUT_DIR}/lang_c
	out/lang_c

all: ${OUT_DIR}/lang_c

${OUT_DIR}/lang_c: ${SP2_TARGET_RELEASE} main.c
	mkdir -p ${OUT_DIR}
	gcc main.c -I ${SP2_INCLUDE} -L ${SP2_TARGET_RELEASE} -Wl,-Bstatic -lservicepoint2 -Wl,-Bdynamic -o ${OUT_DIR}/lang_c

${SP2_TARGET_RELEASE}:
	cd ${SP2_DIR} && cargo build --release --all-features

clean:
	rm -r ${SP2_INCLUDE_DIR} || true
	rm -r ${OUT_DIR} || true
	rm -r ${SP2_TARGET_RELEASE} || true