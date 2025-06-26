/*
 * fwl - Quick tool for restoring iptables rules.
 * Copyright (C) 2025  Andrew Kushyk
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use colored::Colorize;
use nix::unistd::Uid;

/// Prints the license information for the fwl crate.
///
/// This function displays the copyright notice and the GPL-3.0 license terms under which the fwl crate is distributed.
/// It informs users of their rights and obligations, ensuring compliance with open-source licensing requirements.
/// The output is formatted as plain text and includes a reference to the GPL-3.0 license for further details.
///
/// The GPL-3.0 is a copyleft license that allows users to freely use, modify, and distribute the software,
/// provided the source code is made available and modifications are distributed under the same license.
pub fn print_license_info() {
    println!("fwl  Copyright (C) 2025  Andrew Kushyk\n\
This program comes with ABSOLUTELY NO WARRANTY; for details see https://www.gnu.org/licenses/gpl-3.0.html\n\
This is free software, and you are welcome to redistribute it\n\
under certain conditions; for details see https://www.gnu.org/licenses/gpl-3.0.html\n");
}

/// Validates whether the program is running with root privileges.
///
/// This function checks if the current user has root privileges using `nix::unistd::Uid::current().is_root()`.
/// If the program lacks root privileges, it prints an error message in red using the `colored` crate and exits with a status code of 1.
/// Root privileges are required because managing iptables rules involves system-level operations that only the superuser can perform.
///
/// # Notes
/// - This function uses the `nix` crate, which provides a safe, Rust-idiomatic interface to Unix system calls, avoiding direct use of `libc`.
/// - Ensure the `nix` crate is included in `Cargo.toml` with the `user` feature enabled (e.g., `nix = { version = "0.30", features = ["user"] }`).
pub fn validate_root_priviliges() {
    if !Uid::current().is_root() {
        eprintln!(
            "{}{}",
            "This program requires root privileges. Please run it with sudo.\n\n".red(),
            "Examples:\nsudo ./fwl\nsudo ./fwl -r /path/to/your/iptables.rules\nsudo ./fwl --rules-file /path/to/your/iptables.rules"
        );
        std::process::exit(1);
    }
}
