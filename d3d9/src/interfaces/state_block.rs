use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DStateBlock9;

use crate::com::Com;

#[derive(Clone)]
pub struct StateBlock<'device> {
    inner: Com<IDirect3DStateBlock9>,
    _lifetime: PhantomData<&'device ()>,
}

impl<'device> StateBlock<'device> {
    pub fn with_ptr(inner: NonNull<IDirect3DStateBlock9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            _lifetime: PhantomData::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DStateBlock9 {
        self.inner.as_ptr()
    }
}
