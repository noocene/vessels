use crate::{
    core::{
        acquire,
        hal::crypto::{HashData, Hasher},
    },
    kind::{Future, Serde},
    replicate::Share,
    Kind,
};

use failure::{format_err, Error, Fail};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::{self, Debug, Formatter};

#[derive(Hash, Kind, Clone, PartialEq, Eq)]
pub struct Checksum(pub(crate) [u8; 32]);

impl Debug for Checksum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Checksum {}",
            self.0
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

#[derive(Kind)]
pub struct Resource<T: Serialize + DeserializeOwned + Sync + Send + 'static> {
    checksum: Checksum,
    acquire: Option<Box<dyn FnOnce() -> Future<Serde<T>> + Sync + Send>>,
}

#[derive(Fail, Kind)]
#[fail(display = "reification failed: {}", cause)]
pub struct ReifyError<T: Serialize + DeserializeOwned + Sync + Send + 'static> {
    #[fail(cause)]
    cause: Error,
    pub resource: Resource<T>,
}

impl<T: Serialize + DeserializeOwned + Sync + Send + 'static> Debug for ReifyError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ReifyError {{ cause: {:?} }}", self.cause)
    }
}

impl<T: Serialize + DeserializeOwned + Sync + Send + 'static> Resource<T> {
    pub async fn new_shared(item: &T) -> Self
    where
        T: Share,
    {
        let item = item.share();
        Resource {
            checksum: acquire::<Box<dyn Hasher>>()
                .await
                .unwrap()
                .hash_data(&item)
                .await,
            acquire: Some(Box::new(move || Box::pin(async move { Serde(item) }))),
        }
    }
    pub async fn new(item: T) -> Self {
        Resource {
            checksum: acquire::<Box<dyn Hasher>>()
                .await
                .unwrap()
                .hash_data(&item)
                .await,
            acquire: Some(Box::new(move || Box::pin(async move { Serde(item) }))),
        }
    }
    pub async fn new_ref(item: &T) -> Self {
        Resource {
            checksum: acquire::<Box<dyn Hasher>>()
                .await
                .unwrap()
                .hash_data(item)
                .await,
            acquire: None,
        }
    }
    pub fn reify(self) -> Future<Result<T, ReifyError<T>>> {
        Box::pin(async move {
            if let Some(acquire) = self.acquire {
                Ok(acquire().await.0)
            } else {
                // TODO reify from abstract acquisition methods
                Err(ReifyError {
                    cause: format_err!("no suitable acquisition method"),
                    resource: self,
                })
            }
        })
    }
    pub fn clone_ref(&self) -> Self {
        Resource {
            checksum: self.checksum.clone(),
            acquire: None,
        }
    }
}
