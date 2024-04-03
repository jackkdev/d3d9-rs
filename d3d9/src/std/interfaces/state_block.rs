use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DStateBlock9;

use crate::com::Com;

#[derive(Clone)]
pub struct StateBlock {
    inner: Com<IDirect3DStateBlock9>,
}

impl StateBlock {
    pub fn with_ptr(inner: NonNull<IDirect3DStateBlock9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DStateBlock9 {
        self.inner.as_ptr()
    }
}
