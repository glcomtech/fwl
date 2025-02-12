pub mod restore_fun;
use restore_fun::{print_license_info, validate_root_priviliges, restore_ipt_rules, display_ipt_rules};

fn main() {
    print_license_info();
    validate_root_priviliges(); // exits if not root
    restore_ipt_rules();
    display_ipt_rules();
}
