use std::collections::HashMap;

use crate::{config::config, config::taskmasterd::Taskmasterd};
use crate::{config::program::Program, errors::ConfigParseError};

#[derive(Debug, Default)]
pub struct Config {
    pub(in crate::config) programs: HashMap<String, Program>,
    pub(in crate::config) taskmasterd: Taskmasterd,
}

impl Config {
    pub fn default() -> Self {
        Config {
            programs: HashMap::new(),
            taskmasterd: Taskmasterd::default(),
        }
    }

    pub fn get_program(&self, name: &String) -> Option<&Program> {
        self.programs.get(name)
    }

    pub fn find_program(&self, program: &String) -> Option<&Program> {
        self.programs.get(program)
    }

    pub fn find_config() -> Result<String, ConfigParseError> {
        config::DEFAULT_CONFIG_PATHS
            .iter()
            .find(|p| std::path::Path::new(p).is_file())
            .map(|p| p.to_string())
            .ok_or(ConfigParseError::FileNotFound)
    }
}

pub const DEFAULT_CONFIG_PATHS: [&str; 1] = ["./taskmaster.conf"];
