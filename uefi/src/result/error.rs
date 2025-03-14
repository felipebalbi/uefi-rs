//! Module for UEFI-specific error encodings. See [`Error`].

use super::Status;
use core::fmt::{Debug, Display};

/// An UEFI-related error with optionally additional payload data. The error
/// kind is encoded in the `status` field (see [`Status`]). Additional payload
/// may be inside the `data` field.
#[derive(Debug, PartialEq, Eq)]
pub struct Error<Data: Debug = ()> {
    status: Status,
    data: Data,
}

impl<Data: Debug> Error<Data> {
    /// Create an `Error`.
    ///
    /// # Panics
    ///
    /// Panics if `status` is [`Status::SUCCESS`].
    pub const fn new(status: Status, data: Data) -> Self {
        assert!(!matches!(status, Status::SUCCESS));
        Self { status, data }
    }

    /// Get error `Status`.
    pub const fn status(&self) -> Status {
        self.status
    }

    /// Get error data.
    pub const fn data(&self) -> &Data {
        &self.data
    }

    /// Split this error into its inner status and error data
    #[allow(clippy::missing_const_for_fn)]
    pub fn split(self) -> (Status, Data) {
        (self.status, self.data)
    }
}

// Errors without error data can be autogenerated from statuses

impl From<Status> for Error<()> {
    fn from(status: Status) -> Self {
        Error::new(status, ())
    }
}

impl<Data: Debug> Display for Error<Data> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UEFI Error {}: {:?}", self.status(), self.data())
    }
}

#[cfg(feature = "unstable")]
impl<Data: Debug> core::error::Error for Error<Data> {}
