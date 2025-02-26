use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{error::Error, solver::SolverType, tui::ThemeType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_size", rename = "defaultSize")]
    pub default_size: usize,
    #[serde(default, rename = "defaultSolver")]
    pub default_solver: SolverType,
    #[serde(default, rename = "defaultTheme")]
    pub default_theme: ThemeType,
}

impl Config {
    /// Loads config from default json file path
    pub fn load() -> Self {
        Self::from_json(Self::get_path()).unwrap_or_default()
    }

    /// Saves config to default json path
    pub fn save(&self) -> Result<(), Error> {
        self.to_json(Self::get_path())
    }

    /// Loads config from given path
    pub fn from_json(file: impl AsRef<Path>) -> Result<Self, Error> {
        let buffer = BufReader::new(File::open(file)?);
        Ok(serde_json::from_reader(buffer)?)
    }

    /// Saves config to given path
    pub fn to_json(&self, file: impl AsRef<Path>) -> Result<(), Error> {
        let buffer = BufWriter::new(File::create(file)?);
        Ok(serde_json::to_writer_pretty(buffer, self)?)
    }

    /// Gets config directory
    pub fn get_dir() -> PathBuf {
        dirs::config_dir().unwrap_or(".".into()).join("futoshiki")
    }

    /// Gets config file path
    pub fn get_path() -> PathBuf {
        Self::get_dir().join("config.json")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_size: 4,
            default_solver: Default::default(),
            default_theme: Default::default(),
        }
    }
}

fn default_size() -> usize {
    4
}
