use std::{marker::PhantomData, ptr::NonNull};

use winapi::shared::d3d9::IDirect3DSwapChain9;

use crate::com::Com;

#[derive(Clone)]
pub struct SwapChain<'device> {
    inner: Com<IDirect3DSwapChain9>,
    _lifetime: PhantomData<&'device ()>,
}

impl<'device> SwapChain<'device> {
    pub fn with_ptr(inner: NonNull<IDirect3DSwapChain9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            _lifetime: PhantomData::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DSwapChain9 {
        self.inner.as_ptr()
    }
}
