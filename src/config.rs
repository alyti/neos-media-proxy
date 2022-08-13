use std::{collections::HashMap, net::SocketAddr};

use secrecy::Secret;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub profiles: HashMap<String, Profile>,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub addr: Option<SocketAddr>
}


#[derive(Deserialize, Debug)]
pub struct Profile {
    pub provider: String,
    pub hostname: String,
    pub username: String,
    pub password: Secret<String>,
}
