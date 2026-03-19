use std::{io::Write, path::PathBuf};

use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use colored::Colorize;
use env_logger::Builder;
use log::{Level, LevelFilter};

#[derive(Parser, Debug)]
#[command(version, about = "Centralized Python orchestrator written in Rust.")]
struct CliRaw {
    #[arg(long, default_value = ".", help = "Path to mud config")]
    config_path: String,
    #[arg(short = 'D', long, default_value_t = false, help = "Allow config fallback")]
    defaulting: bool,
    #[arg(short = 'c', long, default_value_t = false, help = "Allow color in general")]
    color: bool,
    #[arg(short = 'C', long, default_value_t = false, help = "Allow warn/error line color")]
    colorful: bool,
    #[arg(short = 'l', long, default_value_t = false, help = "Allow logging")]
    logging: bool,
    #[arg(short = 'L', long, default_value_t = false, help = "Allow multiline logging")]
    logging_long: bool,
    #[arg(short = 'd', long, default_value_t = false, help = "Mud config path")]
    debug: bool,
}

#[allow(dead_code)]
pub struct Cli {
    pub config_path: PathBuf,
    pub allow_default: bool,
    pub debug: bool,
    pub color: bool,
    pub colorful: bool,
    pub logging: bool,
    pub logging_long: bool,
}

impl Cli {
    pub fn parse_with_inference() -> Result<Self> {
        let from_cli = CliRaw::try_parse()?;

        if from_cli.logging {
            let color = from_cli.color.clone();
            let colorful = from_cli.colorful.clone();
            Builder::from_default_env()
                .format(move |buf, record| {
                    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                    let level = if color {
                        match record.level() {
                            Level::Error => "ERR".red(),
                            Level::Warn => "WRN".yellow(),
                            Level::Info => "INF".green(),
                            Level::Debug => "DBG".blue(),
                            Level::Trace => "TRC".magenta(),
                        }
                    } else {
                        record.level().to_string().normal()
                    };

                    let msg = record.args().to_string();
                    let msg = if colorful {
                        match record.level() {
                            Level::Error => msg.red(),
                            Level::Warn => msg.yellow(),
                            Level::Info => msg.green(),
                            Level::Debug => msg.blue(),
                            Level::Trace => msg.magenta(),
                        }
                    } else {
                        msg.normal()
                    };

                    writeln!(buf, "[{}][{}] {}", timestamp, level, msg)
                })
                .filter(None, if from_cli.debug { LevelFilter::Debug } else { LevelFilter::Warn })
                .try_init()?;
        }

        Ok(Self {
            config_path: {
                let path = PathBuf::from(&from_cli.config_path).canonicalize()?;
                let joined = path.join("mud.toml");
                if path.is_dir() { joined } else { path }
            },
            allow_default: from_cli.defaulting,
            debug: from_cli.debug,
            color: from_cli.color,
            colorful: from_cli.colorful,
            logging: from_cli.logging,
            logging_long: from_cli.logging_long,
        })
    }
}
