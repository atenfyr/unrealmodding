use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use log::{error, warn};
use serde::{Deserialize, Serialize};

use crate::game_mod::SelectedVersion;
use crate::AppData;

#[derive(Serialize, Deserialize, Debug)]
struct ModConfig {
    install_path: String,
    refuse_mismatched_connections: bool,
    current: ModsConfigData,
    profiles: HashMap<String, ModsConfigData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModsConfigData {
    mods: HashMap<String, ModConfigData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModConfigData {
    // TODO: make this a non-Option at some point
    force_latest: Option<bool>,
    priority: u16,
    enabled: bool,
    version: String,
}

pub(crate) fn load_config(data: &mut AppData) {
    let config_path = data.data_path.as_ref().unwrap().join("modconfig.json");
    if config_path.is_file() {
        let config_str = fs::read_to_string(config_path).unwrap();
        let config: ModConfig = serde_json::from_str(&config_str).unwrap_or_else(|_| {
            error!("Failed to parse modconfig.json");
            panic!();
        });

        data.refuse_mismatched_connections = config.refuse_mismatched_connections;
        // TODO: properly check this
        data.install_path = Some(PathBuf::from(config.install_path));

        for (mod_id, mod_config) in config.current.mods.iter() {
            let game_mod = data.game_mods.get_mut(mod_id);
            if game_mod.is_none() {
                warn!(
                    "Mod {} referenced in modconfig.json is not installed",
                    mod_id
                );
                continue;
            }
            let game_mod = game_mod.unwrap();
            let force_latest = mod_config.force_latest.unwrap_or(true);

            if !force_latest {
                game_mod.selected_version = match game_mod.selected_version {
                    SelectedVersion::Latest(version) => SelectedVersion::Specific(version),
                    SelectedVersion::LatestIndirect(version) => {
                        SelectedVersion::Specific(version.unwrap())
                    }
                    SelectedVersion::Specific(version) => SelectedVersion::Specific(version),
                };
            }

            game_mod.active = mod_config.enabled;
        }
    }
}