use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::PathBuf;

use crate::config;

pub struct Source {
    pub words: Vec<(String, String)>,
}

pub fn list() -> Result<Vec<String>> {
    let dir = config::sources_dir()?;
    let mut names = Vec::new();
    for entry in
        fs::read_dir(&dir).with_context(|| format!("Failed to read {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json")
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
        {
            names.push(stem.to_string());
        }
    }
    names.sort();
    Ok(names)
}

pub fn load(name: &str) -> Result<Source> {
    let path = source_path(name)?;
    if !path.exists() {
        return Err(anyhow!(
            "Source '{name}' not found at {}",
            path.display()
        ));
    }
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let value: serde_json::Value = serde_json::from_str(&raw)
        .with_context(|| format!("Failed to parse {}", path.display()))?;
    let obj = value
        .as_object()
        .ok_or_else(|| anyhow!("Source '{name}' must be a JSON object {{cn: foreign}}"))?;

    let mut words = Vec::with_capacity(obj.len());
    for (k, v) in obj {
        let v_str = v.as_str().ok_or_else(|| {
            anyhow!("Source '{name}': value for key '{k}' must be a string")
        })?;
        words.push((k.clone(), v_str.to_string()));
    }

    Ok(Source { words })
}

fn source_path(name: &str) -> Result<PathBuf> {
    Ok(config::sources_dir()?.join(format!("{name}.json")))
}
