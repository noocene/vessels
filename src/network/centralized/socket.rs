use crate::network::DataChannel;

use crate::targets;
use failure::Error;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use futures::Future;

/// A socket server configuration.
#[derive(Clone, Debug, Copy)]
pub struct ListenConfig {
    /// The address and port on which to bind.
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

impl<T> From<T> for ConnectConfig
where
    T: Into<SocketAddr>,
{
    fn from(addr: T) -> ConnectConfig {
        ConnectConfig {
            address: addr.into(),
        }
    }
}

/// A socket connection configuration.
#[derive(Clone, Debug, Copy)]
pub struct ConnectConfig {
    /// The address and port of the target server.
    pub address: SocketAddr,
}

/// A socket connection state.
#[derive(Clone, Debug, Copy)]
pub struct ConnectionDetails {
    /// The address and port of the remote peer.
    pub address: SocketAddr,
}

/// A socket server.
pub type Server = super::Server;

/// Opens a socket server using the provided configuration.
pub fn listen<T>(config: T) -> impl Future<Item = Server, Error = Error>
where
    T: Into<ListenConfig>,
{
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    return targets::web::network::centralized::listen(config.into());

    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    targets::native::network::centralized::listen(config.into())
}

/// Connects to a remote socket server using the provided configuration.
pub fn connect<T>(config: T) -> impl Future<Item = Box<dyn DataChannel + 'static>, Error = Error>
where
    T: Into<ConnectConfig>,
{
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    return targets::web::network::centralized::connect(config.into());

    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    targets::native::network::centralized::connect(config.into())
}
