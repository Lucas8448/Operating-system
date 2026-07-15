# Yggdrasil

Rust `no_std` UEFI/QEMU systems playground. Current scope: a bootable `x86_64-unknown-uefi` firmware app that draws a text UI, handles keyboard input, and exercises UEFI console/boot-service plumbing before a real kernel handoff.

## Status

- Boots through OVMF/QEMU as `uefi-img/EFI/BOOT/BOOTX64.EFI`.
- Uses Rust nightly `nightly-2025-09-14`, pinned in `rust-toolchain.toml`.
- UI lives in `src/main.rs`: menu navigation, system info, memory status, hardware/boot panels, exit/shutdown paths.
- Not a kernel yet: memory management, drivers, scheduler, filesystems, and actual `ExitBootServices` handoff are placeholders.

## Build and run

Requires `rustup`, the pinned nightly target, QEMU, and OVMF firmware. The Makefile assumes Homebrew QEMU paths on macOS.

```bash
rustup toolchain install nightly-2025-09-14 --target x86_64-unknown-uefi
cargo +nightly-2025-09-14 build --target x86_64-unknown-uefi
make run
```

`make run` builds `rust-kernel.efi`, stages it at `uefi-img/EFI/BOOT/BOOTX64.EFI`, creates a writable `uefi-vars.fd` if needed, then starts `qemu-system-x86_64` with OVMF.

Inside QEMU: arrow keys navigate, `Enter` opens a panel, `Esc` exits.

## Recruiter scan

This repo shows low-level Rust work without an operating-system-sized claim: custom UEFI target, `no_main`, panic-abort profiles, firmware console I/O, input polling, QEMU boot wiring, and a deliberately small state-machine UI.

## License

MIT. See `license`.
