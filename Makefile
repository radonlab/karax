.phony: default target emulate

default:
	@echo "Usage: make <target>"

target:
	cargo build --target aarch64-unknown-none-softfloat

emulate: target
	qemu-system-aarch64 \
        -machine virt \
        -cpu cortex-a53 \
        -nographic \
        -m 512M \
        -kernel target/aarch64-unknown-none-softfloat/debug/karax
