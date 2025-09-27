# 🌳 Yggdrasil

> *“At the center of the cosmos stands Yggdrasil — a great ash tree, holding the nine realms in its roots and branches.”*

Yggdrasil is a **UEFI-first Rust playground** for future OS experiments. It currently boots as a `no_std` firmware app compiled for `x86_64-unknown-uefi`, paints a retro text UI, and listens for keyboard input so you can explore firmware services before handing off to a kernel properly.

---

## ✨ Design Notes

- 🦀 **Rust nightly (`nightly-2025-09-14`)** pinned via `rust-toolchain.toml`.
- 🧠 **UEFI runtime** with the [`uefi`](https://docs.rs/uefi) crate providing console, colour, and input helpers.
- 🎛️ **State-machine menu** in `src/main.rs` with placeholder panels for system info, memory, hardware, boot options, exit services, and shutdown.
- 💾 **Makefile flow** that assembles a FAT image (`uefi-img/EFI/BOOT/BOOTX64.EFI`) and spins up QEMU with OVMF firmware (Homebrew paths by default).

---

## 🚀 Getting Airborne

> ⚠️ You need `rustup`, a nightly compiler, and QEMU with OVMF firmware.

```bash
# (Optional) pre-install the pinned nightly + target
rustup toolchain install nightly-2025-09-14 \
  --target x86_64-unknown-uefi

# Build the firmware app
cargo +nightly-2025-09-14 build --target x86_64-unknown-uefi

# Or let the Makefile prepare the FAT image and launch QEMU
make run
```

`make run` will:
1. Compile `rust-kernel.efi` for `x86_64-unknown-uefi`.
2. Stage the binary as `uefi-img/EFI/BOOT/BOOTX64.EFI`.
3. Copy Homebrew’s `edk2-i386-vars.fd` to `uefi-vars.fd` if it doesn’t exist.
4. Boot QEMU (`qemu-system-x86_64`) with HVF acceleration when available, falling back to TCG otherwise.

Inside QEMU, use the arrow keys to roam, `Enter` to open a panel, and `Esc` to quit back to firmware.

---

## 📜 License

MIT. See `license` for the full text and branch the tree however you like.

---