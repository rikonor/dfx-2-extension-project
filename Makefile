EXTENSION_NAME = project

TARGET 	   ?= wasm32-unknown-unknown
TARGET_DIR ?= target/$(TARGET)/debug

all: component output-path

build:
	@cargo build \
		--target $(TARGET)

component: build
	@wasm-tools component new \
		$(TARGET_DIR)/$(EXTENSION_NAME).wasm \
	> $(TARGET_DIR)/$(EXTENSION_NAME).component.wasm \

output-path:
	@realpath $(TARGET_DIR)/$(EXTENSION_NAME).component.wasm
