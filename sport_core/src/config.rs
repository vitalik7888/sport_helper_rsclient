use std::net::{IpAddr, Ipv4Addr};
use confy::ConfyError;
use crossterm::event::KeyCode;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub struct KeyMap {
    pub quit: char,
    pub accept: KeyCode,
    pub reject: KeyCode,
}

impl Default for KeyMap {
    fn default() -> Self {
        Self { quit: 'q', accept: KeyCode::Enter, reject: KeyCode::Esc }
    }
}

pub type ConfigError = confy::ConfyError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: IpAddr,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5050,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountConfig {
    pub id: u64,
}

impl AccountConfig {
    pub fn is_valid(&self) -> bool {
        self.id > 0
    }
}

impl Default for AccountConfig {
    fn default() -> Self {
        AccountConfig { id: 0 }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub account: AccountConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfyError> {
         confy::load("sport_helper", None)
    }

    pub fn store(self) -> Result<(), ConfyError> {
        confy::store("sport_helper", None, self)
    }
}
