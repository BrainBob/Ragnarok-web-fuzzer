use crate::models::LootEntry;
use anyhow::Result;
use std::fs;

pub fn save_text(entries: &[LootEntry], path: &str) -> Result<()> {
    let lines: Vec<String> = entries
        .iter()
        .map(|e| format!("[{}] {}b  {}", e.status, e.size, e.url))
        .collect();
    fs::write(path, lines.join("\n"))?;
    Ok(())
}

pub fn save_json(entries: &[LootEntry], path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(entries)?;
    fs::write(path, json)?;
    Ok(())
}
