use colored::Colorize;
use std::fs::File;
use std::process::Command;

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
