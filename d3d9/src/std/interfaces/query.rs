use std::ptr::NonNull;

use winapi::shared::d3d9::IDirect3DQuery9;

use crate::com::Com;

#[derive(Clone)]
pub struct Query {
    inner: Com<IDirect3DQuery9>,
}

impl Query {
    pub fn with_ptr(inner: NonNull<IDirect3DQuery9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DQuery9 {
        self.inner.as_ptr()
    }
}
