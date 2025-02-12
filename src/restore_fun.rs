use std::process::Command;
use std::fs::File;
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
        eprintln!("{}", "This program requires root privileges. Please run with sudo.\nExample: sudo ./fwl".red());
        // exits if not root
        std::process::exit(1);
    }
}

/// restores iptables firewall rules from /etc/iptables/iptables.rules file
pub fn restore_ipt_rules() {
    let file = File::open("/etc/iptables/iptables.rules")
        .expect("Failed to open iptables.rules file. Check if it exists.");

    let output = Command::new("iptables-restore")
        .stdin(file)
        .output()
        .expect("Failed to execute iptables-restore < /etc/iptables/iptables.rules command");

    if output.status.success() {
        println!("{}", "iptables rules restored successfully.".green());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}{}", "Error:\n".red(), stderr.red());
    }
}

/// displays iptables firewall filter table
pub fn display_ipt_rules() {
    let output = Command::new("iptables")
        .arg("-nvL")
        .output()
        .expect("Failed to execute iptables -nvL");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("\n\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}{}", "Error:\n".red(), stderr.red());
    }
}
