//! Module describes errors of the smart house library

use thiserror::Error;

/// Describes errors of the library
#[derive(Error, Debug)]
pub enum Error {
    /// Describes error in case of device not found in the provider
    /// by device name and room name
    #[error(r#"Not found device "{}" in room "{}""#, device_name, room_name)]
    DeviceNotFoundError {
        device_name: String,
        room_name: String,
    },
}

/// Describes alias for the library Result type
pub type Result<T> = std::result::Result<T, self::Error>;
