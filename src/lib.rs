pub mod helper_functionality;
pub mod restore_functionality;
use helper_functionality::helper_fun::{print_license_info, validate_root_priviliges};
use restore_functionality::restore_fun::{display_ipt_rules, restore_ipt_rules};

/// default restore of iptables rules
pub fn default_iptables_restore() {
    print_license_info();
    validate_root_priviliges(); // exits if not root
    restore_ipt_rules();
    display_ipt_rules();
}
