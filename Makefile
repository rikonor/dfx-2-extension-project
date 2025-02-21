NAME = rs_component

TARGET 	   ?= wasm32-unknown-unknown
TARGET_DIR ?= target/$(TARGET)/debug

all: component output-path

build:
	@cargo build \
		--target $(TARGET)

component: build
	@wasm-tools component new \
		$(TARGET_DIR)/$(NAME).wasm \
	> $(TARGET_DIR)/$(NAME).component.wasm \

output-path:
	@realpath $(TARGET_DIR)/$(NAME).component.wasm
