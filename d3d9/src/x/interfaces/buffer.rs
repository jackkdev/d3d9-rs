use std::ptr::NonNull;

use d3dx9_sys::d3dx9core::ID3DXBuffer;
use winapi::um::winnt::VOID;

use crate::com::Com;

#[derive(Clone)]
pub struct Buffer {
    inner: Com<ID3DXBuffer>,
}

impl Buffer {
    pub fn with_ptr(inner: NonNull<ID3DXBuffer>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
        }
    }

    pub fn as_ptr(&self) -> *mut ID3DXBuffer {
        self.inner.as_ptr()
    }

    pub fn get_buffer_ptr(&self) -> *const VOID {
        unsafe { self.inner.GetBufferPointer() }
    }

    pub fn get_buffer_size(&self) -> u32 {
        unsafe { self.inner.GetBufferSize() }
    }
}
