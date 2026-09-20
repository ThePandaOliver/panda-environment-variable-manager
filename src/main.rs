use std::path::PathBuf;
use anyhow::Result;
use clap::Parser;

mod cli;
mod envfile;
mod systemd;

use cli::{Cli, Commands, Scope};
use envfile::EnvFile;

fn get_config_path(scope: Scope) -> PathBuf {
	match scope {
		Scope::User => dirs::config_dir()
			.map(|p| p.join("environment.d/envman.conf"))
			.unwrap_or_else(|| PathBuf::from("~/.config/environment.d/envman.conf")),
		Scope::System => PathBuf::from("/etc/environment"),
	}
}

fn main() -> Result<()> {
	let args = Cli::parse();
	let config_path = get_config_path(args.scope);
	let mut env_file = EnvFile::load(&config_path)?;

	match args.command {
		Commands::Set { key, value } => {
			env_file.vars.insert(key.clone(), value.clone());
			env_file.save(&config_path)?;
			systemd::set_live_env(args.scope, &key, &value)?;
			println!("✓ Set {}={} ({:?})", key, value, args.scope);
		}
		Commands::Get { key } => {
			if let Some(val) = env_file.vars.get(&key) {
				println!("{}", val);
			} else {
				eprintln!("Variable '{}' not found in {:?}", key, config_path);
			}
		}
		Commands::Unset { key } => {
			if env_file.vars.remove(&key).is_some() {
				env_file.save(&config_path)?;
				systemd::unset_live_env(args.scope, &key)?;
				println!("✓ Unset {} ({:?})", key, args.scope);
			} else {
				println!("Variable '{}' does not exist.", key);
			}
		}
		Commands::List => {
			for (k, v) in &env_file.vars {
				println!("{}={}", k, v);
			}
		}
	}

	Ok(())
}