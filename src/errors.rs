use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TelemetryError;

impl fmt::Display for TelemetryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Recording telemetry has failed")
    }
}

impl Error for TelemetryError {}
