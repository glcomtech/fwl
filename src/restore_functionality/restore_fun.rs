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
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

/// Restores iptables firewall rules from the specified file.
///
/// # Arguments
///
/// * `rules_file` - The path to the file containing iptables rules.
///
/// # Behavior
///
/// - Prompts the user for confirmation before restoring rules.
/// - Opens the specified file and uses `iptables-restore` to apply the rules.
/// - Prints success or error messages accordingly.
pub fn restore_ipt_rules(rules_file: &str) {
    print!(
        "This will restore iptables rules from {}. Continue? (Y/n): ",
        rules_file.green()
    );

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let trimmed_input = input.trim().to_lowercase();
    if !trimmed_input.is_empty() && trimmed_input != "y" {
        println!("{}", "Operation cancelled.".red());
        return;
    }

    let file = match File::open(rules_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open {} file: {}", rules_file, e);
            return;
        }
    };

    let output = match Command::new("iptables-restore").stdin(file).output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to execute iptables-restore: {}", e);
            return;
        }
    };

    if output.status.success() {
        println!("{}", "iptables rules restored successfully.".green());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}{}", "Error:\n".red(), stderr.red());
    }
}

/// Displays the current iptables firewall filter table.
///
/// This function executes the `iptables -nvL` command to list the current rules in the iptables filter table.
/// It prints the output to the console, providing a detailed view of the active firewall rules.
/// This is useful for verifying the current state of the firewall configuration or for diagnostic purposes.
///
/// # Functionality
/// - Executes the `iptables -nvL` command, which lists all chains and rules in the filter table.
/// - Handles potential errors gracefully:
///   - If the `iptables` command cannot be executed (e.g., the binary is not found), it prints an error message to stderr and returns early.
///   - If the command executes but fails (e.g., due to permission issues or invalid options), it prints the command's stderr output in red for better visibility.
/// - Uses the `colored` crate to format error messages in red, enhancing readability.
///
/// # Output
/// - On success: Prints the output of `iptables -nvL` to stdout, prefixed with two newlines for better console formatting.
/// - On failure: Prints error messages to stderr, with errors highlighted in red.
///
/// # Error Handling
/// - If `Command::new("iptables").arg("-nvL").output()` fails (e.g., iptables is not installed), it catches the error and prints a message like "Failed to execute iptables -nvL: {}".
/// - If the command executes but returns a non-zero exit status, it prints "Error:\n" followed by the command's stderr output.
///
/// # Considerations
/// - This function assumes a Linux environment with `iptables` installed and available in the system's PATH.
/// - While listing iptables rules does not typically require root privileges, this program is designed to run with root access because it also includes functionality to modify firewall rules.
/// - For systems using nftables instead of iptables, this function may not work as expected, and alternative methods would be needed to display firewall rules.
/// - The use of `String::from_utf8_lossy` ensures that any invalid UTF-8 sequences in the command's output are handled gracefully, preventing panics.
///
/// # Notes
/// - This function is part of a larger command-line tool for managing iptables rules, where it is typically called after restoring rules to display the updated configuration.
/// - The two newlines (`\n`) before printing the output are added for better readability in the console, separating it from previous output.
pub fn display_ipt_rules() {
    let output = match Command::new("iptables").arg("-nvL").output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to execute iptables -nvL: {}", e);
            return;
        }
    };

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}{}", "Error:\n".red(), stderr.red());
    }
}
