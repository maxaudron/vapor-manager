use std::path::Path;

use serde::{Deserialize, Serialize};

use super::BestLap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupMeta {
    pub avg_lap: BestLap,
}

impl SetupMeta {
    pub fn read(path: &Path) -> Self {
        let mut path = path.to_owned();
        path.push("meta.json.vapor");
        let data = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    }

    pub fn save(&self, path: &Path) {
        if !path.exists() {
            std::fs::create_dir_all(path).unwrap();
        }

        let mut path = path.to_owned();
        path.push("meta.json.vapor");
        let file = std::fs::File::create(path).unwrap();
        serde_json::to_writer_pretty(file, self).unwrap()
    }
}
