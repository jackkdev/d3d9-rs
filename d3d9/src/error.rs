//! Provides error handling for Windows APIs.
use std::{
    error::Error,
    fmt,
    fmt::{Debug, Display, Formatter},
    mem::transmute,
    ptr,
};

use winapi::um::{
    winbase::{FormatMessageA, FORMAT_MESSAGE_FROM_SYSTEM},
    winnt::HRESULT,
};

/// A result type wrapping [`WindowsError`].
pub type WindowsResult<T> = Result<T, WindowsError>;

/// Represents an error returned by the Windows API.
#[derive(Debug)]
pub struct WindowsError {
    code: u32,
    message: String,
}

impl WindowsError {
    /// Creates a new instance from the given [`HRESULT`] code.
    pub fn from_hresult(hresult: HRESULT) -> Self {
        let message = unsafe {
            const N_BUFFER: usize = 512;
            let mut buffer = [0i8; N_BUFFER];

            FormatMessageA(
                FORMAT_MESSAGE_FROM_SYSTEM,
                ptr::null(),
                hresult as u32,
                0,
                &mut buffer as *mut _,
                N_BUFFER as u32,
                ptr::null_mut(),
            );

            let buffer: [u8; N_BUFFER] = transmute(buffer);

            String::from_utf8(buffer.to_vec())
                .expect("failed to convert windows error message into a utf-8 string")
        };

        Self {
            code: hresult as u32,
            message,
        }
    }
}

impl Error for WindowsError {}

impl Display for WindowsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "WindowsError({:X}): {}", self.code, self.message)
    }
}

#[macro_export]
macro_rules! check_hresult {
    ($hresult:expr) => {{
        let result = $hresult;
        if result != 0 {
            return Err($crate::error::WindowsError::from_hresult(result));
        }
    }};
}
