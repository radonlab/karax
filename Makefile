.PHONY: default target emulate

default:
	@echo "Usage: make <target>"

target:
	cargo build --target aarch64-unknown-none-softfloat

emulate: target
	qemu-system-aarch64 \
        -M raspi3b \
        -cpu cortex-a53 \
        -m 1G \
        -kernel target/aarch64-unknown-none-softfloat/debug/karax \
        -display none \
        -serial null \
        -serial stdio
