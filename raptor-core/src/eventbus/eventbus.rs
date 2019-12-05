use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io;

pub struct EventBus {}

impl EventBus {
    pub fn send<Msg>(&self, addr: String, msg: &Msg) -> Result<(), io::Error>
    where
        Msg: Serialize,
    {
        Ok(())
    }

    pub fn request<Msg, Response>(&self, addr: String, msg: &Msg) -> Result<Response, io::Error>
    where
        Msg: Serialize,
        Response: DeserializeOwned,
    {
        Err(io::Error::last_os_error())
    }
}
