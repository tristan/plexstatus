use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::{
    env,
    fs::File,
    io::Read,
    path::PathBuf,
};

#[derive(Deserialize)]
struct Config {
    token: String,
    url: String,
}

impl Config {
    pub fn load() -> Result<Config> {
        let mut home_dir: PathBuf = env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or_else(|| anyhow!("failed to find $HOME"))?;
        home_dir.push(".config");
        home_dir.push("plexstatus");
        home_dir.push("config.toml");
        let mut file = File::open(&home_dir)?;
        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str)?;
        Ok(toml::from_str(&toml_str)?)
    }
}

#[derive(Deserialize)]
struct MediaContainer {
    size: usize,
}

fn main() -> Result<()> {
    let config = Config::load()?;

    let body: String = ureq::get(&format!("{}/status/sessions", config.url))
        .query("X-Plex-Token", &config.token)
        .call()?
        .into_string()?;

    let result: MediaContainer = quick_xml::de::from_str(&body)?;
    if result.size > 0 {
        // if there is media playing, exit with a non 0 exit code
        std::process::exit(1);
    }

    Ok(())
}
