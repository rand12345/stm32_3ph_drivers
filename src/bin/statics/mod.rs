use crate::{config::Payload, types::*};
use embassy_sync::signal::Signal;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref READINGS: ConfigType = embassy_sync::mutex::Mutex::new(Payload::default());
    pub static ref FIRSTMQTT: Status = Signal::new();
}

// pub const LAST_READING_TIMEOUT_SECS: u64 = 10;
