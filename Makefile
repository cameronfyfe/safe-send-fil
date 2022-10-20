WASM := wasm32-unknown-unknown
NAME := safe_send_fil

.PHONY: default
default: build


.PHONY: build
build:
	cargo build

.PHONY: release
release: artifacts
	cargo build \
		--release \
		--locked
	cp target/$@/wbuild/$(NAME)/$(NAME).compact.wasm $</$(NAME).wasm

.PHONY: clean
clean:
	cargo clean

artifacts:
	mkdir $@
