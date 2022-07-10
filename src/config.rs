use std::env;
use std::path;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};


#[derive(Clone, Debug)]
pub struct Config {
    pub resource_root: path::PathBuf,
    pub addr: SocketAddr,
}

impl Config {
    pub fn resource_root(&self) -> path::PathBuf {
        self.resource_root.clone()
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            resource_root: env::current_dir().unwrap(),
            addr: SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                3000,
            ),
        }
    }
}