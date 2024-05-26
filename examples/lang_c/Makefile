CC := gcc

THIS_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
REPO_ROOT := $(THIS_DIR)/../..

build: out/lang_c

clean:
	rm -r out
	rm include/servicepoint.h

run: out/lang_c
	out/lang_c

PHONY: build clean dependencies run

out/lang_c: dependencies src/main.c
	mkdir out || true
	${CC} src/main.c \
		-I include \
		-L $(REPO_ROOT)/target/release \
		-Wl,-Bstatic -lservicepoint \
		-Wl,-Bdynamic -llzma \
		-o out/lang_c \

dependencies: FORCE
	mkdir include || true
	# generate servicepoint header
	SERVICEPOINT_HEADER_OUT=$(THIS_DIR)/include cargo build \
		--manifest-path=$(REPO_ROOT)/crates/servicepoint_binding_c/Cargo.toml

	# build release binary to link against
	cargo build --manifest-path=$(REPO_ROOT)/crates/servicepoint/Cargo.toml --release \
		--features c_api,all_compressions

FORCE: ;
