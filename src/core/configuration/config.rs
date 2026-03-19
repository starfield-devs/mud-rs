use std::path::PathBuf;

use anyhow::Result;
use log::error;

use crate::{
    core::configuration::{cli, file},
    debug_p,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    metadb_path: PathBuf,
    config_path: PathBuf,
    debug: bool,
    logging: bool,
    logging_long: bool,
    color: bool,
    colorful: bool,
}

impl Config {
    pub fn parse() -> Self {
        match Self::try_parse() {
            Ok(parsed) => parsed,
            Err(e) => {
                println!("{:}", e);
                std::process::exit(1)
            }
        }
    }

    pub fn try_parse() -> Result<Self> {
        let cli = cli::Cli::parse_with_inference()?; // Explicitly not `.inspect_err`. We want clap errors here.
        let file = file::File::parse_with_inference(&cli.config_path, cli.allow_default)
            .inspect_err(|e| error!("M3W to fail to get file config: {}", e))?;
        let result = Self {
            metadb_path: file.metadb_path,
            config_path: cli.config_path,
            debug: cli.debug,
            logging: cli.logging,
            logging_long: cli.logging_long,
            color: cli.color,
            colorful: cli.colorful,
        };

        debug_p!(result.logging_long.clone(), result);

        Ok(result)
    }
}
