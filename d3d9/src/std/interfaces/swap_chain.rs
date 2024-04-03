use std::ptr::NonNull;

use winapi::shared::d3d9::IDirect3DSwapChain9;

use crate::com::Com;

#[derive(Clone)]
pub struct SwapChain {
    inner: Com<IDirect3DSwapChain9>,
}

impl SwapChain {
    pub fn with_ptr(inner: NonNull<IDirect3DSwapChain9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DSwapChain9 {
        self.inner.as_ptr()
    }
}
