use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DIndexBuffer9;

use crate::com::Com;

#[derive(Clone)]
pub struct IndexBuffer<'device> {
    inner: Com<IDirect3DIndexBuffer9>,
    _lifetime: PhantomData<&'device ()>,
}

impl<'device> IndexBuffer<'device> {
    pub fn with_ptr(inner: NonNull<IDirect3DIndexBuffer9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            _lifetime: PhantomData::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DIndexBuffer9 {
        self.inner.as_ptr()
    }
}
