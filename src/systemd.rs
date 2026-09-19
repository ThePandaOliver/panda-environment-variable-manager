use std::process::Command;
use anyhow::Result;
use crate::cli::Scope;

pub fn set_live_env(scope: Scope, key: &str, value: &str) -> Result<()> {
    let mut cmd = Command::new("systemctl");
    if scope == Scope::User {
        cmd.arg("--user");
    }
    cmd.args(["set-environment", &format!("{}={}", key, value)]);
    cmd.status()?;

    // If user scope, also update DBus activation environment for GUI/desktop apps
    if scope == Scope::User {
        let _ = Command::new("dbus-update-activation-environment")
            .args(["--systemd", &format!("{}={}", key, value)])
            .status();
    }
    Ok(())
}

pub fn unset_live_env(scope: Scope, key: &str) -> Result<()> {
    let mut cmd = Command::new("systemctl");
    if scope == Scope::User {
        cmd.arg("--user");
    }
    cmd.args(["unset-environment", key]);
    cmd.status()?;
    Ok(())
}