use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DSurface9;

use crate::com::Com;

#[derive(Clone)]
pub struct Surface {
    inner: Com<IDirect3DSurface9>,
}

impl Surface {
    pub fn with_ptr(inner: NonNull<IDirect3DSurface9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DSurface9 {
        self.inner.as_ptr()
    }
}
