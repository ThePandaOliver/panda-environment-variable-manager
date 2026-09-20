use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "envman", about = "Manage environment variables")]
pub struct Cli {
    #[arg(short, long, value_enum, default_value_t = Scope::User)]
    pub scope: Scope,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    User,
    System,
}

#[derive(Subcommand)]
pub enum Commands {
    Set {
        key: String,
        value: String,
    },
    Get {
        key: String,
    },
    Unset {
        key: String,
    },
    List,
}
