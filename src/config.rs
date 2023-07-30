use color_eyre::{eyre::eyre, Result};
use serde::Deserialize;
use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
};

use crate::paths;

#[derive(Deserialize)]
pub struct Config {
    pub api: Api,
}

#[derive(Deserialize)]
pub struct Api {
    pub id: String,
    pub secret: String,
}

pub enum LoadResult {
    Opened(Config),
    Created(String),
}

pub fn load() -> Result<LoadResult> {
    let path = paths::get_configpath()?;
    match File::open(&path) {
        Ok(mut file) => {
            let mut config_str = String::new();
            file.read_to_string(&mut config_str)?;
            Ok(LoadResult::Opened(toml::from_str(&config_str)?))
        }
        Err(err) if err.kind() == ErrorKind::NotFound => {
            let mut file = File::create(&path)?;
            file.write_all(include_bytes!("config.toml"))?;
            Ok(LoadResult::Created(path.to_string_lossy().into_owned()))
        }
        Err(err) => Err(eyre!(err)),
    }
}
