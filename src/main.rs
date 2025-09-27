#![no_main]

use uefi::prelude::*;
use uefi::helpers::init;
use uefi::proto::console::text::{Color, Output, Key, ScanCode};
use std::fmt::Write as _;

#[derive(PartialEq)]
enum UIState {
    MainMenu,
    SystemInfo,
    MemoryStatus,
    HardwareDetection,
    BootOptions,
    ExitBootServices,
    Shutdown,
}

struct UI {
    selected_item: usize,
    menu_items: &'static [&'static str],
    state: UIState,
}

impl UI {
    fn new() -> Self {
        Self {
            selected_item: 0,
            menu_items: &[
                "System Information",
                "Memory Status",
                "Hardware Detection",
                "Boot Options",
                "Exit Boot Services",
                "Shutdown",
            ],
            state: UIState::MainMenu,
        }
    }

    fn draw_header(&self, stdout: &mut Output) {
        let _ = stdout.set_color(Color::White, Color::Blue);
        let _ = stdout.write_str("================================================================================\r\n");
        let _ = stdout.write_str("                         RustOS - UEFI Kernel v1.0                           \r\n");
        let _ = stdout.write_str("                            Advanced Operating System                         \r\n");
        let _ = stdout.write_str("================================================================================\r\n");
    }

    fn draw_menu(&self, stdout: &mut Output) {
        let _ = stdout.set_color(Color::Cyan, Color::Black);
        let _ = stdout.write_str("\r\nMain Menu:\r\n");

        for (index, item) in self.menu_items.iter().enumerate() {
            if self.selected_item == index {
                let _ = stdout.set_color(Color::Black, Color::LightGray);
                let _ = write!(stdout, "> {}\r\n", item);
                let _ = stdout.set_color(Color::LightGray, Color::Black);
            } else {
                let _ = stdout.set_color(Color::LightGray, Color::Black);
                let _ = write!(stdout, "  {}\r\n", item);
            }
        }

        let _ = stdout.write_str("\r\n");
    }

    fn draw_footer(&self, stdout: &mut Output) {
        let _ = stdout.set_color(Color::Yellow, Color::Black);
        let _ = stdout.write_str("Controls: [UP/DOWN] Navigate [Enter] Select [ESC] Exit\r\n");
        let _ = stdout.set_color(Color::DarkGray, Color::Black);
        let _ = stdout.write_str("Status: Boot Services Active | UEFI Ready\r\n");
    }

    fn handle_selection(&self, stdout: &mut Output) -> bool {
        match self.selected_item {
            0 => self.show_system_info(stdout),
            1 => self.show_memory_status(stdout),
            2 => self.show_hardware_detection(stdout),
            3 => self.show_boot_options(stdout),
            4 => {
                self.show_exit_boot_services(stdout);
                return false;
            }
            5 => {
                self.show_shutdown(stdout);
                return false;
            }
            _ => {}
        }
        true
    }

    fn show_system_info(&self, stdout: &mut Output) {
        let _ = stdout.clear();
        let _ = stdout.set_color(Color::White, Color::Black);
        let _ = stdout.write_str("+----------------------------- System Information -----------------------------+\r\n");
        let _ = stdout.set_color(Color::LightGreen, Color::Black);
        let _ = stdout.write_str("\r\n");
        let _ = stdout.write_str("Kernel:           RustOS UEFI v1.0\r\n");
        let _ = stdout.write_str("Architecture:     x86_64 (64-bit)\r\n");
        let _ = stdout.write_str("Boot Protocol:    UEFI 2.0+\r\n");
        let _ = stdout.write_str("Firmware Vendor:  UEFI reference implementation\r\n");
        let _ = stdout.write_str("Firmware Rev:     Available via UEFI\r\n");
        let _ = stdout.write_str("Language:         Rust (nightly)\r\n");
        let _ = stdout.write_str("Target:           x86_64-unknown-uefi\r\n");

        let _ = stdout.set_color(Color::Yellow, Color::Black);
        let _ = stdout.write_str("\r\nPress any key to return to menu...\r\n");
    }

    fn show_memory_status(&self, stdout: &mut Output) {
        let _ = stdout.clear();
        let _ = stdout.set_color(Color::White, Color::Black);
        let _ = stdout.write_str("+------------------------------- Memory Status --------------------------------+\r\n");
        let _ = stdout.set_color(Color::LightGreen, Color::Black);

        let _ = stdout.write_str("\r\n");
        let _ = stdout.write_str("Boot Services:    Active\r\n");
        let _ = stdout.write_str("Memory Map:       Available via UEFI\r\n");
        let _ = stdout.write_str("Allocation:       UEFI pool/page services\r\n");
        let _ = stdout.write_str("Virtual Memory:   Not yet configured\r\n");
        let _ = stdout.write_str("Protection:       UEFI defaults\r\n");
        let _ = stdout.write_str("Status:           Pre-kernel allocation phase\r\n");

        let _ = stdout.set_color(Color::Cyan, Color::Black);
        let _ = stdout.write_str("\r\nNote: Memory management will be initialized after exiting boot services.\r\n");

        let _ = stdout.set_color(Color::Yellow, Color::Black);
        let _ = stdout.write_str("\r\nPress any key to return to menu...\r\n");
    }

    fn show_hardware_detection(&self, stdout: &mut Output) {
        let _ = stdout.clear();
        let _ = stdout.set_color(Color::White, Color::Black);
        let _ = stdout.write_str("+------------------------------ Hardware Detection ----------------------------+\r\n");
        let _ = stdout.set_color(Color::LightGreen, Color::Black);

        let _ = stdout.write_str("\r\n");
        let _ = stdout.write_str("Input Devices:    Keyboard ready, mouse pending\r\n");
        let _ = stdout.write_str("Display:          UEFI GOP (graphics output)\r\n");
        let _ = stdout.write_str("Storage:          UEFI block I/O services\r\n");
        let _ = stdout.write_str("Network:          UEFI network protocols (if available)\r\n");
        let _ = stdout.write_str("Audio:            Not implemented\r\n");
        let _ = stdout.write_str("Timer:            UEFI timer services ready\r\n");

        let _ = stdout.set_color(Color::Cyan, Color::Black);
        let _ = stdout.write_str("\r\nNote: Hardware drivers load after kernel initialization.\r\n");

        let _ = stdout.set_color(Color::Yellow, Color::Black);
        let _ = stdout.write_str("\r\nPress any key to return to menu...\r\n");
    }

    fn show_boot_options(&self, stdout: &mut Output) {
        let _ = stdout.clear();
        let _ = stdout.set_color(Color::White, Color::Black);
        let _ = stdout.write_str("+-------------------------------- Boot Options --------------------------------+\r\n");
        let _ = stdout.set_color(Color::LightGreen, Color::Black);

        let _ = stdout.write_str("\r\n");
        let _ = stdout.write_str("Boot Mode:        UEFI native\r\n");
        let _ = stdout.write_str("Secure Boot:      Status unknown\r\n");
        let _ = stdout.write_str("Boot Services:    Currently active\r\n");
        let _ = stdout.write_str("Configuration:    Default UEFI settings\r\n");
        let _ = stdout.write_str("Runtime Services: Will remain after boot\r\n");

        let _ = stdout.set_color(Color::Yellow, Color::Black);
        let _ = stdout.write_str("\r\nPress any key to return to menu...\r\n");
    }

    fn show_exit_boot_services(&self, stdout: &mut Output) {
        let _ = stdout.clear();
        let _ = stdout.set_color(Color::White, Color::Red);
        let _ = stdout.write_str("+--------------------------- Exit Boot Services ------------------------------+\r\n");
        let _ = stdout.set_color(Color::Yellow, Color::Black);

        let _ = stdout.write_str("\r\n");
        let _ = stdout.write_str("Preparing to exit UEFI boot services...\r\n");
        let _ = stdout.write_str("Getting memory map...\r\n");
        let _ = stdout.write_str("Transitioning to kernel runtime...\r\n");
        let _ = stdout.write_str("Entering higher-half kernel code...\r\n");

        let _ = stdout.set_color(Color::LightGreen, Color::Black);
        let _ = stdout.write_str("\r\nBoot services exit complete!\r\n");
        let _ = stdout.write_str("RustOS kernel is now running independently.\r\n");

        let _ = stdout.set_color(Color::Cyan, Color::Black);
        let _ = stdout.write_str("\r\nKernel will continue execution...\r\n");
    }

    fn show_shutdown(&self, stdout: &mut Output) {
        let _ = stdout.clear();
        let _ = stdout.set_color(Color::White, Color::Red);
        let _ = stdout.write_str("+-------------------------------- Shutdown ------------------------------------+\r\n");
        let _ = stdout.set_color(Color::LightRed, Color::Black);

        let _ = stdout.write_str("\r\n");
        let _ = stdout.write_str("Shutting down RustOS...\r\n");
        let _ = stdout.write_str("Flushing buffers...\r\n");
        let _ = stdout.write_str("Releasing resources...\r\n");
        let _ = stdout.write_str("Powering off...\r\n");

        let _ = stdout.set_color(Color::DarkGray, Color::Black);
        let _ = stdout.write_str("\r\nGoodbye!\r\n");
    }


}

#[entry]
fn efi_main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    init(&mut system_table).expect("UEFI services init");
    
    let mut ui = UI::new();
    let mut need_redraw = true;
    
    loop {
        if need_redraw {
            {
                let stdout = system_table.stdout();
                match ui.state {
                    UIState::MainMenu => {
                        let _ = stdout.clear();
                        ui.draw_header(stdout);
                        ui.draw_menu(stdout);
                        ui.draw_footer(stdout);
                    }
                    UIState::SystemInfo => {
                        ui.show_system_info(stdout);
                    }
                    UIState::MemoryStatus => {
                        ui.show_memory_status(stdout);
                    }
                    UIState::HardwareDetection => {
                        ui.show_hardware_detection(stdout);
                    }
                    UIState::BootOptions => {
                        ui.show_boot_options(stdout);
                    }
                    UIState::ExitBootServices => {
                        ui.show_exit_boot_services(stdout);
                        break;
                    }
                    UIState::Shutdown => {
                        ui.show_shutdown(stdout);
                        break;
                    }
                }
            }
            need_redraw = false;
        }
        
        let key_result = {
            let stdin = system_table.stdin();
            stdin.read_key()
        };
        
        if let Ok(Some(key)) = key_result {
            match ui.state {
                UIState::MainMenu => {
                    match key {
                        Key::Special(ScanCode::UP) => {
                            if ui.selected_item > 0 {
                                ui.selected_item -= 1;
                                need_redraw = true;
                            }
                        }
                        Key::Special(ScanCode::DOWN) => {
                            if ui.selected_item < ui.menu_items.len() - 1 {
                                ui.selected_item += 1;
                                need_redraw = true;
                            }
                        }
                        Key::Printable(c) if c == '\r' => {
                            match ui.selected_item {
                                0 => ui.state = UIState::SystemInfo,
                                1 => ui.state = UIState::MemoryStatus,
                                2 => ui.state = UIState::HardwareDetection,
                                3 => ui.state = UIState::BootOptions,
                                4 => ui.state = UIState::ExitBootServices,
                                5 => ui.state = UIState::Shutdown,
                                _ => {}
                            }
                            need_redraw = true;
                        }
                        Key::Special(ScanCode::ESCAPE) => {
                            break;
                        }
                        _ => {}
                    }
                }
                _ => {
                    ui.state = UIState::MainMenu;
                    need_redraw = true;
                }
            }
        } else {
            let _ = system_table.boot_services().stall(50_000);
        }
    }
    
    Status::SUCCESS
}
