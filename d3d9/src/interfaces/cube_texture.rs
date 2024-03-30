use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DCubeTexture9;

use crate::com::Com;

#[derive(Clone)]
pub struct CubeTexture<'device> {
    inner: Com<IDirect3DCubeTexture9>,
    _lifetime: PhantomData<&'device ()>,
}

impl<'device> CubeTexture<'device> {
    pub fn with_ptr(inner: NonNull<IDirect3DCubeTexture9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            _lifetime: PhantomData::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DCubeTexture9 {
        self.inner.as_ptr()
    }
}
