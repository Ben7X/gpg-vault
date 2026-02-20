pub mod args;
pub mod config;

use std::fs::{self, File};

use std::io::Write;
use std::path::{Path, PathBuf};

use crate::args::Args;
use anyhow::Context;
use clap::Parser;
use config::Configuration;
use std::process::Command;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(args.verbosity.clone())
        .init();
    tracing::debug!("{:?}", args);

    // read env variables
    let filter = get_filter(&args.command);

    match &args.command {
        args::Command::Init { create_default } => init(*create_default),
        args::Command::Config {} => config(),
        args::Command::Seal {
            dry_run, status, ..
        } => seal(*dry_run, filter.to_owned(), *status),
        args::Command::Unseal {
            dry_run, status, ..
        } => unseal(*dry_run, filter.to_owned(), *status),
        args::Command::Status { .. } => status(filter.to_owned()),
    }
}

fn get_filter(command: &args::Command) -> Vec<String> {
    if let Ok(values) = std::env::var("GPG_VAULT_GROUPS") {
        let filter = values
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        tracing::info!("Using filter from env {:?}", filter);
        filter
    } else {
        let filter = match &command {
            args::Command::Seal { filter, .. } => filter,
            args::Command::Unseal { filter, .. } => filter,
            args::Command::Status { filter } => filter,
            _ => &vec![],
        };
        filter.to_vec()
    }
}

fn get_config_path() -> anyhow::Result<PathBuf> {
    let home = std::env::var("HOME").context("$HOME variable not set")?;
    let mut path = PathBuf::new();
    path.push(home);
    path.push(".config/gpg-vault/config.yaml");
    tracing::debug!("{:?}", path);
    Ok(path)
}

fn read_config() -> anyhow::Result<Configuration> {
    let config_path = get_config_path()?;
    let content = fs::read_to_string(config_path)?;
    let config: Configuration = serde_yaml::from_str(&content)?;
    tracing::debug!("{:?}", config);
    Ok(config)
}

fn init(create_default: bool) -> anyhow::Result<()> {
    let config_path = get_config_path()?;
    if config_path.exists() {
        return config();
    }
    if create_default {
        tracing::info!("Creating default config");
        let default_config = Configuration::new();
        tracing::debug!("Default config {:?}", default_config);
        let content = serde_yaml::to_string(&default_config)?;
        // check if config folder exists
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                fs::create_dir(parent)?;
            }
        }
        let mut file = File::create(config_path)?;
        write!(file, "{}", content)?;
    } else {
        tracing::info!("Configration does not exists");
    }
    Ok(())
}

fn status(filter: Vec<String>) -> anyhow::Result<()> {
    let config = read_config()?;

    for group in config.groups {
        if !filter.is_empty() && !filter.contains(&group.name) {
            continue;
        }
        tracing::info!("Group {:?}:", group.name);
        for item in group.items {
            let normalized = item.get_path()?;

            let mut value = String::from("\t");
            value.push_str(&normalized);

            let unsealed = Path::new(&normalized);
            let mut sealed_value = normalized.clone();
            sealed_value.push_str(".gpg");
            let sealed = Path::new(&sealed_value);
            tracing::debug!("Unsealed {}, sealed {}", normalized, sealed_value);

            if unsealed.exists() {
                value.push_str("\t UNSEALED");
            }
            if sealed.exists() {
                value.push_str("\t SEALED");
            }
            if !sealed.exists() && !unsealed.exists() {
                value.push_str("\t NOT FOUND");
            }
            tracing::info!("{}", value);
        }
    }
    Ok(())
}

fn config() -> anyhow::Result<()> {
    let config = read_config()?;
    let content = serde_yaml::to_string(&config)?;
    tracing::info!("{}", &content);
    Ok(())
}

fn unseal(dry_run: bool, filter: Vec<String>, show_status: bool) -> anyhow::Result<()> {
    let config = read_config()?;
    tracing::info!("Using gpg-key-id {}", config.gpg_key);

    for group in config.groups {
        if !filter.is_empty() && !filter.contains(&group.name) {
            tracing::debug!("Group filter does not match {:?}", filter);
            continue;
        }
        for item in group.items {
            let item_path = item.get_path()?;
            let encrypted_item = format!("{}.gpg", item_path);
            let path = Path::new(&encrypted_item);

            if path.exists() {
                let cmd_decrypt = format!("gpg -d {} > {}", encrypted_item, item_path,);
                tracing::debug!("{:?}", cmd_decrypt);
                if !dry_run {
                    let status = Command::new("sh").arg("-c").arg(cmd_decrypt).status()?;
                    if !status.success() {
                        fs::remove_file(item_path)?;
                        anyhow::bail!("Could not unseal item");
                    }
                    fs::remove_file(encrypted_item)?;
                }
            }
        }
    }
    if show_status {
        status(filter)?;
    }
    Ok(())
}

fn seal(dry_run: bool, filter: Vec<String>, show_status: bool) -> anyhow::Result<()> {
    let config = read_config()?;
    tracing::info!("Using gpg-key-id {}", config.gpg_key);

    for group in config.groups {
        if !filter.is_empty() && !filter.contains(&group.name) {
            tracing::debug!("Group filter does not match {:?}", filter);
            continue;
        }
        for item in group.items {
            let item_path = item.get_path()?;
            let path = Path::new(&item_path);

            if path.exists() {
                let cmd_encrypt = format!("gpg --recipient {} -e {}", config.gpg_key, item_path);
                tracing::debug!("{:?}", cmd_encrypt);
                if !dry_run {
                    let status = Command::new("sh").arg("-c").arg(cmd_encrypt).status()?;
                    if !status.success() {
                        anyhow::bail!("Could not seal item");
                    }
                    fs::remove_file(item_path)?;
                }
            }
        }
    }
    if show_status {
        status(filter)?;
    }
    Ok(())
}
