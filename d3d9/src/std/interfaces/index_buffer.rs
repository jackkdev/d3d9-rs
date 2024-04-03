use std::{ptr, ptr::NonNull};

use winapi::{shared::d3d9::IDirect3DIndexBuffer9, um::winnt::VOID};

use crate::{check_hresult, check_hresult_mut, com::Com, error::WindowsResult};

#[derive(Clone)]
pub struct IndexBuffer {
    inner: Com<IDirect3DIndexBuffer9>,
}

impl IndexBuffer {
    pub fn with_ptr(inner: NonNull<IDirect3DIndexBuffer9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DIndexBuffer9 {
        self.inner.as_ptr()
    }

    pub fn lock<T>(&self, offset: u32, size: u32) -> WindowsResult<&mut [T]> {
        unsafe {
            let mut c_data: *mut VOID = ptr::null_mut();

            check_hresult_mut!(self.inner.Lock(offset, size, &mut c_data as *mut _, 0))?;

            Ok(std::slice::from_raw_parts_mut(
                c_data as *mut T,
                (size as usize) / std::mem::size_of::<T>(),
            ))
        }
    }

    pub fn unlock(&self) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.Unlock())?;
        }

        Ok(())
    }
}
