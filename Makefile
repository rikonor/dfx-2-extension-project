EXTENSION_NAME = project

CARGO_RELEASE     ?=
CARGO_TARGET      ?= wasm32-unknown-unknown
CARGO_TARGET_DIR  ?= target/$(CARGO_TARGET)/debug
COMPONENT_OUT_DIR ?= $(CARGO_TARGET_DIR)

all: component output-path

build:
	@cargo build \
		$(if $(CARGO_TARGET),--target $(CARGO_TARGET)) \
		$(if $(CARGO_RELEASE),--release)

component: build
	@wasm-tools component new \
		$(CARGO_TARGET_DIR)/$(EXTENSION_NAME).wasm \
	> $(COMPONENT_OUT_DIR)/$(EXTENSION_NAME).component.wasm \

output-path:
	@realpath $(COMPONENT_OUT_DIR)/$(EXTENSION_NAME).component.wasm
