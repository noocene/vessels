use crate::network::Connection;

use crate::errors::Error;
use crate::targets;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use futures::Future;

/// A socket server configuration.
#[derive(Clone, Debug)]
pub struct ListenConfig {
    pub address: SocketAddr,
}

impl<T> From<T> for ListenConfig
where
    T: Into<u16>,
{
    fn from(port: T) -> ListenConfig {
        let port: u16 = port.into();
        ListenConfig {
            address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port).into(),
        }
    }
}

/// A socket connection configuration.
#[derive(Clone, Debug, Default, Copy)]
pub struct ConnectConfig {}

/// A socket connection state.
#[derive(Clone, Debug)]
pub struct ConnectionDetails {
    pub address: SocketAddr,
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
