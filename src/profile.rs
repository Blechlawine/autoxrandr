/*
    Copyright (C) 2024 Blechlawine
    GNU General Public License v3.0+ ( see LICENSE or https://www.gnu.org/licenses/gpl-3.0.txt )
*/
use std::{collections::HashMap, path::Path, process::Command};

use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::xrandr_output::{parse_active_monitors, XRandrOutput};

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    connected_devices: HashMap<String, DisplayDeviceProfile>,
    off_devices: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Profiles(pub HashMap<String, Profile>);

#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayDeviceProfile {
    resolution: (u32, u32),
    offset: (u32, u32),
    primary: bool,
    refresh_rate: f32,
}

impl Profiles {
    pub fn save(&self, path: &Path) {
        let json = serde_json::to_string(&self).unwrap();
        std::fs::write(path, json).unwrap();
    }

    pub fn load(path: &Path) -> Self {
        let json = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&json).unwrap_or_default()
    }
}

impl From<XRandrOutput> for Profile {
    fn from(value: XRandrOutput) -> Self {
        let active_monitors = Command::new("xrandr")
            .arg("--listactivemonitors")
            .output()
            .unwrap();
        let (_, active_monitors) = parse_active_monitors(&active_monitors.stdout).unwrap();
        let connected_devices = value
            .displays
            .iter()
            .filter(|d| active_monitors.iter().any(|a| *a == d.connector))
            .map(|d| {
                let connector = &d.connector;
                let resolution = d.resolution.unwrap_or((0, 0));
                let offset = d.offset.unwrap_or((0, 0));
                let primary = d.primary;
                let refresh_rate = d
                    .capabilities
                    .iter()
                    .find_map(|c| {
                        // find the refresh_rate with current == true
                        let c = c.refresh_rates.iter().find(|r| r.current)?;

                        Some(c.clock)
                    })
                    .unwrap();
                (
                    connector.to_string(),
                    DisplayDeviceProfile {
                        resolution,
                        offset,
                        primary,
                        refresh_rate,
                    },
                )
            })
            .collect();
        let off_devices = value
            .displays
            .iter()
            .filter(|d| !active_monitors.iter().any(|a| *a == d.connector))
            .map(|d| d.connector.to_string())
            .collect();

        Profile {
            connected_devices,
            off_devices,
        }
    }
}

impl Profile {
    pub fn apply(&self) {
        let mut args = vec![];
        for (k, device) in &self.connected_devices {
            //println!("{}: {:?}", k, device);
            println!(
                "Applying device {} with resolution: {:?}, offset: {:?}, primary: {}, refresh_rate: {}",
                k.green(), device.resolution, device.offset, device.primary, device.refresh_rate
            );
            args.push("--output".to_string());
            args.push(k.to_string());
            if device.primary {
                args.push("--primary".to_string());
            }
            if device.resolution != (0, 0) {
                args.push("--mode".to_string());
                args.push(format!("{}x{}", device.resolution.0, device.resolution.1));
            }
            if device.offset != (0, 0) {
                args.push("--pos".to_string());
                args.push(format!("{}x{}", device.offset.0, device.offset.1));
            }
            if device.refresh_rate != 0.0 {
                args.push("--rate".to_string());
                args.push(format!("{}", device.refresh_rate));
            }
        }

        for k in &self.off_devices {
            println!("Disabling device {}", k);
            args.push("--output".to_string());
            args.push(k.to_string());
            args.push("--off".to_string());
        }
        std::process::Command::new("xrandr")
            .args(&args)
            .output()
            .unwrap();
    }
}
