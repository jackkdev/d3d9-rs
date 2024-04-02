use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DVertexBuffer9;

use crate::com::Com;

#[derive(Clone)]
pub struct VertexBuffer<'device> {
    inner: Com<IDirect3DVertexBuffer9>,
    _lifetime: PhantomData<&'device ()>,
}

impl<'device> VertexBuffer<'device> {
    pub fn with_ptr(inner: NonNull<IDirect3DVertexBuffer9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            _lifetime: PhantomData::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DVertexBuffer9 {
        self.inner.as_ptr()
    }
}
