CC := gcc

THIS_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
REPO_ROOT := $(THIS_DIR)/../../../..

build: out/lang_c

clean:
	rm -r out || true
	rm include/servicepoint.h || true
	cargo clean

run: out/lang_c
	out/lang_c

PHONY: build clean dependencies run

out/lang_c: dependencies src/main.c
	mkdir -p out || true
	${CC} src/main.c \
		-I include \
		-L $(REPO_ROOT)/target/release \
		-Wl,-Bstatic -lservicepoint_binding_c \
		-Wl,-Bdynamic -llzma \
		-o out/lang_c

dependencies: FORCE
	mkdir -p include || true
	# generate servicepoint header and binary to link against
	SERVICEPOINT_HEADER_OUT=$(THIS_DIR)/include cargo build \
		--manifest-path=$(REPO_ROOT)/crates/servicepoint_binding_c/Cargo.toml \
		--release

FORCE: ;
