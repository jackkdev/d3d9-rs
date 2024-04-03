use std::ptr::NonNull;

use winapi::shared::d3d9::IDirect3DCubeTexture9;

use crate::com::Com;

#[derive(Clone)]
pub struct CubeTexture {
    inner: Com<IDirect3DCubeTexture9>,
}

impl CubeTexture {
    pub fn with_ptr(inner: NonNull<IDirect3DCubeTexture9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DCubeTexture9 {
        self.inner.as_ptr()
    }
}
