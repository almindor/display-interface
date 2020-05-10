#![no_std]

//! A generic display interface
//!
//! This crate contains an error type and traits to implement for bus interface drivers drivers to
//! be consumed by display drivers. It abstracts over the different communication methods available
//! to drive a display and allows a driver writer to focus on driving the display itself and only
//! have to implement a single interface.

pub mod prelude;

/// A ubiquitous error type for all kinds of problems which could happen when communicating with a
/// display
#[derive(Clone, Debug)]
pub enum DisplayError {
    /// Invalid transfer format selected (e.g. u16 on u8 interface)
    InvalidFormatError,
    /// Unable to write to bus
    BusWriteError,
    /// Unable to assert or de-assert data/command switching signal
    DCError,
    /// Unable to assert chip select signal
    CSError,
}

/// This trait implements a write-only interface for a display which has separate data and command
/// modes. It is the responsibility of implementations to activate the correct mode in their
/// implementation when corresponding method is called.
pub trait WriteOnlyDataCommand
{
    /// Width of the data type used for commands and data transfers
    type Width: Copy;

    /// Send a batch of commands to display
    fn send_commands<S>(&mut self, cmd: S) -> Result<(), DisplayError>
    where
        S: IntoIterator<Item = Self::Width>;

    /// Send pixel data to display
    fn send_data<S>(&mut self, buf: S) -> Result<(), DisplayError>
    where
        S: IntoIterator<Item = Self::Width>;
}
