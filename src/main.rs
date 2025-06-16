use clap::Parser;
use fwl::default_iptables_restore;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "/etc/iptables/iptables.rules")]
    rules_file: String,
}

fn main() {
    let args = Args::parse();
    default_iptables_restore(&args.rules_file);
}
