use crate::prelude::*;

use std::env;
use std::fs;
use std::path;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde_json;
use serde::{Deserialize};


#[derive(Clone, Deserialize)]
pub struct Config {
    pub resource_root: path::PathBuf,
    pub port: u16,
}


impl Config {
    pub fn try_from(env_args: env::Args) -> Result<Config, String> {
        let config_pb = match parse_config_pathbuf(env_args) {
            Ok(config_pathbuf) => config_pathbuf,
            Err(e) => return Err(e)
        };

        parse_config(config_pb)
    }

    pub fn addr(&self) -> SocketAddr {
        SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            self.port,
        )
    }
}


fn parse_config_pathbuf(mut env_args: env::Args) -> Result<path::PathBuf, String> {
    match env_args.nth(1) {
        Some(config_file) => Ok(path::PathBuf::from(config_file)),
        None => Err("Failed to parse config file from cli command".to_string())
    }
}


fn parse_config(config_file_name: path::PathBuf) -> Result<Config, String> {
    let config_file = match env::current_dir() {
        Ok(current_dir) => current_dir.join(config_file_name),
        Err(e) => return Err(e.to_string())
    };

    let json_str = match fs::read_to_string(config_file) {
        Ok(r) => r,
        Err(_) => return Err("Failed to read to string".to_string()),
    };

    let conf: Config = match serde_json::from_str(&json_str) {
        Ok(conf) => conf,
        Err(e) => return Err(e.to_string())
    };

    Ok(conf)
}