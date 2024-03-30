use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DSurface9;

use crate::com::Com;

#[derive(Clone)]
pub struct Surface<'device> {
    inner: Com<IDirect3DSurface9>,
    _lifetime: PhantomData<&'device ()>,
}

impl<'device> Surface<'device> {
    pub fn with_ptr(inner: NonNull<IDirect3DSurface9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            _lifetime: PhantomData::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DSurface9 {
        self.inner.as_ptr()
    }
}
