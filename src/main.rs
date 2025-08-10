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

use clap::Parser;
use fwl::default_iptables_restore;

#[derive(Parser)]
#[command(
     author = "Andrew Kushyk",
     version = "0.3.0",
     about = "Quick tool for restoring iptables rules.",
     long_about = None
)]
struct Args {
    #[arg(short, long, default_value = "/etc/iptables/iptables.rules")]
    rules_file: String,
}

fn main() {
    let args = Args::parse();
    default_iptables_restore(&args.rules_file);
}
