use crate::errors::StmError;
use defmt::Format;
use miniserde::__private::String;
use miniserde::{json, Deserialize, Serialize};

#[derive(Default, Format, Serialize, Deserialize, Debug)] // references only
pub struct Payload {
    pub meter1: f32,
    pub meter2: f32,
    pub meter3: f32,
}

impl Payload {
    pub fn update_from_json(&mut self, slice: &[u8]) -> Result<(), StmError> {
        *self = json::from_str::<Self>(
            core::str::from_utf8(slice).map_err(|_e| StmError::InvalidConfigData)?,
        )
        .map_err(|_e| StmError::InvalidConfigData)?;
        Ok(())
    }
    pub fn dump_to_json(&self) -> String {
        json::to_string(&self)
    }
}
