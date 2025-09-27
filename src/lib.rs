#![no_std]
#![no_main]

extern crate alloc;
extern crate rlibc;
use uefi::prelude::*;
use uefi::helpers::init;
use uefi::proto::console::text::{Color, Output};
use core::fmt::Write as _;

#[entry]
fn efi_main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    init(&mut system_table).expect("UEFI services init");

    let stdout: &mut Output = system_table.stdout();
    let _ = stdout.clear();
    let _ = stdout.set_color(Color::Yellow, Color::Black);
    let _ = write!(stdout, "Rust UEFI Kernel Loader\r\n");
    let _ = stdout.set_color(Color::LightGreen, Color::Black);
    let _ = write!(stdout, "Boot services active.\r\n");

    loop {}
}
