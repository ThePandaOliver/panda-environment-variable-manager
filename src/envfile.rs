use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub struct EnvFile {
    pub vars: BTreeMap<String, String>,
}

impl EnvFile {
    pub fn load(path: &Path) -> Result<Self> {
        let mut vars = BTreeMap::new();
        if path.exists() {
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read file at path: {}", path.display()))?;
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }
                if let Some((k, v)) = trimmed.split_once('=') {
                    // Strip optional quotes around value
                    let val = v.trim().trim_matches('"').trim_matches('\'');
                    vars.insert(k.trim().to_string(), val.to_string());
                }
            }
        }

        Ok(Self { vars })
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }
        let mut output = String::new();
        for (k, v) in &self.vars {
            output.push_str(&format!("{}=\"{}\"\n", k, v));
        }
        fs::write(path, output).with_context(|| format!("Failed to write to {:?}", path))?;
        Ok(())
    }
}
