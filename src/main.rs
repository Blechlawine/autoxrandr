/*
    Copyright (C) 2024 Blechlawine
    GNU General Public License v3.0+ ( see LICENSE or https://www.gnu.org/licenses/gpl-3.0.txt )
*/
use std::process::Command;

use clap::Parser;
use owo_colors::OwoColorize;

use crate::profile::Profiles;

mod profile;
mod xrandr_output;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    /// Save a layout
    Save {
        /// Name of the layout
        name: String,
    },
    /// Apply a layout
    Apply {
        /// Name of the layout
        name: String,
    },
    /// Remove a layout
    Remove {
        /// Name of the layout
        name: String,
    },
    /// List all saved layouts
    List,
}

fn main() {
    let cli = Cli::parse();

    let config_path = home::home_dir().unwrap().join(".config/autoxrandr");
    if !config_path.exists() {
        std::fs::create_dir_all(&config_path).unwrap();
    }

    let profiles_path = config_path.join("xprofile.json");

    match cli {
        Cli::Save { name } => {
            let xrandr_output = Command::new("xrandr").output().unwrap().stdout;

            let xrandr_output = String::from_utf8_lossy(&xrandr_output);
            let xrandr_output = xrandr_output::parse(&xrandr_output);
            let mut profiles = Profiles::load(&profiles_path);
            profiles.0.insert(name.clone(), xrandr_output.into());
            profiles.save(&profiles_path);
            println!("Saved layout {}", name.green())
        }
        Cli::Apply { name } => {
            let profiles = Profiles::load(&profiles_path);
            let selected_profile = profiles
                .0
                .get(&name)
                .unwrap_or_else(|| panic!("No profile with name {}", name.red()));
            println!("Applying layout {}", name.green());
            selected_profile.apply();
        }
        Cli::Remove { name } => {
            let mut profiles = Profiles::load(&profiles_path);
            profiles.0.remove(&name);
            profiles.save(&profiles_path);
            println!("Removed layout {}", name.yellow())
        }
        Cli::List => {
            let profiles = Profiles::load(&profiles_path);
            for (name, _) in profiles.0 {
                println!("{}", name);
            }
        }
    }
}
