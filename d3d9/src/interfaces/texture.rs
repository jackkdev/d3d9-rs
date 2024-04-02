use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DTexture9;

use crate::com::Com;

#[derive(Clone)]
pub struct Texture<'device> {
    inner: Com<IDirect3DTexture9>,
    _lifetime: PhantomData<&'device ()>,
}

impl<'device> Texture<'device> {
    pub fn with_ptr(inner: NonNull<IDirect3DTexture9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            _lifetime: PhantomData::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DTexture9 {
        self.inner.as_ptr()
    }
}
