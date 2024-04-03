use std::ptr::NonNull;

use winapi::shared::d3d9::IDirect3DTexture9;

use crate::com::Com;

#[derive(Clone)]
pub struct Texture {
    inner: Com<IDirect3DTexture9>,
}

impl Texture {
    pub fn with_ptr(inner: NonNull<IDirect3DTexture9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DTexture9 {
        self.inner.as_ptr()
    }
}
