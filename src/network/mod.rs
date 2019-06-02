use futures::Future;

/// Provides peer-to-peer mesh networking functionality.
pub mod mesh;

/// Provides client/server networking functionality.
pub mod centralized;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ConnectionStatus {
    Connecting,
    Open,
    Closed,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        ConnectionStatus::Connecting
    }
}

pub trait Connection {
    type TransportDetails;
    fn transport_details(&self) -> Self::TransportDetails;
    fn status(&self) -> ConnectionStatus;
    fn on_open(
        &self,
    ) -> Box<
        dyn Future<
                Item = Box<dyn Connection<TransportDetails = Self::TransportDetails>>,
                Error = (),
            > + Send,
    >;
    fn on_close(
        &self,
    ) -> Box<
        dyn Future<
                Item = Box<dyn Connection<TransportDetails = Self::TransportDetails>>,
                Error = (),
            > + Send,
    >;
}
