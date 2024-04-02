use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DVertexShader9;

use crate::com::Com;

#[derive(Clone)]
pub struct VertexShader<'device> {
    inner: Com<IDirect3DVertexShader9>,
    _lifetime: PhantomData<&'device ()>,
}

impl<'device> VertexShader<'device> {
    pub fn with_ptr(inner: NonNull<IDirect3DVertexShader9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            _lifetime: PhantomData::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DVertexShader9 {
        self.inner.as_ptr()
    }
}
