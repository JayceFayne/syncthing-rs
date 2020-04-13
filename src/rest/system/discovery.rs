use crate::rest::DeviceID;
use serde::Deserialize;
use std::collections::HashMap;

//FIXME: verify that this is the device id
pub type Addr = String;

#[derive(Debug, Deserialize)]
pub struct Addresses {
    pub addresses: Vec<Addr>,
}

pub type Discovery = HashMap<DeviceID, Addresses>;
