pub mod helper_functionality;
pub mod restore_functionality;
use helper_functionality::helper_fun::{print_license_info, validate_root_priviliges};
use restore_functionality::restore_fun::{display_ipt_rules, restore_ipt_rules};

/// Restores iptables rules from the specified file and displays the current rules.
///
/// This function:
/// 1. Prints the license information.
/// 2. Validates that the program is running with root privileges.
/// 3. Restores iptables rules from the provided file after user confirmation.
/// 4. Displays the current iptables rules.
///
/// # Note
///
/// This function requires root privileges and may alter the system's firewall configuration.
/// Use with caution.
///
/// # Arguments
///
/// * `rules_file` - The path to the file containing iptables rules to restore.
pub fn default_iptables_restore(rules_file: &str) {
    print_license_info();
    validate_root_priviliges(); // exits if not root
    restore_ipt_rules(rules_file);
    display_ipt_rules();
}
