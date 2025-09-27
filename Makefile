RUST_KERNEL_DIR=.
TARGET=x86_64-unknown-uefi
TOOLCHAIN?=nightly-2025-09-14
BUILD_STD_FLAGS=
BUILD_DIR=$(RUST_KERNEL_DIR)/target/$(TARGET)/debug
EFI_NAME=rust-kernel
EFI_FILE=$(BUILD_DIR)/$(EFI_NAME).efi
UEFI_IMAGE_DIR=uefi-img
QEMU_SHARE=$(shell brew --prefix)/share/qemu
OVMF_CODE?=$(QEMU_SHARE)/edk2-x86_64-code.fd
OVMF_VARS?=./uefi-vars.fd

.PHONY: all build run release clean image tree

all: run

build:
	cargo +$(TOOLCHAIN) build $(BUILD_STD_FLAGS) --target $(TARGET)

release:
	cargo +$(TOOLCHAIN) build --release $(BUILD_STD_FLAGS) --target $(TARGET)

uefi-vars:
	@if [ ! -f $(OVMF_VARS) ]; then \
		echo "Creating writable UEFI vars file..."; \
		cp $(QEMU_SHARE)/edk2-i386-vars.fd $(OVMF_VARS); \
	fi

tree: build
	rm -rf $(UEFI_IMAGE_DIR)
	mkdir -p $(UEFI_IMAGE_DIR)/EFI/BOOT
	cp $(EFI_FILE) $(UEFI_IMAGE_DIR)/EFI/BOOT/BOOTX64.EFI

image: tree

run: tree uefi-vars
	@echo "Starting UEFI kernel in QEMU..."
	@echo "UEFI Code: $(OVMF_CODE)"
	@echo "UEFI Vars: $(OVMF_VARS)"
	@echo "Boot Image: $(UEFI_IMAGE_DIR)/EFI/BOOT/BOOTX64.EFI"
	@echo ""
	qemu-system-x86_64 \
	  -machine q35,accel=hvf:tcg \
	  -cpu qemu64 \
	  -m 256M \
	  -drive if=pflash,format=raw,readonly=on,file=$(OVMF_CODE) \
	  -drive if=pflash,format=raw,file=$(OVMF_VARS) \
	  -drive format=raw,file=fat:rw:$(UEFI_IMAGE_DIR) \
	  -serial stdio \
	  -display cocoa \
	  -monitor none

clean:
	rm -rf target $(UEFI_IMAGE_DIR) $(OVMF_VARS)
