// b"Local\\acpmf_physics\0"
// b"Local\\acpmf_static\0"

#[cfg(all(windows, not(feature = "debugger")))]
use std::ffi::c_void;

use super::TelemetryError;

#[cfg(windows)]
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HANDLE,
        System::Memory::{MapViewOfFile, OpenFileMappingA, FILE_MAP_READ},
    },
};

mod conversion;
mod data;
mod graphics;
mod laphistory;
mod physics;
mod r#static;

pub use data::*;
pub use graphics::*;
pub use laphistory::*;
pub use physics::*;
pub use r#static::*;

pub trait SharedMemoryPage {
    const NAME: &'static [u8; 21];

    #[cfg(all(windows, not(feature = "debugger")))]
    fn get_reference() -> Result<&'static Self, TelemetryError>
    where
        Self: Sized + std::fmt::Debug,
    {
        let handle: HANDLE = unsafe {
            OpenFileMappingA(FILE_MAP_READ.0, false, PCSTR::from_raw(Self::NAME.as_ptr()))
        }
        .map_err(|e| TelemetryError::ConnectionFailed(e))?;

        let file_view: *const c_void =
            unsafe { MapViewOfFile(handle, FILE_MAP_READ, 0, 0, 0) }.Value;
        // trace!("map view of file: {:?}", file_view);

        let data: &Self = unsafe { &(*(file_view as *const Self)) };
        // trace!("data: {:?}", data);

        Ok(data)
    }

    #[cfg(any(not(windows), feature = "debugger"))]
    fn get_reference() -> Result<&'static Self, TelemetryError>
    where
        Self: Sized + std::fmt::Debug,
    {
        Ok(Self::debug_data())
    }

    fn debug_data() -> &'static Self
    where
        Self: Sized + std::fmt::Debug;
}
