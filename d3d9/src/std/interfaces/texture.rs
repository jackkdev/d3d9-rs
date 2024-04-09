use std::{
    mem::{size_of, MaybeUninit},
    ptr,
    ptr::NonNull,
};

use winapi::shared::{d3d9::IDirect3DTexture9, d3d9types::D3DLOCKED_RECT};

use crate::{
    check_hresult, check_hresult_mut, com::Com, error::WindowsResult, std::types::LockFlags,
};

#[derive(Clone)]
pub struct Texture {
    inner: Com<IDirect3DTexture9>,
    size: (u32, u32),
}

impl Texture {
    pub fn with_ptr(inner: NonNull<IDirect3DTexture9>, size: (u32, u32)) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            size,
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DTexture9 {
        self.inner.as_ptr()
    }

    pub fn lock_rect<T>(&self, level: u32, flags: LockFlags) -> WindowsResult<(&mut [T])> {
        unsafe {
            let mut locked_rect: D3DLOCKED_RECT = MaybeUninit::zeroed().assume_init();

            check_hresult_mut!(self
                .inner
                .LockRect(level, &mut locked_rect, ptr::null(), flags.0))?;

            Ok(std::slice::from_raw_parts_mut(
                locked_rect.pBits as *mut T,
                (self.size.0 * self.size.1 * size_of::<T>() as u32) as usize,
            ))
        }
    }

    pub fn unlock_rect(&self, level: u32) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.UnlockRect(level))?;
        }

        Ok(())
    }
}
