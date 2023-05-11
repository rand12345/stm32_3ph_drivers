use crate::config::Payload;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex as _Mutex;
use embassy_sync::signal::Signal;

// pub type MutexBool = Mutex<_Mutex, bool>;
pub type ConfigType = embassy_sync::mutex::Mutex<_Mutex, Payload>;
pub type Status = Signal<_Mutex, bool>;
