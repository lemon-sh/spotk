use std::{env, fs, io, path::PathBuf};

pub fn get_configpath() -> io::Result<PathBuf> {
    let configdir = if let Some(dir) = dirs::config_dir() {
        dir
    } else {
        env::current_dir()?
    };

    let configpath = configdir.join("spotk.toml");
    if !configdir.is_dir() {
        fs::create_dir_all(configdir)?;
    }

    Ok(configpath)
}

pub fn get_cachepath() -> io::Result<PathBuf> {
    let cachedir = if let Some(dir) = dirs::cache_dir() {
        dir
    } else {
        env::current_dir()?
    }
    .join("spotk");

    let cachepath = cachedir.join("spotkcache.db3");
    if !cachedir.is_dir() {
        fs::create_dir_all(cachedir)?;
    }

    Ok(cachepath)
}
