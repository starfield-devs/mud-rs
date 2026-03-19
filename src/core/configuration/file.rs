use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use log::{error, warn};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(default)]
struct Database {
    metadb_path: String,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            metadb_path: "mud.sqlite".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Default, PartialEq)]
struct FileRaw {
    #[serde(default)]
    database: Database,
}

impl FileRaw {
    /// Return the default `Self` if `can-default`, else exit.
    /// Warns and errors respectively.
    fn fallback_or_exit(can_default: bool) -> Self {
        if can_default {
            warn!("Using default (-D) config instead!");
            return Self::default();
        } else {
            error!("Config file defaulting (-D) not enabled. Goodbye!");
            std::process::exit(1);
        }
    }

    fn parse(
        path: &Path,
        can_default: bool,
    ) -> Result<Self> {
        // This does lots of early return magic, so no combinators are used.
        let content: String = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                warn!("Failed to read config file at {:?}: {}", path, e);
                return Ok(FileRaw::fallback_or_exit(can_default));
            }
        };

        let result = match toml::from_str(&content) {
            Ok(ok) => {
                if ok != Self::default() {
                    ok
                } else {
                    warn!("Failed to use config file at {:?}: File has no valid config options", path);
                    FileRaw::fallback_or_exit(can_default)
                }
            }
            Err(e) => {
                warn!("Failed to parse config file at {:?}: {:}", path, e);
                FileRaw::fallback_or_exit(can_default)
            }
        };

        Ok(result)
    }
}

pub struct File {
    pub metadb_path: PathBuf,
}

impl File {
    pub fn parse_with_inference(
        path: &PathBuf,
        can_default: bool,
    ) -> Result<Self> {
        let file = FileRaw::parse(&path, can_default)?;

        let metadb_path = {
            let mut pathbuf = PathBuf::from(file.database.metadb_path);

            if !pathbuf.is_absolute() {
                pathbuf = std::env::current_dir()
                    .inspect_err(|e| error!("Failed to get the working directory: {}", e))?
                    .join(pathbuf)
            };

            pathbuf // Explicitly *not* `.canonicalize()`! This will be find-or-create later.
        };

        Ok(Self { metadb_path: metadb_path })
    }
}
