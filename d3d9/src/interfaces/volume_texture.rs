use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DVolumeTexture9;

use crate::com::Com;

#[derive(Clone)]
pub struct VolumeTexture<'device> {
    inner: Com<IDirect3DVolumeTexture9>,
    _lifetime: PhantomData<&'device ()>,
}

impl<'device> VolumeTexture<'device> {
    pub fn with_ptr(inner: NonNull<IDirect3DVolumeTexture9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            _lifetime: PhantomData::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DVolumeTexture9 {
        self.inner.as_ptr()
    }
}
