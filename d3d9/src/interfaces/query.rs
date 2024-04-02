use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DQuery9;

use crate::com::Com;

#[derive(Clone)]
pub struct Query<'device> {
    inner: Com<IDirect3DQuery9>,
    _lifetime: PhantomData<&'device ()>,
}

impl<'device> Query<'device> {
    pub fn with_ptr(inner: NonNull<IDirect3DQuery9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            _lifetime: PhantomData::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DQuery9 {
        self.inner.as_ptr()
    }
}
