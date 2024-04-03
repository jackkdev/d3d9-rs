//! A type-safe wrapper around the Direct3D9 library.
//!
//! # Usage
//!
//! ```rs
//! use d3d9::{
//!     error::WindowsResult,
//!     std::interfaces::Context,
//! };
//!
//! fn main() -> WindowsResult<()> {
//!     // A context is synonymous to the `IDirect3D9` interface.
//!     let context = Context::new()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # SDK
//!
//! Unfortunately, for this library to compile properly, the Direct3D SDK (2010) must be installed.
//! It can be found [here](https://www.microsoft.com/en-us/download/details.aspx?id=6812). The
//! location in which the SDK is installed is typically constant, although it can be overridden
//! via the `DIRECTX_SDK` environment variable when compiling.
//!
//! # Safety
//!
//! For the most part, no *panics* should be thrown. If the `HRESULT` returned by an API function
//! is not `0` (success), then any further processing will be terminated and an [`Err`] variant
//! will be returned. This *should* prevent any null pointers.

pub mod com;
pub mod error;
pub mod std;
pub mod x;
