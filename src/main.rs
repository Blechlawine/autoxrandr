use std::process::Command;

use clap::Parser;

mod parse_xrandr_output;
mod xrandr_output;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    /// Save a layout
    Save {
        /// Name of the layout
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli::Save { name: _ } => {
            let xrandr_output = Command::new("xrandr").output().unwrap().stdout;

            let xrandr_output = String::from_utf8_lossy(&xrandr_output);
            let xrandr_output = parse_xrandr_output::parse(&xrandr_output);
            dbg!(xrandr_output);
        }
    }
}
