use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DPixelShader9;

use crate::com::Com;

#[derive(Clone)]
pub struct PixelShader<'device> {
    inner: Com<IDirect3DPixelShader9>,
    _lifetime: PhantomData<&'device ()>,
}

impl<'device> PixelShader<'device> {
    pub fn with_ptr(inner: NonNull<IDirect3DPixelShader9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            _lifetime: PhantomData::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DPixelShader9 {
        self.inner.as_ptr()
    }
}
