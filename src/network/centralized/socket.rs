use crate::network::Connection;

use crate::errors::{Error, Result};
use crate::targets;

use futures::Future;

/// A socket server configuration.
#[derive(Clone, Debug, Default)]
pub struct ListenConfig {
    pub port: u16,
    pub address: String,
}

impl<T> From<T> for ListenConfig
where
    T: Into<u16>,
{
    fn from(port: T) -> ListenConfig {
        let port: u16 = port.into();
        ListenConfig {
            port,
            address: "127.0.0.1".to_owned(),
        }
    }
}

/// A socket connection configuration.
#[derive(Clone, Debug, Default, Copy)]
pub struct ConnectConfig {}

/// A socket connection state.
#[derive(Clone, Debug, Default)]
pub struct ConnectionDetails {
    pub address: String,
}

pub type Server = super::Server<ConnectionDetails>;

/// Opens a socket server using the provided configuration.
pub fn listen<T>(config: T) -> impl Future<Item = Server, Error = Error>
where
    T: Into<ListenConfig>,
{
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    targets::native::network::centralized::listen(config.into())
}
