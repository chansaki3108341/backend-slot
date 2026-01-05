use serde::Deserialize;
use std::{
    fs,
    io,
    path::{Path, PathBuf},
};

use crate::languages::{builtin_color_for, is_supported_color, Language};

#[derive(Debug)]
pub enum ConfigError {
    Read { path: PathBuf, source: io::Error },
    Parse { path: PathBuf, source: toml::de::Error },
    UnsupportedSchemaVersion { path: PathBuf, found: u32 },
    EmptyLanguages { path: PathBuf },
    EmptyLanguageName { path: PathBuf, index: usize },
    InvalidColor { path: PathBuf, index: usize, color: String },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Read { path, source } => {
                write!(f, "failed to read config '{}': {}", path.display(), source)
            }
            ConfigError::Parse { path, source } => {
                write!(f, "failed to parse config '{}' as TOML: {}", path.display(), source)
            }
            ConfigError::UnsupportedSchemaVersion { path, found } => write!(
                f,
                "unsupported schema_version={} in '{}'; expected 1",
                found,
                path.display()
            ),
            ConfigError::EmptyLanguages { path } => write!(
                f,
                "config '{}' must define a non-empty 'languages' array",
                path.display()
            ),
            ConfigError::EmptyLanguageName { path, index } => write!(
                f,
                "config '{}': languages[{}] has an empty name",
                path.display(),
                index
            ),
            ConfigError::InvalidColor { path, index, color } => write!(
                f,
                "config '{}': languages[{}] has unsupported color '{}' (supported: red, blue, green, yellow, magenta, cyan, purple, orange, white)",
                path.display(),
                index,
                color
            ),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Read { source, .. } => Some(source),
            ConfigError::Parse { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[derive(Deserialize)]
struct RawConfig {
    #[serde(default)]
    schema_version: Option<u32>,
    #[serde(default)]
    languages: Vec<LanguageEntry>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum LanguageEntry {
    // languages = ["Rust", "Go", ...]
    Name(String),
    // languages = [{ name="Rust", color="red" }, ...]
    Object {
        name: String,
        #[serde(default)]
        color: Option<String>,
    },
}

pub fn load_languages_from_file(path: &Path) -> Result<Vec<Language>, ConfigError> {
    let path_buf = path.to_path_buf();

    let content = fs::read_to_string(path)
        .map_err(|e| ConfigError::Read { path: path_buf.clone(), source: e })?;

    let raw: RawConfig = toml::from_str(&content)
        .map_err(|e| ConfigError::Parse { path: path_buf.clone(), source: e })?;

    let schema_version = raw.schema_version.unwrap_or(1);
    if schema_version != 1 {
        return Err(ConfigError::UnsupportedSchemaVersion { path: path_buf, found: schema_version });
    }

    if raw.languages.is_empty() {
        return Err(ConfigError::EmptyLanguages { path: path.to_path_buf() });
    }

    let mut out = Vec::with_capacity(raw.languages.len());
    for (idx, entry) in raw.languages.into_iter().enumerate() {
        let (name, color_opt) = match entry {
            LanguageEntry::Name(name) => (name, None),
            LanguageEntry::Object { name, color } => (name, color),
        };

        let name = name.trim().to_string();
        if name.is_empty() {
            return Err(ConfigError::EmptyLanguageName { path: path.to_path_buf(), index: idx });
        }

        let color = match color_opt {
            Some(c) => c.trim().to_lowercase(),
            None => builtin_color_for(&name).to_string(),
        };

        if !is_supported_color(&color) {
            return Err(ConfigError::InvalidColor { path: path.to_path_buf(), index: idx, color });
        }

        out.push(Language { name, color });
    }

    if out.is_empty() {
        return Err(ConfigError::EmptyLanguages { path: path.to_path_buf() });
    }

    Ok(out)
}
