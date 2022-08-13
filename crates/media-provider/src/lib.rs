use std::collections::HashMap;

use async_trait::async_trait;

mod jellyfin;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("client")]
    ClientError(#[from] reqwest::Error),
    #[error("bad credentials were provided")]
    BadCredentials,
    
}

#[async_trait]
pub trait Session: Sync {
    async fn libraries(self: &Self);
}

#[async_trait]
pub trait SessionProvider: Sync {
    fn name(&self) -> &'static str;
    async fn login(&self, hostname: &str, username: &str, password: &str) -> Result<Box<dyn Session>, LoginError>;
}

impl std::fmt::Debug for dyn SessionProvider {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "MediaProvider {{ name: {} }}", self.name())
    }
}

inventory::collect!(&'static dyn SessionProvider);

pub fn providers() -> HashMap<&'static str, Box<&'static &'static dyn SessionProvider>> {
    let mut map: HashMap<&'static str, Box<&&dyn SessionProvider>>  = HashMap::new();
    for p in inventory::iter::<&dyn SessionProvider> {
        let name = p.name();
        map.insert(name, Box::new(p));
    }
    map
}