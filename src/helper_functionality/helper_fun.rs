use colored::Colorize;

/// Prints license info
pub fn print_license_info() {
    println!("fwl  Copyright (C) 2025  Andrew Kushyk\n\
This program comes with ABSOLUTELY NO WARRANTY; for details see https://www.gnu.org/licenses/gpl-3.0.html/\n\
This is free software, and you are welcome to redistribute it\n\
under certain conditions; for details see https://www.gnu.org/licenses/gpl-3.0.html/\n");
}

/// validates if user has root priviliges. Terminates the program otherwise.
pub fn validate_root_priviliges() {
    if unsafe { libc::getuid() } != 0 {
        eprintln!(
            "{}",
            "This program requires root privileges. Please run with sudo.\nExample: sudo ./fwl"
                .red()
        );
        // exits if not root
        std::process::exit(1);
    }
}
