# Guideline

## Raspberry Pi 3 Model B+ Hardware Configuration

Based on the Raspberry Pi 3 Model B+ product brief, the key hardware specifications are:

- **Processor**: Broadcom BCM2837B0 SoC with quad-core Cortex-A53 (ARMv8) 64-bit processor running at 1.4GHz
- **Memory**: 1GB LPDDR2 SDRAM
- **Connectivity**:
  - Ethernet over USB 2.0 (max 300 Mbps)
  - 2.4GHz and 5GHz IEEE 802.11.b/g/n/ac wireless LAN
  - Bluetooth 4.2/BLE
- **GPIO**: 40-pin GPIO header
- **Ports**: 4× USB 2.0 ports, HDMI output, 3.5mm audio jack, CSI camera port, DSI display port

## Target Architecture

The system kernel is specifically designed for the **Cortex-A53** processor, which implements the ARMv8 architecture and supports both AArch64 (64-bit) and AArch32 (32-bit) execution states.

## Rust Cross-Compilation Toolchain

According to the Rust platform support documentation for `aarch64-unknown-none`, when developing for the Cortex-A53 core:

- The instruction set used is **AArch64** (64-bit ARM architecture)
- The required cross-compilation target is `aarch64-unknown-none-softfloat`
- This target provides a bare-metal environment without operating system support
- The softfloat variant is appropriate for environments where floating-point operations should be handled via software emulation

To install this target, use:

```sh
rustup target add aarch64-unknown-none-softfloat
```

## References

1. [Raspberry Pi 3 Model B+ Product Brief](https://datasheets.raspberrypi.com/rpi3/raspberry-pi-3-b-plus-product-brief.pdf)
2. [ARM Cortex-A53 - Wikipedia](https://en.wikipedia.org/wiki/ARM_Cortex-A53)
3. [ARM Architecture Family - Armv8-A](https://en.wikipedia.org/wiki/ARM_architecture_family#Armv8-A)
4. [Rust Platform Support: aarch64-unknown-none](https://doc.rust-lang.org/nightly/rustc/platform-support/aarch64-unknown-none.html)
